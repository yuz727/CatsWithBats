// use crate::components::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    multiplayer::{ClientSocket, SocketAddress, PlayerInfo, ClientPlayerInfo, ClientListVector},
    MAP,
};

use super::components::{Bat, Face, Player, PlayerNumber};

use crate::game::npc::*;

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
const RUG_SPEED: f32 = 200.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 58000.;
const NPC_SIZE: f32 = 30.;


#[derive(Component, Clone, Serialize, Deserialize)]
pub struct PlayerVelocity {
    pub velocity: Vec2,
    pub prev_position: (f32, f32),
    pub confused: bool, 
}

impl PlayerVelocity {
    pub fn new() -> Self {
        Self {
            velocity: Vec2::splat(0.),
            prev_position: (0.0, 0.0),
            confused: false, 
        }
    }
}
pub fn move_player(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity),
        (With<Player>, Without<Face>, Without<Bat>),
    >,
    //  mut face: Query<&mut Transform, (With<Face>, Without<Player>, Without<Bat>)>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.single_mut();
    //  let mut face_transform = face.single_mut();
    //let mut bat_transform = bat.single_mut();

    let mut deltav = Vec2::splat(0.);
    if input.pressed(KeyCode::A) {
        deltav.x -= 1000.;
    }
    if input.pressed(KeyCode::D) {
        deltav.x += 1000.;
    }
    if input.pressed(KeyCode::W) {
        deltav.y += 1000.;
    }
    if input.pressed(KeyCode::S) {
        deltav.y -= 1000.;
    }

    // confusion check
    if velocity.confused {
        deltav = -deltav;
    }

    let deltat = time.delta_seconds();

    // Calculate change in vector
    let acc = ACCEL_RATE * deltat;

    if unsafe { MAP == 1 } {
        let rug_collision = bevy::sprite::collide_aabb::collide(
            Vec3::new(0., 0., 1.),
            Vec2::new(732.6, 507.6),
            transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        );
        if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
            || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
        {
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(RUG_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
        } else {
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc))
                    .clamp_length_max(PLAYER_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };
        }
        velocity.velocity = velocity.velocity * deltat;
        velocity.velocity = collision_check_map1(
            transform.translation,
            velocity.velocity,
            Vec3::splat(10000.),
        );
    } else if unsafe { MAP == 2 || MAP == 3 } {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        velocity.velocity = velocity.velocity * deltat;
    } else if unsafe { MAP == 4 } {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        velocity.velocity = velocity.velocity * deltat;
        velocity.velocity = collision_check_map4(
            transform.translation,
            velocity.velocity,
            Vec3::splat(10000.),
        );
    }

    // movement
    transform.translation.x = (transform.translation.x + velocity.velocity.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );

    transform.translation.y = (transform.translation.y + velocity.velocity.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );

    // face_transform.translation.x = transform.translation.x;
    // face_transform.translation.y = transform.translation.y;
    //bat_transform.translation.x = transform.translation.x - 5.;
    //bat_transform.translation.y = transform.translation.y;
}

pub fn player_npc_collisions(
    mut player: Query<&mut Transform, (With<Player>, Without<NPC>)>,
    mut npc_query: Query<&mut Transform, (With<NPC>, Without<Player>)>,
) {
    let mut player_transform = player.single_mut();

    for mut npc_transform in npc_query.iter_mut() {
        let collision = bevy::sprite::collide_aabb::collide(
            npc_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
            player_transform.translation,
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
        );
        if collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
            player_transform.translation.x = player_transform.translation.x - 5.;
            npc_transform.translation.x = npc_transform.translation.x + 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
            player_transform.translation.x = player_transform.translation.x + 5.;
            npc_transform.translation.x = npc_transform.translation.x - 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
            player_transform.translation.y = player_transform.translation.y - 5.;
            npc_transform.translation.y = npc_transform.translation.y + 5.;
        } else if collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
            player_transform.translation.y = player_transform.translation.y + 5.;
            npc_transform.translation.y = npc_transform.translation.y - 5.;
        }
    }
}

