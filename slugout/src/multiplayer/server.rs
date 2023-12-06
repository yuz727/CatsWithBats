use super::BallInfo;
use super::BatInfo;
use super::ClientListVector;
use super::ConnectRequest;
use super::PlayerInfo;
use crate::game::ball::BallNumber;
use crate::game::components::*;
use crate::game::player_movement::*;
use crate::multiplayer::client;
use crate::multiplayer::helper::decrypt_aes;
use crate::multiplayer::helper::decrypt_rsa;
use crate::multiplayer::helper::encrypt_aes;
use crate::multiplayer::server;
use bevy::prelude::*;
use rand::distributions::Alphanumeric;
use rand::Rng;
use rsa::pkcs1::EncodeRsaPublicKey;
use rsa::pkcs8::Document;
use rsa::pkcs8::LineEnding;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use serde::{Deserialize, Serialize};
use serde_json;
use std::convert;
use std::net::UdpSocket;
use std::str;
use rsa::{Error, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

//CONSTANTS FOR SERVER CLIENTS COMMUNICTATION
const SERVER_IDENTIFIER_ONE: &str = "MPK"; // MY PUBLIC KEY
const SERVER_IDENTIFIER_ONE_BYTES: &'static [u8] = SERVER_IDENTIFIER_ONE.as_bytes();
const SERVER_IDENTIFIER_TWO: &str = "SCC"; // SENT CLIENT CHALLENGE
const SERVER_IDENTIFIER_TWO_BYTES: &'static [u8] = SERVER_IDENTIFIER_TWO.as_bytes();
const SERVER_IDENTIFIER_THREE: &str = "CCR"; // CLIENT CHALLENGE RESULT
const SERVER_IDENTIFIER_THREE_BYTES: &'static [u8] = SERVER_IDENTIFIER_THREE.as_bytes();
const SERVER_IDENITIFIER_FOUR: &str = "SGM"; // SERVER GAME MESSAGE
const SERVER_IDENTIFIER_FOUR_BYTES: &'static [u8] = SERVER_IDENITIFIER_FOUR.as_bytes();
const SERVER_IDENTIFIER_FIVE: &str = "STG"; // Start game message
const SERVER_IDENTIFIER_FIVE_BYTES: &'static [u8] = SERVER_IDENTIFIER_FIVE.as_bytes();
const SERVER_IDENTIFIER_SIX: &str = "SLM"; // SERVER LOBBY MESSAGE
const SERVER_IDENTIFIER_SIX_BYTES: &'static [u8] = SERVER_IDENTIFIER_SIX.as_bytes();

const CLIENT_IDENTIFIER_ONE: &str = "PKR"; // PUBLIC KEY REQUEST
const CLIENT_IDENTIFIER_ONE_BYTES: &'static [u8] = CLIENT_IDENTIFIER_ONE.as_bytes();
const CLIENT_IDENTIFIER_TWO: &str = "SSK"; // SHARED SESSION KEY
const CLIENT_IDENTIFIER_TWO_BYTES: &'static [u8] = CLIENT_IDENTIFIER_TWO.as_bytes();
const CLIENT_IDENTIFIER_THREE: &str = "CAC"; // CLIENT ATTEMPTED CHALLENGE
const CLIENT_IDENTIFIER_THREE_BYTES: &'static [u8] = CLIENT_IDENTIFIER_THREE.as_bytes();
const CLIENT_IDENTIFIER_FOUR: &str = "CGM"; // CLIENT GAME MSG
const CLIENT_IDENTIFIER_FOUR_BYTES: &'static [u8] = CLIENT_IDENTIFIER_FOUR.as_bytes();


#[derive(Component)]
pub struct OtherPlayer;

pub fn create_server(
    mut server_info: ResMut<super::ServerSocket>,
    server_address: Res<super::SocketAddress>,
) {
    let server_address_str = &server_address.0;
    match UdpSocket::bind(server_address_str) {
        Ok(s) => {
            s.set_nonblocking(true).expect("Cannot set nonblocking");
            server_info.socket = Some(s);
            println!("Created server {}", server_address_str);
        }
        Err(e) => {
            println!("Failed to create server: {}", e);
            // You can add more debug information here
        }
    }
}

pub fn update(
    mut server_socket: ResMut<super::ServerSocket>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
) {
    // Check if the server socket is not yet initialized
    if server_socket.socket.is_none() {
        // If not initialized, return early from the function
        return;
    }

    let mut buf = [0; 1024];

    match server_socket.socket.as_mut().unwrap().recv_from(&mut buf) {
        Ok((amt, src)) => {
            let identifier = std::str::from_utf8(&buf[0..3]).unwrap();
            // println!("server side identifier is: {}", identifier);
            // println!("server amt is: {}", amt);
            let data: &[u8];
            if amt > 3 {
                data = &buf[3..amt];
                // println!("big data is: {}", str::from_utf8(&data).unwrap());
            } else {
                data = &buf[0..amt];
                // println!("small data is: {}", str::from_utf8(&data).unwrap());
            }

            // println!("looking at our clients:");
            let mut found_client = false;

            // First try to handle clients that are already authenticated
            for client in server_socket.authenticated_clients.iter_mut() {
                if client.client_address == src {
                    found_client = true;

                    /*handle_game_communication(
                        &mut commands,
                        &asset_server,
                        query,
                        server_socket,
                    );*/
                    // Update yarn ball vec in ServerSocket based on server balls' position
                    // if let Some(server) = &mut server_socket.socket {
                    //     // Clear existing yarn balls
                    //     server_socket.yarn_balls.clear();

                    //     // Update yarn balls based on the current state of the game
                    //     for (transform, _ball_velocity,_balll) in query.iter_mut() {
                    //         // Your logic to convert the game state to yarn balls
                    //         let yarn_ball = YarnBall{position: (transform.translation.x, transform.translation.y)};

                    //         // Add the yarn ball to the vector in ServerSocket
                    //         server_socket.yarn_balls.push(yarn_ball);
                    //     }
                    // }

                    

                    break;
                } else {
                    continue;
                }
            }

            // This is a new unauthenticated client, somewhere in the midst of authentication
            if !found_client {
                println!("did not find an authenticated client");
                // If they have not started authentication at all lets denote that
                if server_socket.clients.get(&src).is_none() {
                    server_socket
                        .clients
                        .insert(src, super::ServerHandshakeStage::NotStarted);
                }

                let clients_current_stage = server_socket.clients.get(&src).unwrap();
                // println!("clients current stage is: {:?}", clients_current_stage);

                // We have not sent them our public key yet
                if identifier == CLIENT_IDENTIFIER_ONE
                    && *clients_current_stage == super::ServerHandshakeStage::NotStarted
                {
                    println!("client {} is requesting public key", src);
                    let public_key = server_socket.public_key.as_ref().unwrap();

                    let document = public_key.to_pkcs1_pem(LineEnding::default()).unwrap();
                    let document_bytes = document.as_bytes();

                    let mut message = Vec::new();
                    message.extend_from_slice(SERVER_IDENTIFIER_ONE_BYTES);
                    message.extend_from_slice(document_bytes);

                    server_socket
                        .socket
                        .as_mut()
                        .unwrap()
                        .send_to(&message, src)
                        .unwrap();
                    server_socket
                        .clients
                        .insert(src, super::ServerHandshakeStage::SentPublicKey);
                    return;
                }
                // We already sent them our public key and now they are sending us their shared key

                // We already sent them our public key and now they are sending us their shared key
                else if identifier == CLIENT_IDENTIFIER_TWO
                    && *clients_current_stage == super::ServerHandshakeStage::SentPublicKey
                {
                    println!("client {} is sending us their shared key", src);
                    let encrypted_key = data;

                    let decrypted_session_key_vec =
                        decrypt_rsa(server_socket.private_key.as_ref().unwrap(), encrypted_key);
                    let decrypted_session_key_array: [u8; 16] = match decrypted_session_key_vec
                        .unwrap()
                        .try_into()
                    {
                        Ok(arr) => arr,
                        Err(_) => {
                            println!("Failed to convert decrypted session key to a 16-byte array");
                            return;
                        }
                    };
                    // Now lets store this client's session key
                    server_socket
                        .client_keys
                        .insert(src, Some(decrypted_session_key_array));

                    // Time to create a challenge for the client to solve
                    let mut rng = rand::thread_rng();
                    let challenge = rng.gen::<u64>();
                    server_socket.client_challenges.insert(src, challenge);
                    let challenge_bytes = challenge.to_be_bytes();
                    let mut message = Vec::new();
                    let ciphertext = encrypt_aes(&decrypted_session_key_array, &challenge_bytes);
                    message.extend_from_slice(SERVER_IDENTIFIER_TWO_BYTES);
                    message.extend_from_slice(&ciphertext);

                    // Encrypt message with aes key
                    server_socket
                        .socket
                        .as_mut()
                        .unwrap()
                        .send_to(&message, src)
                        .unwrap();
                    server_socket
                        .clients
                        .insert(src, super::ServerHandshakeStage::SentChallenge);
                    println!("Sent client a challenge!");

                    return;
                }
                // Authenticate client
                else if identifier == CLIENT_IDENTIFIER_THREE
                    && *clients_current_stage == super::ServerHandshakeStage::SentChallenge
                {
                    // decrypt challenge and check if it is correct
                    let encrypted_challenge = data;
                    let aes_key = server_socket.client_keys.get(&src).unwrap();
                    let decrypted_challenge = decrypt_aes(&aes_key.unwrap(), encrypted_challenge);
                    // Now lets break this challenge up into its timestamp, and random nonce
                    // parse json for nonce and timestamp
                    let challenge_json = str::from_utf8(&decrypted_challenge).unwrap();
                    let challenge_json: serde_json::Value =
                        serde_json::from_str(challenge_json).unwrap();
                    let nonce = challenge_json["nonce"].as_u64().unwrap();
                    let timestamp = challenge_json["timestamp"].as_u64().unwrap();
                    // Now lets check if nonce is within 15 seconds of current time
                    let current_time = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    let challenge_nonce = server_socket.client_challenges.get(&src).unwrap();

                    // Guard against replay attacks and success or failure of challenge
                    if  nonce != challenge_nonce + 1 {
                        println!("Client {} failed to solve challenge in time or provided incorrect answer", src);

                        // This client failed so lets remove their key and challenge from ServerSocket
                        let mut message = Vec::new();
                        message.extend_from_slice(SERVER_IDENTIFIER_THREE_BYTES);
                        let cipher_text = encrypt_aes(
                            &server_socket.client_keys.get(&src).unwrap().unwrap(),
                            "FAILED".as_bytes(),
                        );
                        message.extend_from_slice(&cipher_text);

                        // Failure
                        server_socket
                            .socket
                            .as_mut()
                            .unwrap()
                            .send_to(&message, src)
                            .unwrap();
                        server_socket
                            .clients
                            .insert(src, super::ServerHandshakeStage::FailedAuthentication);

                        // Reset communications completely
                        server_socket.client_keys.remove(&src);
                        server_socket.client_challenges.remove(&src);
                        server_socket.clients.remove(&src);
                        return;
                    } else {
                        // Client solved challenge so lets send them a success message
                        println!("Client {} successfully authenticated", src);

                        let mut message = Vec::new();
                        message.extend_from_slice(SERVER_IDENTIFIER_THREE_BYTES);
                        let cipher_text = encrypt_aes(
                            &server_socket.client_keys.get(&src).unwrap().unwrap(),
                            "PASSED".as_bytes(),
                        );
                        message.extend_from_slice(&cipher_text);

                        server_socket
                            .socket
                            .as_mut()
                            .unwrap()
                            .send_to(&message, src)
                            .unwrap();

                        // represent client is authenticated, don't remove key or challenge
                        server_socket
                            .clients
                            .insert(src, super::ServerHandshakeStage::Authenticated);
                        // Add client to authenticated clients list
                        let client_length = server_socket.authenticated_clients.len();
                        let new_username = 
                            format!("user{}", client_length + 1);
                        server_socket
                            .authenticated_clients
                            .push(super::AuthenticatedClient {
                                client_address: src,
                                username: new_username,
                                player_info: Some(super::PlayerInfo {
                                    position: determine_start_pos(client_length),
                                    velocity: PlayerVelocity {velocity:Vec2 { x: 0., y: 0. }, prev_position: determine_start_pos(client_length), confused: false},
                                    health: 3,
                                }),
                                bat_info: Some(BatInfo {
                                    is_swinging: false,
                                    is_left: true
                                })
                            });

                        return;
                    }
                } 
                else if identifier == CLIENT_IDENTIFIER_FOUR
                && *clients_current_stage == super::ServerHandshakeStage::Authenticated {
                    
                } else {
                    println!("unrecognized handshake message");
                }
            }
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // Handle the case where no data is available to read
            //println!("No data available to read");
            //std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        Err(e) => {
            // Handle other errors
            println!("other error")
        }
    }
}

/*pub fn handle_game_communication(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut query: Query<(&mut Transform, &mut BallVelocity, &mut Ball), (With<Ball>, Without<Player>)>,
    mut server_socket: ResMut<super::ServerSocket>,
) {
    // Update yarn ball vec in ServerSocket based on server balls' position
    if let Some(server) = &mut server_socket.socket {
        // Clear existing yarn balls
        server_socket.yarn_balls.clear();

        // Update yarn balls based on the current state of the game
        for (transform, ball_velocity, ball) in query.iter_mut() {
            // Your logic to convert the game state to yarn balls
            let yarn_ball = YarnBall{position: (transform.translation.x, transform.translation.y)};

            // Add the yarn ball to the vector in ServerSocket
            server_socket.yarn_balls.push(yarn_ball);
        }
    }
}*/


pub fn determine_start_pos(player_number: usize) -> (f32, f32) {

    if player_number > 4 {
        panic!("Player number must be between 1 and 4");
    }
    return match player_number {
        1 => (-225.0, 225.0),
        2 => (225.0, 225.0),
        3 => (-225.0, -225.0),
        4 => (225.0, -225.0),
        _ => (0., 0.),
    }
}


pub fn send_start_signal(server_socket: ResMut<super::ServerSocket>) {
    // Iterate over authenticated clients
    //for authenticated_client in &server_socket.authenticated_clients {
    // need to send message to clients to start the game
    //}
    let mut message =  Vec::new();
    message.extend_from_slice(SERVER_IDENTIFIER_FIVE_BYTES);
    for client in server_socket.authenticated_clients.iter()
    {
        server_socket.socket.as_ref().unwrap().send_to(&message, client.client_address).expect("Failed to send data.");
    }

    println!("Sent Start Signal")
}
pub fn send_client_list(server_socket: ResMut<super::ServerSocket>) {
    let mut usernames_bytes = Vec::new();
    let mut list_as_strings = Vec::new();
    for client in server_socket.authenticated_clients.iter() {
   
        list_as_strings.push(serde_json::to_string(client).unwrap());
    }
    for client in list_as_strings.iter()
    {
        // println!("{}", client);
        usernames_bytes.push(client.as_bytes());
        usernames_bytes.push("$".as_bytes());
    }
    
    let flattened_usernames: Vec<u8> = usernames_bytes.into_iter().flatten().copied().collect();
    // First loop that grabs all usernames
        for client in server_socket.authenticated_clients.iter() {
        // Get clients address and client key
        let client_address = client.client_address;
        let client_key = server_socket.client_keys.get(&client_address).unwrap().unwrap();
        
        let mut message =  Vec::new();
        let ciphertext = encrypt_aes(&client_key, &flattened_usernames);
        message.extend_from_slice(SERVER_IDENTIFIER_SIX_BYTES);
        message.extend_from_slice(&ciphertext);
        server_socket.socket.as_ref().unwrap().send_to(&message, client_address).unwrap();
        
    }
}


pub fn received_update(
    mut server_socket: ResMut<super::ServerSocket>,
    mut other_players: Query<
        (&mut Transform, &mut PlayerNumber),
        (With<OtherPlayer>,(Without<Face>, Without<Bat>))>,
    mut other_players_bats: Query<
    (&mut Transform, &mut PlayerNumber),
    (With<OtherPlayer>,(Without<Face>, With<Bat>))>,
    mut balls: Query<
    (&mut BallVelocity, &BallNumber),
    With<Ball>>,
    )
    {
    if server_socket.socket.is_none() {
        // If not initialized, return early from the function
        return;
    }
    server_socket.socket.as_mut().unwrap().set_nonblocking(true).expect("could not set as non-blocking");
    let mut buf = [0; 1024];
    match server_socket.socket.as_mut().unwrap().recv_from(&mut buf) {

        Ok((amt, src)) => {
            // No data 
            if amt == 0
            {
                return;
            }
            // There was data 
            let id = std::str::from_utf8(&buf[0..5]).unwrap();
            if id == "PLAFC"
            {
                let data = std::str::from_utf8(&buf[5..amt]).unwrap();
                // println!("server received player data!");
                // We can now assume that a player has moved 
                let info: PlayerInfo = serde_json::from_str(data).unwrap();
                for client in server_socket.authenticated_clients.iter_mut()
                {   
                    // This is the player that has moved 
                    if client.client_address.eq(&src)
                    {   
                        // Are they not me 
                        let mut they_are_me = true;
                        for (mut transform, mut player_number) in other_players.iter_mut()
                        {
                            if player_number.number == client.username[4..client.username.len()].parse::<usize>().unwrap()
                            {   
                                //update screen
                                they_are_me = false;
                                // transform.translation.x = info.position.0;
                                // transform.translation.y = info.position.1;
                                client.player_info = Some(info.clone());
                            }
                        }
                        if they_are_me { 
                            client.player_info = Some(info.clone());
                        }
                    
                    }
                }
                // Now we can send this data back to the clients 
                let message = serde_json::to_string(&server_socket.authenticated_clients).expect("Failed to serialize");
                let server_id = "PLAFS";
                // Now we can send this data back to the clients 
                let big_message  = server_id.to_string() + &message;
                for client in server_socket.authenticated_clients.iter()
                {
                    server_socket.socket.as_ref().unwrap().send_to(big_message.as_bytes(), client.client_address).expect("Failed to send data.");
                }
            }
            else if id == "BALFC"
            {

                let data = std::str::from_utf8(&buf[5..amt]).unwrap();
                // We can now assume that a player has moved 
                let info: BallInfo = serde_json::from_str(data).unwrap();
                for ball in server_socket.yarn_balls.iter_mut()
                {
                    if ball.ball_number == info.ball_number
                    {
                        ball.position = info.position.clone();
                        ball.velocity = info.velocity.clone();
                    }
                }
                
                let message = serde_json::to_string(&server_socket.yarn_balls).expect("Failed to serialize");
                let server_id = "BALFS";
                // Now we can send this data back to the clients 
                let big_message  = server_id.to_string() + &message;
                // println!("sending data {}", big_message);
                for client in server_socket.authenticated_clients.iter()
                {
                    server_socket.socket.as_ref().unwrap().send_to(big_message.as_bytes(), client.client_address).expect("Failed to send data.");
                }
            }
            else if id == "BATFC"
            {
                let data = std::str::from_utf8(&buf[5..amt]).unwrap();
            
                // We can now assume that a player has moved 
                let info: BatInfo = serde_json::from_str(data).unwrap();
                for client in server_socket.authenticated_clients.iter_mut()
                {   
                    // This is the bat that has moved 
                    if client.client_address.eq(&src)
                    {   
                        // Are they not me 
                        let mut they_are_me = true;
                        for (mut transform, mut player_number) in other_players_bats.iter_mut()
                        {
                            for(mut player_transform, _player_number) in other_players.iter()
                            {
                                if player_number.number == client.username[4..client.username.len()].parse::<usize>().unwrap()
                                {   
                                    they_are_me = false;
                                    
                                    // if info.is_left
                                    // {
                                    //     transform.translation.x = player_transform.translation.x - 5.;
                                    // }
                                    // else 
                                    // {
                                    //     transform.translation.x = player_transform.translation.x + 8.;
                                    // }
                                    // //TODO ALSO NEED TO HANDLE IS SWINGING 

                                    client.bat_info = Some(info.clone());
                                }
                            }
                        }
                        if they_are_me { 
                            client.bat_info = Some(info.clone());
                        }
                    
                    }
                }
                // Now we can send this data back to the clients 
                let message = serde_json::to_string(&server_socket.authenticated_clients).expect("Failed to serialize");
                let server_id = "BATFS";
                // Now we can send this data back to the clients 
                let big_message  = server_id.to_string() + &message;
                for client in server_socket.authenticated_clients.iter()
                {
                    server_socket.socket.as_ref().unwrap().send_to(big_message.as_bytes(), client.client_address).expect("Failed to send data.");
                }
            }
            else if id == "PHIFC" {
                let data = std::str::from_utf8(&buf[5..amt]).unwrap();
                let info: PlayerInfo = serde_json::from_str(data).unwrap();
                for client in server_socket.authenticated_clients.iter_mut()
                {   
                    // This is the player that has moved 
                    if client.client_address.eq(&src)
                    {   
                        // Are they not me 
                        let mut they_are_me = true;
                        for (mut transform, mut player_number) in other_players.iter_mut()
                        {
                            if player_number.number == client.username[4..client.username.len()].parse::<usize>().unwrap()
                            {   
                                //update screen
                                they_are_me = false;
                                // transform.translation.x = info.position.0;
                                // transform.translation.y = info.position.1;
                                client.player_info = Some(info.clone());
                            }
                        }
                        if they_are_me { 
                            client.player_info = Some(info.clone());
                        }
                    
                    }
                }
                // Now we can send this data back to the clients 
                let message = serde_json::to_string(&server_socket.authenticated_clients).expect("Failed to serialize");
                let server_id = "PHIFC";
                // Now we can send this data back to the clients 
                let big_message  = server_id.to_string() + &message;
                for client in server_socket.authenticated_clients.iter()
                {
                    server_socket.socket.as_ref().unwrap().send_to(big_message.as_bytes(), client.client_address).expect("Failed to send data.");
                }

            }
            
          
        }
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // Handle the case where no data is available to read
            //println!("No data available to read");
            //std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        Err(e) => {
            println!("Error: {:?}", e);
        }
    }
}
// Grab all the player updates and them to the server 





