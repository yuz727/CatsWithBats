use bevy::{prelude::*};
use crate::components::*;


const PLAYER_SIZE: f32 = 30.;

const BALL_SIZE: f32 = 1.;

// define hitbox component
#[derive(Component)]
struct Hitbox;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin
{
    fn build(&self, app: &mut App){
        app.add_systems(Update, player_ball_collision);
    }
}

/*fn setup_hitbox(mut commands: Commands){
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
}*/

pub fn player_ball_collision(
    mut player_collidings: Query<&mut Colliding, (With<Player>, Without<Ball>)>,
    player_transforms: Query<&Transform, (With<Player>, Without<Ball>)>,
    mut balls:   Query<(&Transform, &mut Colliding, &mut BallVelocity), (With<Ball>, Without<Player>)>,
){
    let pt = player_transforms.single();
    let mut pc = player_collidings.single_mut();


    // For every ball
    for (transform, mut colliding, velocity ) in balls.iter_mut(){

        let collision = bevy::sprite::collide_aabb::collide(pt.translation, Vec2::new(PLAYER_SIZE, PLAYER_SIZE), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));
        // Check for a collision with a player
        if collision.is_some(){
            // Collision
            colliding.currently_colliding = true;
            pc.currently_colliding = true;
            bounce_ball(velocity, collision)
        }else{
            // No Collision
            colliding.currently_colliding = false;
            pc.currently_colliding = false;
        }

    }
}

fn bounce_ball(mut ball_velocity: Mut<'_, BallVelocity>, collision: Option<bevy::sprite::collide_aabb::Collision>){
    if collision == Some(bevy::sprite::collide_aabb::Collision::Left) || collision == Some(bevy::sprite::collide_aabb::Collision::Right){
        ball_velocity.velocity.x = -ball_velocity.velocity.x;
    }else if collision == Some(bevy::sprite::collide_aabb::Collision::Top) || collision == Some(bevy::sprite::collide_aabb::Collision::Bottom){
        ball_velocity.velocity.y = -ball_velocity.velocity.y;
    }else if collision == Some(bevy::sprite::collide_aabb::Collision::Inside){
        ball_velocity.velocity.x = -ball_velocity.velocity.x;
        ball_velocity.velocity.y = -ball_velocity.velocity.y;
    }
}

