//  A Rust script that includes an ASCII banner, takes user input for the IP range, performs a ping sweep, and saves the output to a .txt file
// Add this to your Cargo.toml file under [dependencies]
// ping = "0.18.0"

use std::error::Error;
use std::fs::File;
use std::io::{self, Write};
use std::net::IpAddr;
use std::str::FromStr;

use ping::{Ping, PingResult};

fn main() {
    // Print ASCII banner
    print_banner();

    // Get user input for IP range
    let start_ip = get_user_input("Enter the starting IP address: ");
    let end_ip = get_user_input("Enter the ending IP address: ");

    // Perform ping sweep and save output to a file
    if let Ok(start_ip) = IpAddr::from_str(&start_ip) {
        if let Ok(end_ip) = IpAddr::from_str(&end_ip) {
            let output_file = "ping_sweep_results.txt";

            if let Err(err) = ping_sweep_and_save(start_ip, end_ip, output_file) {
                println!("Error: {}", err);
            } else {
                println!("Ping sweep results saved to {}", output_file);
            }
        } else {
            println!("Invalid ending IP address format");
        }
    } else {
        println!("Invalid starting IP address format");
    }
}

// Function to print ASCII banner
fn print_banner() {
    println!("
                       __          _              
   _____ __  __ _____ / /_ ____   (_)____   ____ _
  / ___// / / // ___// __// __ \ / // __ \ / __ `/
 / /   / /_/ /(__  )/ /_ / /_/ // // / / // /_/ / 
/_/    \__,_//____/ \__// .___//_//_/ /_/ \__, /  
                       /_/               /____/
            Rust Ping-Sweep by wint3rmute   
  ");
}

// Function to get user input
fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

// Function to perform ping sweep and save results to a file
fn ping_sweep_and_save(start_ip: IpAddr, end_ip: IpAddr, output_file: &str) -> Result<(), Box<dyn Error>> {
    let mut pinger = Ping::new();
    pinger.set_timeout(std::time::Duration::from_secs(2));

    // Open a file for writing
    let mut file = File::create(output_file)?;

    // Iterate through the IP range and perform ping sweep
    for ip in ip_range(start_ip, end_ip) {
        match pinger.ping(ip) {
            Ok(result) => {
                if result.is_reply {
                    writeln!(file, "Host {} is reachable", ip)?;
                }
            }
            Err(_) => {
                writeln!(file, "Error while pinging {}", ip)?;
            }
        }
    }

    Ok(())
}

// Function to iterate through IP range
fn ip_range(start: IpAddr, end: IpAddr) -> Vec<IpAddr> {
    let mut result = Vec::new();
    let mut current = start;

    while current <= end {
        result.push(current);
        current = match current {
            IpAddr::V4(ipv4) => {
                if ipv4 == std::net::Ipv4Addr::new(255, 255, 255, 255) {
                    break;
                }
                IpAddr::V4(ipv4 + 1)
            }
            IpAddr::V6(ipv6) => {
                if ipv6 == std::net::Ipv6Addr::new(
                    0xffff,
                    0xffff,
                    0xffff,
                    0xffff,
                    0xffff,
                    0xffff,
                    0xffff,
                    0xffff,
                ) {
                    break;
                }
                IpAddr::V6(ipv6 + 1)
            }
        };
    }

    result
}
