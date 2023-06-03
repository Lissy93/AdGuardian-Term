use std::{
    io:: {self, Write},
    env,
    time::Duration
};
use reqwest::{Client, Error};
use colored::*;

use serde_json::Value;
use semver::{Version};

/// Reusable function that just prints success messages to the console
fn print_info(text: &str, is_secondary: bool) {
    if is_secondary {
        println!("{}", text.green().italic().dimmed());
    } else {
        println!("{}", text.green());
    };
}

/// Prints the AdGuardian ASCII art to console
fn print_ascii_art() {
    let art = r"
 █████╗ ██████╗  ██████╗ ██╗   ██╗ █████╗ ██████╗ ██████╗ ██╗ █████╗ ███╗   ██╗
██╔══██╗██╔══██╗██╔════╝ ██║   ██║██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗████╗  ██║
███████║██║  ██║██║  ███╗██║   ██║███████║██████╔╝██║  ██║██║███████║██╔██╗ ██║
██╔══██║██║  ██║██║   ██║██║   ██║██╔══██║██╔══██╗██║  ██║██║██╔══██║██║╚██╗██║
██║  ██║██████╔╝╚██████╔╝╚██████╔╝██║  ██║██║  ██║██████╔╝██║██║  ██║██║ ╚████║
╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝
";
    print_info(art, false);
    print_info("\nWelcome to AdGuardian Terminal Edition!", false);
    print_info("Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance", true);
    print_info("For documentation and support, please visit: https://github.com/lissy93/adguardian-term\n", true);
}

/// Print error message, along with (optional) stack trace, then exit
fn print_error(message: &str, sub_message: &str, error: Option<&Error>) {
    eprintln!(
        "{}{}{}",
        format!("{}", message).red(),
        match error {
            Some(err) => format!("\n{}", err).red().dimmed(),
            None => "".red().dimmed(),
        },
        format!("\n{}", sub_message).yellow(),
    );

    std::process::exit(1);
}

/// Given a key, get the value from the environmental variables, and print it to the console
fn get_env(key: &str) -> Result<String, env::VarError> {
    env::var(key).map(|v| {
        println!(
            "{}",
            format!(
                "{} is set to {}",
                key.bold(),
                if key.contains("PASSWORD") { "******" } else { &v }
            )
            .green()
        );
        v
    })
}

/// Given a possibly undefined version number, check if it's present and supported
fn check_version(version: Option<&str>) {
    let min_version = Version::parse("0.107.29").unwrap();
    
    match version {
        Some(version_str) => {
            let adguard_version = Version::parse(&version_str[1..]).unwrap();
            
            if adguard_version < min_version {
                print_error(
                    "AdGuard Home version is too old, and is now unsupported",
                    format!("You're running AdGuard {}. Please upgrade to v{} or later.", version_str, min_version.to_string()).as_str(),
                    None,
                );
            }
        },
        None => {
            print_error(
                "Unsupported AdGuard Home version",
                format!(
                    concat!(
                        "Failed to get the version number of your AdGuard Home instance.\n",
                        "This usually means you're running an old, and unsupported version.\n",
                        "Please upgrade to v{} or later."
                    ), min_version.to_string()
                ).as_str(),
                None,
            );
        }
    }
}

/// With the users specified AdGuard details, verify the connection (exit on fail)
async fn verify_connection(
    client: &Client,
    ip: String,
    port: String,
    protocol: String,
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\nVerifying connection to your AdGuard instance...".blue());

    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", auth_header_value.parse()?);

    let url = format!("{}://{}:{}/control/status", protocol, ip, port);

    match client
        .get(&url)
        .headers(headers)
        .timeout(Duration::from_secs(2))
        .send()
        .await {
        Ok(res) if res.status().is_success() => {
            // Get version string (if present), and check if valid - exit if not
            let body: Value = res.json().await?;
            check_version(body["version"].as_str());
            // All good! Print success message :)
            let safe_version = body["version"].as_str().unwrap_or("mystery version");
            println!("{}", format!("AdGuard ({}) connection successful!\n", safe_version).green());
            Ok(())
        }
        // Connection failed to authenticate. Print error and exit
        Ok(_) => {
            print_error(
                &format!("Authentication with AdGuard at {}:{} failed", ip, port),
                "Please check your environmental variables and try again.",
                None,
            );
            Ok(())
        },
        // Connection failed to establish. Print error and exit
        Err(e) => {
            print_error(
                &format!("Failed to connect to AdGuard at: {}:{}", ip, port),
                "Please check your environmental variables and try again.",
                Some(&e),
            );
            Ok(())
        }
    }
}

/// Initiate the welcome script
/// This function will:
/// - Print the AdGuardian ASCII art
/// - Check for the required environmental variables
/// - Prompt the user to enter any missing variables
/// - Verify the connection to the AdGuard instance
/// - Verify authentication is successful
/// - Verify the AdGuard Home version is supported
/// - Then either print a success message, or show instructions to fix and exit
pub async fn welcome() -> Result<(), Box<dyn std::error::Error>> {
    print_ascii_art();
    println!("{}", "Starting initialization checks...".blue());

    let client = Client::new();

    // List of available flags, ant their associated env vars
    let flags = [
        ("--adguard-ip", "ADGUARD_IP"),
        ("--adguard-port", "ADGUARD_PORT"),
        ("--adguard-username", "ADGUARD_USERNAME"),
        ("--adguard-password", "ADGUARD_PASSWORD"),
    ];

    let protocol: String = env::var("ADGUARD_PROTOCOL").unwrap_or_else(|_| "http".into()).parse()?;
    env::set_var("ADGUARD_PROTOCOL", protocol);

    // Parse command line arguments
    let mut args = std::env::args().peekable();
    while let Some(arg) = args.next() {
        for &(flag, var) in &flags {
            if arg == flag {
                if let Some(value) = args.peek() {
                    env::set_var(var, value);
                    args.next();
                }
            }
        }
    }

    // If any of the env variables or flags are not yet set, prompt the user to enter them
    for &key in &["ADGUARD_IP", "ADGUARD_PORT", "ADGUARD_USERNAME", "ADGUARD_PASSWORD"] {
        if env::var(key).is_err() {
            println!(
                "{}",
                format!("The {} environmental variable is not yet set", key.bold()).yellow()
            );
            print!("{}", format!("› Enter a value for {}: ", key).blue().bold());
            io::stdout().flush()?;

            let mut value = String::new();
            io::stdin().read_line(&mut value)?;
            env::set_var(key, value.trim());
        }
    }

    // Grab the values of the (now set) environmental variables
    let ip = get_env("ADGUARD_IP")?;
    let port = get_env("ADGUARD_PORT")?;
    let protocol = get_env("ADGUARD_PROTOCOL")?;
    let username = get_env("ADGUARD_USERNAME")?;
    let password = get_env("ADGUARD_PASSWORD")?;
    
    // Verify that we can connect, authenticate, and that version is supported (exit on failure)
    verify_connection(&client, ip, port, protocol, username, password).await
}