// pub fn update(
//     mut server_socket: ResMut<super::ServerSocket>,
//     mut client_list: ResMut<super::ClientList>,
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut player: Query<
//         (&mut Transform, &mut PlayerNumber),
//         (With<OtherPlayer>, Without<Face>, Without<Bat>),
//     >,
//     mut face: Query<(&mut Transform, &PlayerNumber), (With<Face>, With<PlayerNumber>, Without<Player>, Without<Bat>)>,
//     mut bat: Query<(&mut Transform, &PlayerNumber), (With<Bat>, With<PlayerNumber>, Without<Player>, Without<Face>)>,
// ) {
//     // Create a buffer to store incoming data
//     let mut buf = [0; 1024];
//     // Check if the server socket is not yet initialized
//     if server_socket.0.is_none() {
//         // If not initialized, return early from the function
//         return;
//     }
//     // Unwrap to get a mutable reference to the server socket
//     let socket = server_socket.0.as_mut().unwrap();
//     // Set the socket to non-blocking mode
//     socket
//         .set_nonblocking(true)
//         .expect("Cannot set nonblocking");
//     // Attempt to receive data from the socket
//     match socket.recv_from(&mut buf) {
//         // If data is successfully received
//         Ok((size, peer)) => {
//             // Convert the received bytes to a string
//             let client_msg = str::from_utf8(&buf[0..size]).expect("Bad data.");
//             // Get a mutable reference to the client list
//             let clients = &mut client_list.clients;
//             // Determine the player number based on whether the client already exists
//             let player_number =
//                 if let Some(index) = clients.iter().position(|client| client.address == peer) {
//                     // Existing client, get the player number
//                     index + 1
//                 } else {
//                     // This is a new client, add it to the list
//                     clients.push(super::Client {
//                         address: peer,
//                         username: String::from(generate_username(10)),
//                     });
//                     println!("New client connected: {}", peer);
//                     // Assign player number as the length of the client list
//                     clients.len()
//                 };
//             // Check if the received message is a connection request from a new client
//             if let Ok(_connect_request) = serde_json::from_str::<ConnectRequest>(&client_msg) {
//                 // Only spawn the player if the player_number is not 1
//                 if player_number != 1 {
//                     // just spawn the png's for player for now (need to make it work to show different Player#.png's)
//                     commands
//                         .spawn(SpriteBundle {
//                             texture: asset_server.load("Player2.png"),
//                             transform: Transform::with_scale(Transform::from_xyz(0., 0., 10.), Vec3::splat(0.13)),
//                             ..default()
//                         })
//                         .insert(OtherPlayer)
//                         .insert(PlayerVelocity::new())
//                         .insert(Colliding::new())
//                         .insert(PlayerNumber{number: player_number});

