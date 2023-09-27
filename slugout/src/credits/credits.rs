use bevy::prelude::*;

#[derive(Component, Deref, DerefMut)]
pub struct CreditsTimer(Timer);

#[derive(Component)]
pub struct Credits;

// Load credit screen assets
pub fn load_credit_texture(mut commands: Commands, asset_server: Res<AssetServer>){
    
    // Spawn credits sprite
    let credits_lib = vec!["Alex_Credits.png", "PaulR.png", "JakobR.png", "brayden.png", "nicolecredit.png", "lgy2credits.png", "RafaelCredits.png", "LukeCredits.png", "Jimmy.png"] ;
    // Let all sprites start with negative z for despawning conditions
    let mut starting_location = -1.;
    for slides in credits_lib{
        commands.spawn(SpriteBundle {
            texture: asset_server.load(slides),
            transform: Transform::from_xyz(0., 0., starting_location),
            ..default()
        }).insert(Credits).insert(CreditsTimer(Timer::from_seconds(3., TimerMode::Repeating)));
        // Put the next sprite even lower on the z coords
        starting_location -= 10.;
    }
  
}

pub fn cycle_credits(mut commands: Commands, mut move_credits: Query<(&mut Transform, &mut CreditsTimer, Entity), With<Credits>>, time: Res<Time>){
    for(mut transform, mut timer, entity) in move_credits.iter_mut(){ 
        timer.tick(time.delta());
        if timer.just_finished() {
            // move the location up so the shown slides can be despawn
            transform.translation.z += 10.;
            timer.reset();
        }
        if transform.translation.z >= 0.{
            // despawn shown slides
            commands.entity(entity).despawn();
        }
    }
}
