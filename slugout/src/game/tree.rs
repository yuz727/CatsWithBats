use crate::game::npc::States;
use crate::game::npc::*;
use crate::game::npc_events::*;
use crate::game::pathfinding::*;
use crate::GameState;
use bevy::prelude::*;
use rand::prelude::*;
//use bevy::time::Stopwatch;

use super::components::Ball;
use super::components::BallVelocity;
use super::components::Object;
use super::components::Player;

const NPC_SIZE: f32 = 30.;
// 5px/frame @60Hz == 300px/s
const NPC_SPEED: f32 = 300.;
// 1px/frame^2 @60Hz == 3600px/s^2
const NPC_ACCEL_RATE: f32 = 18000.;

// enum States
pub struct NPCTreePlugin;

impl Plugin for NPCTreePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), load_npc);
        app.add_systems(Update, selection);
        app.add_systems(Update, perform_a_star.after(selection));
        app.add_systems(Update, sidestep.after(selection));
        app.add_systems(Update, swing);
        // Aggressive Nodes
        // app.add_systems(Update, danger_check);

        // app.add_systems(Update, player_proximity_check);
        // app.add_systems(Update, tag_is_null);
        // app.add_systems(Update, set_tag_to_closest_ball);
        // app.add_systems(Update, aggression_check);
        // app.add_systems(Update, swing_cooldown_check);
        // app.add_systems(Update, set_tag_to_closest_object);
        //
        // app.add_systems(Update, set_a_star);
        //
    }
}

// fn every_other_time() -> impl Condition<()> {
//     IntoSystem::into_system(|mut flag: Local<bool>| {
//         *flag = !*flag;
//         *flag
//     })
// }
enum Target {
    Player,
    Ball,
    None,
}

fn selection(
    mut npc: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
            &mut States,
            &mut NPCTimer,
        ),
        (With<NPC>, Without<NPCBat>, Without<Ball>, Without<Player>),
    >,
    // mut bat: Query<(
    //     &mut Transform,
    //     (With<NPCBat>, Without<Player>, Without<NPC>, Without<Ball>),
    //  )>,
    player: Query<&mut Transform, (With<Player>, Without<NPC>, Without<Ball>, Without<NPCBat>)>,
    time: Res<Time>,
    ball_query: Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) {
    for (
        mut npc_transform,
        mut npc_velocity,
        mut path,
        maps,
        difficulty,
        mut ani_timer,
        mut state,
        mut swing_timer,
    ) in npc.iter_mut()
    {
        //for mut bat_transform in bat.iter_mut() {
        for player_transform in player.iter() {
            let danger = danger_check(npc_transform.translation, &time, &ball_query);
            if danger {
                if difficulty_check(difficulty.difficulty)
                    && swing_cooldown_check(&mut swing_timer, &time)
                {
                    if set_tag_to_closest_ball(npc_transform.translation, &mut path, &ball_query) {
                        set_a_star(npc_transform.translation, &mut path, maps);
                    } // if
                    return;
                } // if
                return;
            } // if
            if player_proximity_check(npc_transform.translation, player_transform.translation)
                && difficulty_check(difficulty.difficulty)
                && swing_cooldown_check(&mut swing_timer, &time)
            {
                set_tag_to_player(&mut path, player_transform.translation);
                set_a_star(npc_transform.translation, &mut path, maps);
                return;
            } // if
        } // for
          //} // for
    } // for
}

fn danger_check(
    npc_translation: Vec3,
    time: &Res<Time>,
    ball_query: &Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) -> bool {
    // For every ball
    for (ball_transform, ball_velocity, ball) in ball_query.iter() {
        // If a ball is close enough (< 200 pixels away) and it is moving towards the npc, then return true
        let ball_future_position = Vec2::new(
            (ball_transform.translation.x + (ball_velocity.velocity.x * time.delta_seconds()))
                .clamp(-(1280. / 2.) + ball.radius, 1280. / 2. - ball.radius),
            (ball_transform.translation.y + (ball_velocity.velocity.y * time.delta_seconds()))
                .clamp(-(720. / 2.) + ball.radius, 720. / 2. - ball.radius),
        );
        if ball_transform.translation.distance(npc_translation) < 200.
            && ball_future_position.distance(npc_translation.truncate())
                < ball_transform.translation.distance(npc_translation)
        {
            return true;
        }
    }
    return false;
}

fn sidestep(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
) {
    // Implement sidestep logic
    // return NodeStatus::Success;
}

fn player_proximity_check(npc_translation: Vec3, player_translation: Vec3) -> bool {
    // If player is close enough (< 200 pixels away) and it is moving towards the npc, then return true
    if player_translation.distance(npc_translation) < 200. {
        return true;
    }
    return false;
}

fn tag_is_null(path: &Path) -> bool {
    // Implement TAG null check logic
    if path.goal.x == -1. && path.goal.y == -1. {
        return true;
    }
    return false;
}

/*  Find the closest ball to NPC, and set the goal to it.
 */
fn set_tag_to_closest_ball(
    npc_translation: Vec3,
    path: &mut Path,
    ball_query: &Query<
        (&Transform, &BallVelocity, &Ball),
        (With<Ball>, Without<NPC>, Without<NPCBat>, Without<Player>),
    >,
) -> bool {
    let mut ret = Vec2::splat(10000000000.);
    for (ball_transform, _, _) in ball_query.iter() {
        if ball_transform.translation.distance(npc_translation)
            < npc_translation.truncate().distance(ret)
        {
            ret = ball_transform.translation.truncate();
        }
    }
    if ret.x == 10000000000. && ret.y == 10000000000. {
        return false;
    }
    path.goal = ret;
    return true;
}

