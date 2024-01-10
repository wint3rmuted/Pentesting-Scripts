//  a basic Rust script that takes user input for the target IP address, port range, and then performs a port scan, banner grabbing, checks HTTP headers for open ports, and saves the output to a text file.

use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

const ASCII_BANNER: &str = r#"
  


                                  .         .o   oooo                                  .o8                             
                                .o8       .d88   `888                                 "888                             
oooo d8b oooo  oooo   .oooo.o .o888oo   .d'888    888 .oo.    .ooooo.   .oooo.    .oooo888   .ooooo.  oooo d8b  .oooo.o
`888""8P `888  `888  d88(  "8   888   .d'  888    888P"Y88b  d88' `88b `P  )88b  d88' `888  d88' `88b `888""8P d88(  "8
 888      888   888  `"Y88b.    888   88ooo888oo  888   888  888ooo888  .oP"888  888   888  888ooo888  888     `"Y88b. 
 888      888   888  o.  )88b   888 .      888    888   888  888    .o d8(  888  888   888  888    .o  888     o.  )88b
d888b     `V88V"V8P' 8""888P'   "888"     o888o  o888o o888o `Y8bod8P' `Y888""8o `Y8bod88P" `Y8bod8P' d888b    8""888P'

                     Port Scanner / Banner Grabber / Headers Enumeration Script by wint3rmute                                                             
                                                                                                            
                                                                                                          

"#;

fn scan_port(host: &str, port: u16) -> Option<String> {
    match TcpStream::connect_timeout(&(host, port).into(), Duration::from_secs(1)) {
        Ok(mut stream) => {
            let banner = get_banner(&mut stream);
            println!("Port {} is open. Banner: {}", port, banner);
            Some(banner)
        }
        Err(_) => None,
    }
}

fn get_banner(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap_or(0).to_string()
}

fn check_headers(host: &str, port: u16) -> String {
    if let Ok(mut stream) = TcpStream::connect_timeout(&(host, port).into(), Duration::from_secs(1)) {
        // Send a basic HTTP request to receive headers
        let request = format!("GET / HTTP/1.1\r\nHost: {}\r\n\r\n", host);
        stream.write(request.as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        println!("Headers for port {}:\n{}", port, response);
        response
    } else {
        String::new()
    }
}

fn main() {
    // Display ASCII banner
    println!("{}", ASCII_BANNER);

    // Take user input for target IP, port range
    print!("Enter target IP address: ");
    io::stdout().flush().unwrap();
    let mut target_ip = String::new();
    io::stdin().read_line(&mut target_ip).unwrap();
    let target_ip = target_ip.trim();

    print!("Enter starting port: ");
    io::stdout().flush().unwrap();
    let mut start_port = String::new();
    io::stdin().read_line(&mut start_port).unwrap();
    let start_port: u16 = start_port.trim().parse().unwrap();

    print!("Enter ending port: ");
    io::stdout().flush().unwrap();
    let mut end_port = String::new();
    io::stdin().read_line(&mut end_port).unwrap();
    let end_port: u16 = end_port.trim().parse().unwrap();

    let output_file_path = "output.txt";
    let mut output_file = File::create(output_file_path).expect("Unable to create output file");

    writeln!(output_file, "Scanning ports on {}", target_ip).expect("Error writing to file");

    for port in start_port..=end_port {
        if let Some(banner) = scan_port(target_ip, port) {
            // Process the banner information as needed
            writeln!(output_file, "Port {}: Open. Banner: {}", port, banner).expect("Error writing to file");
        }

        // Check headers for ports that appear open
        let headers = check_headers(target_ip, port);
        if !headers.is_empty() {
            writeln!(output_file, "Headers for port {}:\n{}", port, headers).expect("Error writing to file");
        }
    }

    println!("Scan completed. Results saved to {}", output_file_path);
}
