use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::{SocketAddr, UdpSocket};
//use std::os::unix::thread;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use super::{ClientBallInfo, ClientPlayerInfo};
use std::str;
use crate::MultiplayerState;
use crate::multiplayer::client;
// use rsa::pkcs1::{EncodeRsaPublicKey, DecodeRsaPublicKey};
use crate::multiplayer::helper::decrypt_aes;
use crate::multiplayer::helper::encrypt_aes;
use crate::multiplayer::helper::encrypt_rsa;
use crate::multiplayer::ConnectRequest;
use rand::RngCore;
use rsa::pkcs1::DecodeRsaPublicKey;

use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

use super::BallListVector;
use super::ClientListVector;


#[derive(Component)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
}

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

pub fn create_client(
    mut client_info: ResMut<super::ClientSocket>,
    server_address: Res<super::SocketAddress>,
    mut _query: Query<(&Transform, &PlayerVelocity)>,
) {
    let server_address_str = &server_address.0;
    match server_address_str.parse::<SocketAddr>() {
        Ok(server_address) => {
            // Create a new socket for each connection
            match UdpSocket::bind("0.0.0.0:0") {
                Ok(s) => {
                    println!("connecitng to server address: {}", server_address);
                    match s.connect(server_address) {
                        Ok(_) => {
                            let connect_request = ConnectRequest {};
                            match serde_json::to_string(&connect_request) {
                                Ok(connect_request_msg) => {
                                    s.send(connect_request_msg.as_bytes())
                                        .expect("Failed to send connection request");

                                    s.set_nonblocking(true).expect("cannot set nonblocking");
                                    // Store the connected
                                    client_info.socket = Some(s.try_clone().unwrap());
                                    client_info.server_address = Some(server_address);
                                    client_info.client_address = Some(s.local_addr().unwrap());
                                    client_info.stage =
                                        super::ClientHandshakeStage::RequestedPublicKey;
                                    // What if the server was never called so the client does not have a copy of the servers public key yet?
                                    println!("Created client as well, time to send public key request to server");
                                    s.send(CLIENT_IDENTIFIER_ONE_BYTES)
                                        .expect("Failed to send public key request");
                                }
                                Err(e) => {
                                    println!("Failed to serialize connect request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to connect to the server: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to bind to address: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to parse server address: {}", e);
        }
    }
}

pub fn update(
    mut client_info: ResMut<super::ClientSocket>,
    server_address: Res<super::SocketAddress>,
    mut multiplayer_state: ResMut<NextState<MultiplayerState>>,
    mut client_list_for_display: ResMut<super::ClientListVector>,
) {
    let mut buf = [0; 1024];

    // make sure this struct has been instantiated
    if client_info.socket.is_none() {
        return;
    }

    // println!("Doing update");

    match client_info.socket.as_ref().unwrap().recv_from(&mut buf) {
        Ok((amt, src)) => {
            let identifier = std::str::from_utf8(&buf[0..3]).unwrap();
            let data = &buf[3..amt];
            // println!("client side identifier is: {}", identifier);
            // println!("client amt is: {}", amt);

            // We have received the servers public key, lets store it and continue
            if client_info.stage == super::ClientHandshakeStage::RequestedPublicKey
                && identifier == SERVER_IDENTIFIER_ONE
            {
                // println!("Public key as raw is {:?}");
                // We have received the servers public key, lets store it
                let public_key =
                    match RsaPublicKey::from_pkcs1_pem(std::str::from_utf8(data).unwrap()) {
                        Ok(key) => key,
                        Err(e) => {
                            println!("Failed to parse public key: {}", e);
                            return;
                        }
                    };

                // turn this data into a string
                client_info.server_public_key = Some(public_key);

                // Now lets create a shared session key and send it to the server, encrypted with their private key.
                let mut aes_key = [0u8; 16];
                rand::thread_rng().fill_bytes(&mut aes_key);
                println!("Unencrypted AES key: {:?}", aes_key);

                client_info.aes_key = Some(aes_key);
                let stored_public_key = client_info.server_public_key.as_ref().unwrap().clone();
                let encrypted_key = encrypt_rsa(&stored_public_key, &aes_key).unwrap();
                let mut socket = client_info.socket.as_mut().unwrap();
                let encrypted_key_bytes = &encrypted_key;
                let mut message = Vec::new();
                message.extend_from_slice(CLIENT_IDENTIFIER_TWO_BYTES);
                message.extend_from_slice(encrypted_key_bytes);

                // Send the message to the server
                socket.send(&message).unwrap();
                println!("Sent encrypted AES key to server");
                client_info.stage = super::ClientHandshakeStage::SharedSessionKey;
                return;
            } else if client_info.stage == super::ClientHandshakeStage::SharedSessionKey
                && identifier == SERVER_IDENTIFIER_TWO
            {
                println!("Received challenge from the server!");

                let encrypted_nonce = data;
                // We have received a random nonce, and a timestamp from the server, lets decrypt with the AES key we have and get to work

                let aes_key = client_info.aes_key.as_ref().unwrap();
                let decrypted_nonce = decrypt_aes(aes_key, encrypted_nonce);
                let nonce_as_int = u64::from_be_bytes(decrypted_nonce.try_into().unwrap());
                let incremented_nonce = nonce_as_int + 1;
                // Now we must generate a timestamp
                let now = std::time::SystemTime::now();
                let since_the_epoch = now.duration_since(std::time::UNIX_EPOCH).unwrap();

                // Now we must encrypt the incremented nonce and timestamp with the AES key
                let nonce_and_timestamp = json!({
                    "nonce": incremented_nonce,
                    "timestamp": since_the_epoch.as_secs()
                });
                let encrypted_nonce_and_timestamp =
                    encrypt_aes(aes_key, nonce_and_timestamp.to_string().as_bytes());

                // Now we must send the encrypted nonce and timestamp back to the server
                let mut socket = client_info.socket.as_mut().unwrap();
                let encrypted_nonce_and_timestamp_bytes = &encrypted_nonce_and_timestamp;
                let mut message = Vec::new();
                message.extend_from_slice(CLIENT_IDENTIFIER_THREE_BYTES);
                message.extend_from_slice(encrypted_nonce_and_timestamp_bytes);
                socket.send(&message).unwrap();
                client_info.stage = super::ClientHandshakeStage::RespondedToChallenge;
                println!("Sent client challenge Response to server");
                return;
            }
            // Were we authenticated
            else if client_info.stage == super::ClientHandshakeStage::RespondedToChallenge
                && identifier == SERVER_IDENTIFIER_THREE
            {
                // lets see if we passed the challenge, and decrypt server response
                let aes_key = client_info.aes_key.as_ref().unwrap();
                let decrypted_server_response = decrypt_aes(aes_key, data);

                if String::from_utf8(decrypted_server_response).unwrap() == "PASSED" {
                    println!("Challenge passed, we are now authenticated by the server");
                    client_info.stage = super::ClientHandshakeStage::Authenticated;
                } else {
                    println!("Challenge failed, we have failed to authenticate with the server");
                    client_info.stage = super::ClientHandshakeStage::FailedAuthentication;
                    // For now you are terminated from the game if you fail to authenticate
                    //TODO: Terminate client from game
                }
                return;
            }
            // Game communications
            else if client_info.stage == super::ClientHandshakeStage::Authenticated
                && identifier == SERVER_IDENITIFIER_FOUR
            {
                //game comm
                // Never hit 
            } 

            // Send start signal 
            else if client_info.stage == super::ClientHandshakeStage::Authenticated
                && identifier == SERVER_IDENTIFIER_FIVE
            {
                println!("received start signal from server");
                multiplayer_state.set(super::MultiplayerState::Game);
            }  
            // Update the client list so it can be seen on the front end  
            else if client_info.stage == super::ClientHandshakeStage::Authenticated && identifier == SERVER_IDENTIFIER_SIX {
                let aes_key = client_info.aes_key.as_ref().unwrap(); 
                let decrypted_usernames = decrypt_aes(aes_key, data);
                let usernames = String::from_utf8(decrypted_usernames).unwrap();
                let each_username: Vec<&str> = usernames.split("$").collect();
                let string_vector: Vec<String> = each_username.iter().map(|s| s.to_string()).collect();
                let mut client_list_vec = Vec::new();
                let mut counter = 0;
                let limit = string_vector.len() - 1;
                for user in string_vector
                {
                    if counter == limit
                    {
                        break;
                    }
                    let test: Result<super::AuthenticatedClient, serde_json::Error>  = serde_json::from_str(&user.trim());
                    match test{
                        Ok(c) =>  client_list_vec.push(c),
                        Err(e) => println!("{:?}", e),
                    }
                    counter = counter + 1;
                }
                client_list_for_display.0 = client_list_vec;
            }
            else {
                println!("Received unknown message from server");
                println!("message is: {}", std::str::from_utf8(&buf[0..amt]).unwrap());
                return;
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

// This function should run every frame and send the server the updated position of the player


// This function is called when the client is in the game state, a player movement / other change has occurred, such that 
// we want to reupdate the players on the screen 
// If we want to do it on Update there is gonna be plenty of times when this is blank
pub fn update_client_data (
    mut client_list: ResMut<ClientListVector>,
    mut client_socket: ResMut<super::ClientSocket>,
    mut ball_list: ResMut<BallListVector>,
    mut client_player_bat_writer: EventWriter<ClientPlayerInfo>,
    mut client_ball_writer: EventWriter<ClientBallInfo>,

)
{
    if client_socket.socket.is_none() {
        return;
    }
    let socket = client_socket.socket.as_mut().unwrap();
    socket.set_nonblocking(true).expect("could not set non-blocking");
    let mut buf = [0; 1024];

    match client_socket.socket.as_mut().unwrap().recv_from(&mut buf) {
        Ok((amt, _src)) => {
            if amt == 0 {
                println!("No data received");
            } else {
               
                let id = std::str::from_utf8(&buf[0..5]).unwrap();
                let data = std::str::from_utf8(&buf[5..amt]);
                if id == "PLAFS" || id ==  "BATFS" || id == "PHIFC" {
                    match data{
                        Ok(s) => 
                        {
                            // info!("Got player data {}", s);
                            
                            client_list.0 = serde_json::from_str(s).unwrap();
                            client_player_bat_writer.send(ClientPlayerInfo{data: client_list.0.clone()});
                            //info!("{:?}", s);
                        }
                        Err(e) => {
                            println!("Reading data error: {:?}", e);
                        }
                    }
                }
               else if id == "BALFS" {
                // println!("client got ball data");
                    match data{
                        Ok(s) => 
                        {
                            let info: Vec<super::BallInfo> = serde_json::from_str(s).unwrap();
                            ball_list.0 = info;
                            client_ball_writer.send(ClientBallInfo{data: ball_list.0.clone()});
                           
                        }
                        Err(e) => {
                            println!("Reading data error: {:?}", e);
                        }
                    }
               }
            }
        },
        Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
            // Handle the case where no data is available to read
            //println!("No data available to read");
            //std::thread::sleep(std::time::Duration::from_millis(1000));
        },
        Err(e) => {
            println!("{:?}", e);
        }
    }
}


/*pub fn update(
    mut client_socket: ResMut<super::ClientSocket>,
    socket_address: Res<super::SocketAddress>,
    mut query: Query<(&Transform, &PlayerVelocity)>,
) {
    let mut _buf = [0; 1024];

    if client_socket.0.is_none() {
        return;
    }

    let socket = client_socket.0.as_mut().unwrap();
    socket
        .set_nonblocking(true)
        .expect("cannot set nonblocking");

    let (transform, velocity) = match query.iter().next() {
        Some((transform, velocity)) => (transform, velocity),
        None => {
            // Handle the case when there are no entities matching the query
            println!("No entities found matching the query.");
            return;
        }
    };

    let player_info = PlayerInfo {
        position: (transform.translation.x, transform.translation.y),
        health: 100,
    };

    let message = serde_json::to_string(&player_info).expect("Failed to serialize");

    let server_address_str = &socket_address.0;
    socket
        .send_to(message.as_bytes(), server_address_str)
        .expect("Failed to send data.");

    let mut response = [0; 1024];

    match socket.recv_from(&mut response) {
        Ok((size, _peer)) => {
            let response_str = std::str::from_utf8(&response[0..size]).expect("Bad data.");
            println!("Received response: {}", response_str);
        }
        Err(_e) => {
            //eprintln!("Error receiving data: {}", e);
        }
    }

    // Sleep to control the sending frequency
}*/