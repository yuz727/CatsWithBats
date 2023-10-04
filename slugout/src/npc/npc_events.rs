use bevy::prelude::*;
use std::time::Duration;
use rand::prelude::*;

use super::npc::{MovementTimer, NPC, NPCVelocity};

const PLAYER_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const PLAYER_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const ACCEL_RATE: f32 = 3600.;


pub fn rand_movement(mut npcs: Query<(&mut Transform, &mut MovementTimer, &mut NPCVelocity), With<NPC>>, time: Res<Time>){
    let (mut transform, mut timer, mut velocity) = npcs.single_mut();
    let mut rng = thread_rng();
   
    timer.tick(time.delta());
    if timer.just_finished() {
        // The duration of before next movement change
        timer.set_duration(Duration::from_secs(rng.gen_range(0..5)));

        // Decide change in velocity based on rng
        let mut deltav = Vec2::splat(0.);
        let result_x = rng.gen_range(-10..10);
        let result_y =  rng.gen_range(-10..10);
        if result_x < 0 {
            deltav.x -= 10.;
        }
        if result_x > 0 {
            deltav.x += 10.;
        }
        if result_y < 0 {
            deltav.y -= 10.;
        }
        if result_y > 0 {
            deltav.y += 10.;
        }

        let deltat = time.delta_seconds();
     
        // for debugging
        // info!(deltav.x);
        // info!(deltav.y);

        // Calculate change in vector
        let acc = ACCEL_RATE * deltat;    
        velocity.velocity = if deltav.length() > 0. {
            (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(PLAYER_SPEED)
        } else if velocity.velocity.length() > acc {
            velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
        } else {
            Vec2::splat(0.)
        };
        velocity.velocity = velocity.velocity * deltat;
        // velocity.velocity = change;
        
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
}