use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let server = TcpListener::bind("127.0.0.1:8888").expect("Failed to bind address");
    server.set_nonblocking(true).expect("Failed to set non-blocking");

    let mut clients = vec![];
    let (tx, rx) = mpsc::channel::<String>();

    loop {
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            
            let tx = tx.clone();
            clients.push(socket.try_clone().expect("Failed to clone client"));

            thread::spawn(move || {
                let mut buffer = [0; 1024];

                loop {
                    match socket.read(&mut buffer) {
                        Ok(n) if n == 0 => return,
                        Ok(n) => {
                            let msg = String::from_utf8_lossy(&buffer[..n]);
                            let broadcast_msg = format!("Client {}: {}", addr, msg);
                            tx.send(broadcast_msg).expect("Failed to send message");
                        }
                        Err(ref err) if err.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("Client {} disconnected", addr);
                            return;
                        }
                    }

                    thread::sleep(Duration::from_millis(100));
                }
            });
        }

        if let Ok(msg) = rx.try_recv() {
            clients = clients.into_iter().filter_map(|mut client| {
                let mut buff = msg.clone().into_bytes();
                buff.push(b'\n');
                client.write_all(&buff).map(|_| client).ok()
            }).collect::<Vec<_>>();
        }

        thread::sleep(Duration::from_millis(100));
    }
}