pub fn move_player_mult(
    input: Res<Input<KeyCode>>,
    mut player: Query<
        (&mut Transform, &mut PlayerVelocity),
        (With<Player>, Without<Face>, Without<Bat>),
    >,
    mut bat: Query<&mut Transform, (With<Bat>, With<Player>, Without<Face>)>,
    mut client_info: ResMut<crate::multiplayer::ClientSocket>,
    time: Res<Time>,
) {
    let (mut transform, mut velocity) = player.single_mut();
    // let mut face_transform = face.single_mut();
    let mut bat_transform = bat.single_mut();

    /////////////////////////////////////////////////////////////// with objects
    let recliner_size = Vec2::new(109., 184.);
    let recliner_translation = Vec3::new(-60., 210., 1.);
    let recliner = bevy::sprite::collide_aabb::collide(
        recliner_translation,
        recliner_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    let tv_size = Vec2::new(164., 103.);
    let tv_translation = Vec3::new(0., -245., 1.);
    let tv_stand = bevy::sprite::collide_aabb::collide(
        tv_translation,
        tv_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    let table_size = Vec2::new(103., 107.);
    let table_translation = Vec3::new(120., 170., 1.);
    let side_table = bevy::sprite::collide_aabb::collide(
        table_translation,
        table_size,
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );

    /////////////////////////////////////////////////////////////////////

    let mut deltav = Vec2::splat(0.);
    if input.pressed(KeyCode::A) {
        deltav.x -= 1000.;
    }
    if input.pressed(KeyCode::D) {
        deltav.x += 1000.;
    }
    if input.pressed(KeyCode::W) {
        deltav.y += 1000.;
    }
    if input.pressed(KeyCode::S) {
        deltav.y -= 1000.;
    }

    let deltat = time.delta_seconds();

    // Calculate change in vector
    let acc = ACCEL_RATE * deltat;
    let rug_collision = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 0., 1.),
        Vec2::new(732.6, 507.6),
        transform.translation,
        Vec2::new(PLAYER_SIZE, PLAYER_SIZE),
    );
    if (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Right))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Left))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Top))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom))
        || (rug_collision == Some(bevy::sprite::collide_aabb::Collision::Inside))
    {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(RUG_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
    } else {
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
    }
    velocity.velocity = velocity.velocity * deltat;
    /////////////////////
    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.8;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.9;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.velocity.x = 1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.velocity.x = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.velocity.y = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.velocity.y = 1. * 0.85;
    }
    ///////////////////////////

    // movement
    transform.translation.x = (transform.translation.x + velocity.velocity.x).clamp(
        -(1280. / 2.) + PLAYER_SIZE / 2.,
        1280. / 2. - PLAYER_SIZE / 2.,
    );

    transform.translation.y = (transform.translation.y + velocity.velocity.y).clamp(
        -(720. / 2.) + PLAYER_SIZE / 2.,
        720. / 2. - PLAYER_SIZE / 2.,
    );

    // face_transform.translation.x = transform.translation.x;
    // face_transform.translation.y = transform.translation.y;
    if transform.translation.x != velocity.prev_position.0
        || transform.translation.y != velocity.prev_position.1
    {
            //send updated info
            // make sure this struct has been instantiated
        if client_info.socket.is_none() {
            return;
        }

        info!("Sending player info");
        let socket = client_info.socket.as_mut().unwrap();
        socket.set_nonblocking(true).expect("could not set non-blocking");
            let old_position = (transform.translation.x, transform.translation.y);
            // println!("Old client position is {:?}", old_position);
        
            let player_info = PlayerInfo {
                position: (transform.translation.x, transform.translation.y),
                velocity: PlayerVelocity { velocity: Vec2 { x: velocity.velocity.x, y: velocity.velocity.y }, prev_position: (transform.translation.x, transform.translation.y), confused: false },
                health: 100,
            };
            // println!("New client position is {:?}", player_info.position);
        
            let message = serde_json::to_string(&player_info).expect("Failed to serialize");

            let id = "PLAFC";
            let big_message = id.to_string()  + &message;
            // println!("Sending my new positional data to the server, it is: {:?}", message);
            // println!("what is server address string {:?}", server_address_str);
            match socket.send(big_message.as_bytes()) {
                Ok(_) => {},
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    // Ignore WouldBlock errors
                },
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    println!("Socket is already connected");
                },
                Err(e) => {
                    println!("failed to send server updated position");
                    println!("Failed to send data: {:?}", e);
                }
            }
        
        velocity.prev_position = (transform.translation.x, transform.translation.y);
    }
    //bat_transform.translation.x = transform.translation.x - 5.;
    //bat_transform.translation.y = transform.translation.y;
}

