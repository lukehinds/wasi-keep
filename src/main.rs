use anyhow::Result;
use clap::{Command, Arg};

mod utils;
mod keys;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let matches = Command::new("wasi-keys")
        .version("0.1")
        .author("Luke Hinds")
        .about("wasm key keep")
        .arg(
            Arg::new("generate-keys")
                .short('k')
                .long("generate-keys")
                .takes_value(false)
                .help("Generate key pair"),
        )
        .get_matches();

    // Generate keys for user. If successful, exit(0)
    if matches.is_present("generate-keys") {
        let pword = utils::password_prompt();
        match pword {
            Ok(pword) => {
                let keygen = keys::generate_keys(pword);
                match keygen {
                    Ok(_) => {
                        println!("\nKeys generated successfully");
                    }
                    Err(e) => {
                        println!("\nError generating keys: {}", e);
                    }
                }
            }
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
    anyhow::Ok(())
}