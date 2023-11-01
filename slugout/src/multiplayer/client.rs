use std::net::UdpSocket;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct PlayerInfo {
    position: (f32, f32),
    health: u32,
    // Used basic fields for Player info for now
    // Add other relevant fields here
}

pub fn create_client() {
    let server_address = "127.0.0.1:8080";
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind to address.");

    let mut player_info = PlayerInfo {
        position: (42.0, 24.0),
        health: 100,
    };

    let mut move_left = true; // Flag to indicate movement direction


    loop {
        if move_left {
            player_info.position.0 -= 1.0; // Move left
        } else {
            player_info.position.0 += 1.0; // Move right
        }

        let message = serde_json::to_string(&player_info).expect("Failed to serialize");

        socket.send_to(message.as_bytes(), server_address).expect("Failed to send data.");

        let mut response = [0; 1024];
        let (size, _peer) = socket.recv_from(&mut response).expect("Failed to receive data");
        let response_str = std::str::from_utf8(&response[0..size]).expect("Bad data.");

        // Toggle the movement direction
        // ** THIS IS ONLY USED TO EMULATE CHANGE/MOVEMENT **
        move_left = !move_left;

        // Sleep to control the sending frequency
        std::thread::sleep(std::time::Duration::from_secs(5));

        println!("Received response: {}", response_str);   
    }

}