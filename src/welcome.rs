use std::io::{self, Write};
use std::env;
use reqwest::{Client, Error};
use colored::*;

fn print_info(text: &str, is_secondary: bool) {
    let _ = if is_secondary {
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
            None => ColoredString::from("".red().dimmed()),
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
    username: String,
    password: String,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "\nVerifying connection to your AdGuard instance...".blue());

    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", auth_header_value.parse()?);

    let url = format!("http://{}:{}/control/status", ip, port);

    match client.get(&url).headers(headers).send().await {
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
    let username = get_env("ADGUARD_USERNAME")?;
    let password = get_env("ADGUARD_PASSWORD")?;

    verify_connection(&client, ip, port, username, password).await
}
