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

fn print_ascii_art() {
    let art = r"
 █████╗ ██████╗  ██████╗ ██╗   ██╗ █████╗ ██████╗ ██████╗ ██╗ █████╗ ███╗   ██╗
██╔══██╗██╔══██╗██╔════╝ ██║   ██║██╔══██╗██╔══██╗██╔══██╗██║██╔══██╗████╗  ██║
███████║██║  ██║██║  ███╗██║   ██║███████║██████╔╝██║  ██║██║███████║██╔██╗ ██║
██╔══██║██║  ██║██║   ██║██║   ██║██╔══██║██╔══██╗██║  ██║██║██╔══██║██║╚██╗██║
██║  ██║██████╔╝╚██████╔╝╚██████╔╝██║  ██║██║  ██║██████╔╝██║██║  ██║██║ ╚████║
╚═╝  ╚═╝╚═════╝  ╚═════╝  ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝╚═════╝ ╚═╝╚═╝  ╚═╝╚═╝  ╚═══╝
";
    println!("{}", art.green());

    println!("{}", "\nWelcome to AdGuardian Terminal Edition!".green());
    println!("{}", "Terminal-based, real-time traffic monitoring and statistics for your AdGuard Home instance".green().italic().dimmed());
    println!("{}", "For documentation and support, please visit: https://github.com/lissy93/adguardian-term\n".green().italic().dimmed());
}


pub async fn welcome() -> Result<(), Box<dyn std::error::Error>> {
    print_ascii_art();

    println!("{}", "Starting initialization checks...".blue());

    let client = Client::new();

    for &key in &["ADGUARD_IP", "ADGUARD_PORT", "ADGUARD_USERNAME", "ADGUARD_PASSWORD"] {
        if env::var(key).is_err() {
            println!("{}", format!("The {} environmental variable is not yet set", key.bold()).yellow());
    
            print!("{}", format!("› Enter a value for {}: ", key).blue().bold());
            io::stdout().flush()?;
    
            let mut value = String::new();
            io::stdin().read_line(&mut value)?;
            env::set_var(key, value.trim());
        }
    
        if key.contains("PASSWORD") {
            println!("{}", format!("{} is set to ******", key.bold()).green());
        } else {
            println!("{}", format!("{} is set to {}", key.bold(), env::var(key).unwrap()).green());
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
        println!("{}", "AdGuard connection successful!\n".green());
    } else {
        print_error(format!("{}:{}", ip, port), None);
        std::process::exit(1);
    }



    Ok(())
}
