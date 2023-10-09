use bevy::prelude::*;
use crate::components::*;

const PLAYER_SIZE: f32 = 30.;

const BALL_SIZE: f32 = 1.;

// define hitbox component
#[derive(Component)]
struct Hitbox;

fn setup_hitbox(mut commands: Commands){
    commands
        .spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb_u8(100, 170, 200),
                custom_size: Some(Vec2::splat(PLAYER_SIZE)),
                ..default()
            },
            transform: Transform {
                translation: Vec3::new(0., 0., 0.),
                ..default()
            },
            ..default()
        })
        .insert(Hitbox);
}

pub fn player_ball_collision(
    time: Res<Time>,
    player: Query<&mut Transform, (With<Player>, Without<Ball>)>,
    balls:   Query<&Transform, (With<Ball>, Without<Player>)>,
){
    let pt = player.single();

    // For every ball
    for transform in balls.iter(){

        // Check for a collision with a player
        if bevy::sprite::collide_aabb::collide(pt.translation, Vec2::new(PLAYER_SIZE, PLAYER_SIZE), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE)).is_some(){
            // Collision
        }
    }
}

