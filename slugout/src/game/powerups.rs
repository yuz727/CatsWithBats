use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::multiplayer::{ClientSocket, SocketAddress};

use super::components::{Bat, Player, PowerUp, Hitbox};


const PLAYER_SIZE: f32 = 30.; // size of power up as well
// 5px/frame @60Hz == 300px/s
//const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
//const ACCEL_RATE: f32 = 58000.;


pub fn spawn_powerups(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(240, 140, 100),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform::from_xyz(80., -80., 10.),
            ..Default::default()
        })
        .insert(PowerUp {
            powerup: "bigbat".to_string(),
        });
}

pub fn player_powerups(
    mut player_query: Query<(&mut Transform, &mut Player), (With<Player>, Without<PowerUp>)>,
    mut powerups: Query<(&mut Transform, &mut PowerUp, &mut Visibility), (With<PowerUp>, Without<Player>)>,
    
){
    let (mut player_transform, mut player) = player_query.single_mut();

    for (mut powerup_transform, mut powerup, mut powerup_visibility) in powerups.iter_mut(){
        let collision = bevy::sprite::collide_aabb::collide(player_transform.translation, 
            Vec2::new(PLAYER_SIZE, PLAYER_SIZE), powerup_transform.translation, Vec2::new(PLAYER_SIZE, PLAYER_SIZE));
        if collision == Some(bevy::sprite::collide_aabb::Collision::Left) || collision == Some(bevy::sprite::collide_aabb::Collision::Right) || collision == Some(bevy::sprite::collide_aabb::Collision::Top) || collision == Some(bevy::sprite::collide_aabb::Collision::Bottom){
            //println!("running");
            if player.powerup == "none".to_string(){
                player.powerup = powerup.powerup.clone();
                player.powerup_timer = 15.;
                //println!("running");
                *powerup_visibility = Visibility::Hidden;
            }
        }
    }

}

pub fn apply_powerups(
    time: Res<Time>,
    mut player_query: Query<&mut Player, (With<Player>, Without<PowerUp>, Without<Hitbox>, Without<Bat>)>,
    mut bat: Query<&mut Transform, (With<Bat>, Without<PowerUp>, Without<Player>, Without<Hitbox>)>,
    mut hitbox_query: Query<(&mut Transform, &mut Hitbox), (With<Hitbox>, Without<PowerUp>, Without<Player>, Without<Bat>)>,
){ 
    let mut player = player_query.single_mut();
    let mut bat_transform = bat.single_mut();
    let (mut hitbox_transform, mut hitbox) = hitbox_query.single_mut();
    

    if player.powerup == "bigbat".to_string(){ 
        if player.powerup_timer == 15.{
            *bat_transform = bat_transform.with_scale(Vec3::new(0.3, 0.3, 0.));
            *hitbox_transform = hitbox_transform.with_scale(Vec3::new(1.75, 1.75, 0.));
            hitbox.size = Vec2::new(78.75, 131.25);
        }
        player.powerup_timer = player.powerup_timer - time.delta_seconds();
        if player.powerup_timer <= 0.{
            player.powerup = "none".to_string();
            println!("done running");
            *bat_transform = bat_transform.with_scale(Vec3::new(0.175, 0.175, 0.));
            *hitbox_transform = hitbox_transform.with_scale(Vec3::new(1., 1., 0.));
            hitbox.size = Vec2::new(45., 75.);

        }


    }

}
