use crate::components::*;
use bevy::prelude::*;

const WIN_W: f32 = 1280.;
const WIN_H: f32 = 720.;
const BALL_SIZE: f32 = 10.;
const PLAYER_SIZE: f32 = 30.;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, bounce);
    }
}

//ball Creation
fn setup(mut commands: Commands) {

    commands.spawn(SpriteBundle {
        transform: Transform::from_xyz(0.0, 0.0, 2.0).with_scale(Vec3::new(10.0, 10.0,2.0)),
        ..default()
    }) .insert(Ball) .insert(crate::components::BallVelocity {
        velocity: Vec3::new(300.0, 300.0, 2.0),
    }).insert(Colliding::new());
}

//bounce the ball
fn bounce(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Colliding, &mut crate::components::BallVelocity), (With<Ball>, Without<Player>)>,
    mut player_colliding: Query<&mut Colliding, (With<Player>, Without<Ball>)>,
    player_transform: Query<&Transform, (With<Player>, Without<Ball>)>,
) {
    let mut pc = player_colliding.single_mut();
    let pt = player_transform.single();

    for (mut transform, mut colliding, mut ball) in query.iter_mut() {

        // Find the new translation for the x and y for the ball
        let new_translation_x = (transform.translation.x + (ball.velocity.x * time.delta_seconds())).clamp(
            -(1280. / 2.) + BALL_SIZE / 2.,
            1280. / 2. - BALL_SIZE / 2.,
        );

        let new_translation_y = (transform.translation.y + (ball.velocity.y * time.delta_seconds())).clamp(
            -(720. / 2.) + BALL_SIZE / 2.,
            720. / 2. - BALL_SIZE / 2.,
        );

        // Check for collision with player
        let collision = bevy::sprite::collide_aabb::collide(pt.translation, Vec2::new(PLAYER_SIZE,
             PLAYER_SIZE), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let recliner = bevy::sprite::collide_aabb::collide(Vec3::new(120., 160., 1.), 
        Vec2::new(158.,
            178.), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let tv_stand = bevy::sprite::collide_aabb::collide(Vec3::new(-300., -150., 1.), 
        Vec2::new(160.,
            160.), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        let side_table = bevy::sprite::collide_aabb::collide(Vec3::new(280., 20., 1.), 
        Vec2::new(125.,
            113.), transform.translation, Vec2::new(BALL_SIZE, BALL_SIZE));

        // Check for a collision with a player
        if collision == Some(bevy::sprite::collide_aabb::Collision::Left) || collision == Some(bevy::sprite::collide_aabb::
            Collision::Right){
            // Collision with left or right side
            // Adjust colliding variables accordingly
            colliding.currently_colliding = true;
            pc.currently_colliding = true;
            ball.velocity.x = -ball.velocity.x;
            //info!("111111111111111111111");
        }else if collision == Some(bevy::sprite::collide_aabb::Collision::Top) || collision == Some(bevy::sprite::collide_aabb::
            Collision::Bottom){
            // Collision with top or bottom side
            // Adjust colliding variables accordingly
            colliding.currently_colliding = true;
            pc.currently_colliding = true;
            ball.velocity.y = -ball.velocity.y;
        }else if collision == Some(bevy::sprite::collide_aabb::Collision::Inside){
            // Collision with inside
            // Adjust colliding variables accordingly
            colliding.currently_colliding = true;
            pc.currently_colliding = true;
            ball.velocity.x = -ball.velocity.x;
            ball.velocity.y = -ball.velocity.y;
            //info!("222222222222222");
        }else{
            // No Collision
            // Adjust colliding variables accordingly
            colliding.currently_colliding = false;
            pc.currently_colliding = false;
        }

        //other collisions//////////////////////////////////////////////////////
        //let rec = true;
 
        if recliner == Some(bevy::sprite::collide_aabb::Collision::Left) || recliner == Some(bevy::sprite::collide_aabb::
            Collision::Right){
            ball.velocity.x = -ball.velocity.x;
            //info!("33333333333333");
        }else if recliner == Some(bevy::sprite::collide_aabb::Collision::Top) || recliner == Some(bevy::sprite::collide_aabb::
            Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
        }
        // else if recliner == Some(bevy::sprite::collide_aabb::Collision::Inside){
        //     ball.velocity.x = -ball.velocity.x;
        //     ball.velocity.y = -ball.velocity.y;
        // }

        if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Left) || tv_stand == Some(bevy::sprite::collide_aabb::
            Collision::Right){
            ball.velocity.x = -ball.velocity.x;
            //info!("444444444444444");
        }else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Top) || tv_stand == Some(bevy::sprite::collide_aabb::
            Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
        }
        // else if tv_stand == Some(bevy::sprite::collide_aabb::Collision::Inside){
        //     ball.velocity.x = -ball.velocity.x;
        //     ball.velocity.y = -ball.velocity.y;
        // }

        if side_table == Some(bevy::sprite::collide_aabb::Collision::Left) || side_table == Some(bevy::sprite::collide_aabb::
            Collision::Right){
            ball.velocity.x = -ball.velocity.x;
        }else if side_table == Some(bevy::sprite::collide_aabb::Collision::Top) || side_table == Some(bevy::sprite::collide_aabb::
            Collision::Bottom){
            ball.velocity.y = -ball.velocity.y;
        }
        // else if side_table == Some(bevy::sprite::collide_aabb::Collision::Inside){
        //     ball.velocity.x = -ball.velocity.x;
        //     ball.velocity.y = -ball.velocity.y;
        // }

        // Move ball
        transform.translation.x = new_translation_x;
        transform.translation.y = new_translation_y;

        // Bounce when hitting the screen edges
        if transform.translation.x.abs() == WIN_W / 2.0 - BALL_SIZE / 2. {
            ball.velocity.x = -ball.velocity.x;
            //info!("55555555555555");
        }
        if transform.translation.y.abs() == WIN_H / 2.0 - BALL_SIZE / 2.{
            ball.velocity.y = -ball.velocity.y;
        }
    }
}
