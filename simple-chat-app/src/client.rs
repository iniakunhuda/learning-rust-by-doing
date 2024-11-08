use std::io::{self, ErrorKind, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let mut client = TcpStream::connect("127.0.0.1:8888").expect("Failed to connect");
    client.set_nonblocking(true).expect("Failed to set non-blocking");

    let (tx, rx) = mpsc::channel::<String>();

    thread::spawn(move || {
        loop {
            let mut buff = vec![0; 1024];
            match client.read(&mut buff) {
                Ok(n) if n == 0 => {
                    println!("Server disconnected");
                    break;
                }
                Ok(n) => {
                    let msg = String::from_utf8_lossy(&buff[..n]);
                    print!("{}", msg);
                }
                Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("Connection error");
                    break;
                }
            }

            match rx.try_recv() {
                Ok(msg) => {
                    let mut buff = msg.clone().into_bytes();
                    buff.push(b'\n');
                    if let Err(e) = client.write_all(&buff) {
                        println!("Failed to send message: {}", e);
                        break;
                    }
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }

            thread::sleep(Duration::from_millis(100));
        }
    });

    println!("Write messages and press Enter to send:");
    loop {
        let mut buff = String::new();
        io::stdin()
            .read_line(&mut buff)
            .expect("Failed to read from stdin");
        let msg = buff.trim().to_string();
        if msg == ":quit" || tx.send(msg).is_err() {
            break;
        }
    }
    println!("Chat client terminated.");
}