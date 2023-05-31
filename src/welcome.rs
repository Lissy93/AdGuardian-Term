use std::{
    io:: {self, Write},
    env,
    time::Duration
};
use reqwest::{Client, Error};
use colored::*;

fn print_info(text: &str, is_secondary: bool) {
    if is_secondary {
        println!("{}", text.green().italic().dimmed());
    } else {
        println!("{}", text.green());
    };
}


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

fn print_error(address: &str, error: Option<&Error>) {
    eprintln!(
        "{}{}",
        format!("Failed to connect to AdGuard at {}", address).red(),
        match error {
            Some(err) => format!("\n{}", err).red().dimmed(),
            None => "".red().dimmed(),
        },
    );
    eprintln!("{}\n{}", "\nPlease check your environmental variables and try again.".yellow(), "Exiting...".blue());
}


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
            println!("{}", "AdGuard connection successful!\n".green());
            Ok(())
        }
        Ok(_) | Err(_) => {
            print_error(&format!("{}:{}", ip, port), None);
            std::process::exit(1);
        }
    }
}

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

    let ip = get_env("ADGUARD_IP")?;
    let port = get_env("ADGUARD_PORT")?;
    let protocol = get_env("ADGUARD_PROTOCOL")?;
    let username = get_env("ADGUARD_USERNAME")?;
    let password = get_env("ADGUARD_PASSWORD")?;
    
    verify_connection(&client, ip, port, protocol, username, password).await
}