//                     commands
//                         .spawn(SpriteBundle {
//                             texture: asset_server.load("Face.png"),
//                             transform: Transform::with_scale(Transform::from_xyz(0., 0., 20.), Vec3::splat(0.13)),
//                             ..default()
//                         })
//                         .insert(Face)
//                         .insert(PlayerNumber{number: player_number});

//                     commands
//                         .spawn(SpriteBundle {
//                             texture: asset_server.load("Bat.png"),
//                             transform: Transform::with_scale(
//                                 Transform::from_xyz(-5., 0., 20.),
//                                 Vec3::new(0.175, 0.175, 0.),
//                             ),
//                             ..default()
//                         })
//                         .insert(Bat)

//                         .insert(PlayerNumber{number: player_number});
//                 }
//                 /*/ Send a response back to the client with the assigned player ID
//                 let response_msg = serde_json::to_string(&ConnectResponse { player_number }).expect("Failed to serialize");
//                 socket
//                     .send_to(response_msg.as_bytes(), peer)
//                     .expect("Failed to send data");*/
//             }
//             if let Ok(player_info) = serde_json::from_str::<PlayerInfo>(&client_msg) {
//                 // Handle player_info and perform game logic here
//                 if player_number > 1
//                 {
//                     println!(
//                         "Player {} - Received Player Info: Position: {:?}, Health: {}",
//                         player_number, player_info.position, player_info.health
//                     );

