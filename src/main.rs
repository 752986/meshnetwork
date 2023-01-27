use std::{
    io::{read_to_string, BufRead, BufReader, Read, Write},
    net::{Shutdown, TcpListener, TcpStream},
};

fn main() {
    let listener: TcpListener = TcpListener::bind("0.0.0.0:42069").expect("unable to bind listener");

    loop {
        match listener.accept() {
            Ok((mut client, ip)) => {
                println!("accepted incoming connection from {ip}");
                client.write_all(b"hello from declan!").unwrap();
                let mut reader = BufReader::new(&client);
                loop {
                    let mut response = String::new();
                    match reader.read_line(&mut response) {
                        Ok(0) | Err(_) => break,
                        Ok(_) => print!("{}", &response),
                    }
                }
                println!("disconnected from {ip}");
            }
            Err(ip) => {
                println!("unable to connect to client at {ip}");
            }
        }
    }
}
