// use bevy::{prelude::*, window::PresentMode};

// pub fn load_credits(mut commands: Commands, asset_server: Res<AssetServer>){
//     commands.spawn(SpriteBundle {
//         texture: asset_server.load("Crystal.jpg"),
//         transform: Transform::from_xyz(0., 0., -1.),
//         ..default()
//     });
//     credits_alex(&commands.transform);
// }

// pub fn credits_alex(Transform: &mut transform){
//     transform.translation.z = 2.0;
//     info!("Test")
// }