//                     for (mut transform, player_num) in player.iter_mut()
//                     {
//                         if player_num.number == player_number
//                         {
//                             transform.translation.x = player_info.position.0;
//                             transform.translation.y = player_info.position.1;
//                         }
//                         for (mut bat_transform, player_numb) in bat.iter_mut()
//                         {
//                             // if(player_numb.number == player_number)
//                             // {
//                             //     if ((mouse_position.x - WIN_W) / 2.) > transform.translation.x{
//                             //         bat_transform.translation = transform.translation;
//                             //         bat_transform.translation.x = bat_transform.translation.x + 8.;
//                             //         bat_transform.scale.x = -0.175;
//                             //     } else {
//                             //         bat_transform.translation = transform.translation;
//                             //         bat_transform.translation.x = bat_transform.translation.x - 5.;
//                             //         bat_transform.scale.x = 0.175;
//                             //     }
//                             // }
//                         }
//                     }
//                     for(mut transform, player_num) in face.iter_mut()
//                     {
//                         if player_num.number == player_number
//                         {
//                             transform.translation.x = player_info.position.0;
//                             transform.translation.y = player_info.position.1;
//                         }
//                     }

//                 }
//             }

//             // Send the received message back to the client
//             socket
//                 .send_to(client_msg.as_bytes(), peer)
//                 .expect("Failed to send data");
//         }
//         Err(_e) => {
//             //eprintln!("Error receiving data: {}", _e);
//         }
//     }
// }



// if !(client.handshake_stage == super::Ser::Authenticated) {
//     match client.handshake_stage {
//         // We sent them public key and they responded aka shareSessionKey
//         super::HandshakeStage::RequestPublicKey => {

//             let encrypted_session_key = data.as_bytes();
//             println!("encrypted session key: {:?}", encrypted_session_key);
//             println!("data really equals    : {:?}", data);
//             match decrypt_rsa(server_socket.private_key.as_ref().unwrap(), encrypted_session_key) {
//                 Ok(decrypted_session_key) => {
//                     println!("decrypted session key: {:?}", decrypted_session_key);
//                 }
//                 Err(e) => {
//                     println!("Failed to decrypt session key: {:?}", e);
//                 }
//             }
//         }
//         super::HandshakeStage::ShareSessionKey => {
//             // Do nothing for now
//         }
//         super::HandshakeStage::ChallengeCheck => {
//             // Do nothing for now
//         }
//         super::HandshakeStage::Authenticated => {
//             // Do nothing for now
//         }
//     }

// }