pub fn collision_check_map1(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let recliner_size = Vec2::new(109., 184.);
    let recliner_translation = Vec3::new(-60., 210., 1.);
    let recliner = bevy::sprite::collide_aabb::collide(
        recliner_translation,
        recliner_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let tv_size = Vec2::new(164., 103.);
    let tv_translation = Vec3::new(0., -250., 1.);
    let tv_stand = bevy::sprite::collide_aabb::collide(
        tv_translation,
        tv_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let table_size = Vec2::new(103., 107.);
    let table_translation = Vec3::new(120., 170., 1.);
    let side_table = bevy::sprite::collide_aabb::collide(
        table_translation,
        table_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if recliner == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.8;
    } else if recliner == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.8;
    }

    if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if side_table == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}

pub fn collision_check_map4(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let coral_size = Vec2::new(150., 150.);
    let coral1 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral2 = bevy::sprite::collide_aabb::collide(
        Vec3::new(0., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral3 = bevy::sprite::collide_aabb::collide(
        Vec3::new(-320., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral4 = bevy::sprite::collide_aabb::collide(
        Vec3::new(-320., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral5 = bevy::sprite::collide_aabb::collide(
        Vec3::new(320., 180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let coral6 = bevy::sprite::collide_aabb::collide(
        Vec3::new(320., -180., 2.),
        coral_size,
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if coral1 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral1 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral2 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral2 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral3 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral3 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral4 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral4 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral5 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral5 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if coral6 == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.9;
    } else if coral6 == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.9;
    }

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }
    return velocity;
}

pub fn collision_check_no_objects(
    npc_translation: Vec3,
    mut velocity: Vec2,
    player_translation: Vec3,
) -> Vec2 {
    let player_collision = bevy::sprite::collide_aabb::collide(
        player_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
        npc_translation,
        Vec2::new(NPC_SIZE, NPC_SIZE),
    );

    if player_collision == Some(bevy::sprite::collide_aabb::Collision::Left) {
        velocity.x = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Right) {
        velocity.x = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Top) {
        velocity.y = -1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Bottom) {
        velocity.y = 1. * 0.85;
    } else if player_collision == Some(bevy::sprite::collide_aabb::Collision::Inside) {
        velocity.x = -1. * 0.85;
        velocity.y = -1. * 0.85;
    }

    return velocity;
}

pub fn update_other_players (
    mut others: Query<
        (&mut Transform, &mut PlayerVelocity, &PlayerNumber),
        (With<crate::multiplayer::server::OtherPlayer>, Without<Face>, Without<Bat>)>,
    mut others_bats: Query<
    (&mut Transform, &PlayerNumber),
    (With<crate::multiplayer::server::OtherPlayer>, Without<Face>, With<Bat>)>,
    player_num: Res<crate::multiplayer::PlayerNumber>,
    mut client_player_list: EventReader<ClientPlayerInfo>,
    client_list: Res<ClientListVector>
)
{
    for event in client_player_list.iter()
    {
        for client in event.data.iter()
        {
           
            if player_num.0 == client.username[4..client.username.len()].parse::<u32>().unwrap()
            {
                continue;
            }
            for (mut transform, mut velocity, player_number)  in others.iter_mut()
            {
                
                if player_number.number == client.username[4..client.username.len()].parse::<usize>().unwrap()
                {
                    transform.translation.x = client.player_info.as_ref().unwrap().position.0;
                    transform.translation.y = client.player_info.as_ref().unwrap().position.1;
                    *velocity = client.player_info.as_ref().unwrap().velocity.clone();
                }
            }   
            for (mut transform, player_number) in others_bats.iter_mut()
            {
                
                if player_number.number == client.username[4..client.username.len()].parse::<usize>().unwrap()
                {
                    if (client.bat_info.as_ref().unwrap().is_left)
                    {
                    transform.translation.x = client.player_info.as_ref().unwrap().position.0 - 5.;
                    transform.scale.x = transform.scale.x.abs();
                    }
                    else 
                    {
                        transform.translation.x = client.player_info.as_ref().unwrap().position.0 + 8.;
                        transform.scale.x = -transform.scale.x.abs();
                    }

                    transform.translation.y = client.player_info.as_ref().unwrap().position.1; 

                    if (client.bat_info.as_ref().unwrap().is_swinging)
                    {
                        transform.scale.y = -0.175;
                    }
                    else
                    {
                        transform.scale.y = 0.175;
                    }
                }
            }
        }
    }
}