fn difficulty_check(difficulty: i32) -> bool {
    let mut rand = thread_rng();
    if difficulty > rand.gen_range(0.0..100.0) as i32 {
        return true;
    }
    return false;
}

fn aggression_check(mut npcs: Query<(&States,), With<NPC>>) -> bool {
    return true;
}

fn swing_cooldown_check(swing_timer: &mut NPCTimer, time: &Res<Time>) -> bool {
    swing_timer.tick(time.delta());
    if swing_timer.just_finished() {
        return true;
    }
    return false;
}
fn set_tag_to_closest_object(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    object_query: Query<&Transform, With<Object>>,
) {
    // Implement setting TAG to the closest object logic
    // return NodeStatus::Success;
}

fn set_tag_to_player(path: &mut Path, player_translation: Vec3) {
    path.goal = player_translation.truncate().floor();
}

fn set_a_star(npc_translation: Vec3, path: &mut Path, maps: &Maps) {
    // Implement A* pathfinding logic
    let goal = path.goal;
    path.set_new_path(a_star(
        coords_conversion_astar(npc_translation.truncate().floor()),
        coords_conversion_astar(goal),
        maps,
    ));
}

fn perform_a_star(
    mut npcs: Query<
        (&mut Transform, &mut NPCVelocity, &mut Path, &Maps),
        (With<NPC>, Without<Ball>, Without<Player>, Without<NPCBat>),
    >,
    mut bat: Query<
        &mut Transform,
        (
            With<NPCBat>,
            Without<Player>,
            Without<NPC>,
            Without<Ball>,
            Without<NPCFace>,
        ),
    >,
    time: Res<Time>,
) {
    for (mut npc_transform, mut velocity, mut path, maps) in npcs.iter_mut() {
        for mut bat_transform in bat.iter_mut() {
            //  for mut face_transform in face.iter_mut() {
            let Some(Vec2 { x, y }) = path.path.pop() else {
                return;
            };

            let mut deltav = Vec2::splat(0.);
            if npc_transform.translation.x < x {
                deltav.x += 1000.;
            }
            if npc_transform.translation.x > x {
                deltav.x -= 1000.;
            }
            if npc_transform.translation.y < y {
                deltav.y += 1000.;
            }
            if npc_transform.translation.y > y {
                deltav.y -= 1000.;
            }

            let deltat = time.delta_seconds();
            let acc = NPC_ACCEL_RATE * deltat;
            velocity.velocity = if deltav.length() > 0. {
                (velocity.velocity + (deltav.normalize_or_zero() * acc)).clamp_length_max(NPC_SPEED)
            } else if velocity.velocity.length() > acc {
                velocity.velocity + (velocity.velocity.normalize_or_zero() * -acc)
            } else {
                Vec2::splat(0.)
            };

            let recliner_size = Vec2::new(112., 184.);
            let recliner_translation = Vec3::new(-60., 210., 1.);
            let recliner = bevy::sprite::collide_aabb::collide(
                recliner_translation,
                recliner_size,
                npc_transform.translation,
                Vec2::new(NPC_SIZE, NPC_SIZE),
            );

            let tv_size = Vec2::new(164., 104.);
            let tv_translation = Vec3::new(0., -250., 1.);
            let tv_stand = bevy::sprite::collide_aabb::collide(
                tv_translation,
                tv_size,
                npc_transform.translation,
                Vec2::new(NPC_SIZE, NPC_SIZE),
            );

            let table_size = Vec2::new(104., 108.);
            let table_translation = Vec3::new(120., 170., 1.);
            let side_table = bevy::sprite::collide_aabb::collide(
                table_translation,
                table_size,
                npc_transform.translation,
                Vec2::new(NPC_SIZE, NPC_SIZE),
            );

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

            velocity.velocity = velocity.velocity * deltat;
            if velocity.xlock == 0 {
                npc_transform.translation.x = (npc_transform.translation.x + velocity.velocity.x)
                    .clamp(-(1280. / 2.) + NPC_SIZE / 2., 1280. / 2. - NPC_SIZE / 2.);
            }
            if velocity.ylock == 0 {
                npc_transform.translation.y = (npc_transform.translation.y + velocity.velocity.y)
                    .clamp(-(720. / 2.) + NPC_SIZE / 2., 720. / 2. - NPC_SIZE / 2.);
            }

            // Fixes Misalign
            if npc_transform.translation.x != x || npc_transform.translation.y != y {
                npc_transform.translation.x = x;
                npc_transform.translation.y = y;
            }
            bat_transform.translation.x = npc_transform.translation.x - 5.;
            bat_transform.translation.y = npc_transform.translation.y;
        }
    }
}

fn swing(
    mut npcs: Query<
        (
            &mut Transform,
            &mut NPCVelocity,
            &mut Path,
            &Maps,
            &Difficulty,
            &mut AnimationTimer,
        ),
        With<NPC>,
    >,
    ball_query: Query<&Transform, With<Ball>>,
    // _: &QuerySet<(Entity, &Player)>,
    // _: &QuerySet<(Entity, &GameObject)>,
) {
    // Implement swing logic
    // return NodeStatus::Success;
}
