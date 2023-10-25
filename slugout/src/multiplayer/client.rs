use std::io::{self, Write};
use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_client() -> io::Result<()> {
    //ip.push_str(":8080");
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let mut player_info = PlayerInfo {
        position: (42.0, 24.0),
        health: 100,
    };

    let mut move_left = true; // Flag to indicate movement direction

    loop {
        // Update player position based on movement direction
        // ** THIS IS ONLY USED TO EMULATE CHANGE/MOVEMENT **
        if move_left {
            player_info.position.0 -= 1.0; // Move left
        } else {
            player_info.position.0 += 1.0; // Move right
        }

        // Serialize player info to JSON
        let client_msg = serde_json::to_string(&player_info).expect("Failed to serialize");

        // Write the JSON message to the server
        stream.write(client_msg.as_bytes()).expect("Failed to write");

        // Toggle the movement direction
        // ** THIS IS ONLY USED TO EMULATE CHANGE/MOVEMENT **
        move_left = !move_left;

        // Sleep to control the sending frequency
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}