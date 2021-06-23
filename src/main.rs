use std::io::{self, Read};

fn main() {
    loop {
        let mut buffer = [0; 1024];
        let length = io::stdin().read(&mut buffer).unwrap();
        match length {
            0 => {
                eprintln!("stdin was closed");
                break;
            }
            length => {
                let snippet = String::from_utf8_lossy(&buffer[..length]);
                eprintln!("{} bytes read: {:?}", length, snippet);
            }
        }
    }
}
