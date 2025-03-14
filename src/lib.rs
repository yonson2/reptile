use bevy::prelude::*;

mod assets;
mod camera;
mod game;
mod window;

pub struct AppPlugin;
impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((window::plugin, camera::plugin, assets::plugin, game::plugin));
    }
}
