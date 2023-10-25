use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use serde::{Serialize, Deserialize};
use serde_json; 

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_server() -> io::Result<()> {
    let mut player_count = 1;
    let receiver_listener = TcpListener::bind("0.0.0.0:8080").expect("Could not bind to port");
    // Create a vector for new client threads
    let mut thread_vector: Vec<thread::JoinHandle<()>> = Vec::new();
    // for every incominng client try to bind them to port
    for stream in receiver_listener.incoming() {
        let new_stream = stream.expect("Failed to accept client connection");

        println!("Player {} joined.", player_count);
        player_count += 1;

        let handle = thread::spawn(move || {
            handle_new_sender(new_stream).unwrap_or_else(|error| eprintln!("{:?}", error))
        });

        // put each successive message in vector array
        thread_vector.push(handle);
    }
    
    for handle in thread_vector {
        // unwrap message that was sent 
        handle.join().unwrap();
    }

    Ok(())
}

fn handle_new_sender(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        // read what client sent to buffer
        let bytes_read = stream.read(&mut buf)?;
        // return if nothing was sent 
        if bytes_read == 0 {
            return Ok(());
        }
        // Deserialize the received JSON into PlayerInfo
        let client_msg = String::from_utf8_lossy(&buf[..bytes_read]);
        if let Ok(player_info) = serde_json::from_str::<PlayerInfo>(&client_msg) {
            // Handle player_info and perform game logic here
            println!(
                "Received Player Info: Position: {:?}, Health: {}",
                player_info.position, player_info.health
            );
        }

        // Echo back the received message (for demonstration)
        stream.write(&buf[..bytes_read])?;
    }
}