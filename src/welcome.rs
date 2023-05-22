use std::io::{self, Write};
// use std::io::{Error, ErrorKind};
use std::env;
use reqwest::{Client, Error};
use colored::*;


fn print_error(address: String, error: Option<Error>) {
    println!("{}", format!("Failed to connect to AdGuard at {}", address).red());
    match error {
        Some(err) => { 
            println!("{}", format!("{}", err).red().dimmed());
        },
        None => { println!("Didn't get anything"); }
    }
    println!("{}", "\nPlease check your environmental variables and try again.".yellow());
    println!("{}", "Exiting...".blue());
}

pub async fn welcome() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    for &key in &["ADGUARD_IP", "ADGUARD_PORT", "ADGUARD_USERNAME", "ADGUARD_PASSWORD"] {
        if env::var(key).is_err() {
            println!("{}", format!("\nThe {} environmental variable is not set", key.bold()).yellow());

            print!("{}", format!("Enter a value for {}: ", key).cyan().bold());
            io::stdout().flush()?;

            let mut value = String::new();
            io::stdin().read_line(&mut value)?;
            env::set_var(key, value.trim());
        }
    }

    let ip = env::var("ADGUARD_IP").unwrap();
    let port = env::var("ADGUARD_PORT").unwrap();
    let username = env::var("ADGUARD_USERNAME").unwrap();
    let password = env::var("ADGUARD_PASSWORD").unwrap();


    println!("{}", "\nVerifying connection to your AdGuard instance...".blue());

    let auth_string = format!("{}:{}", username, password);
    let auth_header_value = format!("Basic {}", base64::encode(&auth_string));
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("Authorization", auth_header_value.parse()?);

    let url = format!("http://{}:{}/control/status", ip, port);

    let res = match client.get(&url).headers(headers).send().await {
        Ok(response) => response,
        Err(e) => {
            print_error(format!("{}:{}", ip, port), Some(e));
            std::process::exit(1);
        },
    };

    if res.status().is_success() {
        println!("{}", "AdGuard connection successful!".green());
    } else {
        print_error(format!("{}:{}", ip, port), None);
        std::process::exit(1);
    }



    Ok(())
}
