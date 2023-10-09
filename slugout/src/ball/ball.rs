use crate::components::*;
use bevy::prelude::*;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;

#[derive(Component)]
struct BallVelocity {
    velocity: Vec3,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, bounce);
    }
}

//ball Creation
fn setup(mut commands: Commands) {
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 2.0).with_scale(Vec3::new(10.0, 10.0, 2.0)),
            ..default()
        })
        .insert(Ball)
        .insert(BallVelocity {
            velocity: Vec3::new(300.0, 300.0, 2.0),
        });
}

//bounce the ball
fn bounce(time: Res<Time>, mut query: Query<(&mut Transform, &mut BallVelocity)>) {
    for (mut transform, mut ball) in query.iter_mut() {
        transform.translation += ball.velocity * time.delta_seconds();

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() > WIN_W / 2.0 {
            ball.velocity.x = -ball.velocity.x;
        }
        if transform.translation.y.abs() > WIN_H / 2.0 {
            ball.velocity.y = -ball.velocity.y;
        }
    }
}
