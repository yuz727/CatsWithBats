use crate::components::*;
use bevy::prelude::*;

const PLAYER_SIZE: f32 = 30.;

const BALL_SIZE: f32 = 1.;

// define hitbox component
#[derive(Component)]
struct Hitbox;

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(Update, player_ball_collision);
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
