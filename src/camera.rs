use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreStartup, initialize_camera);
}

fn initialize_camera(mut commands: Commands) {
    commands.spawn(((Camera2d, Msaa::Off), MainCamera));
}
