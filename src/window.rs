use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800., 1600.).with_scale_factor_override(1.0),
                    resizable: false,
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(LogPlugin {
                filter: "info,wgpu_core=warn,wgpu_hal=warn,mygame=debug".into(),
                level: bevy::log::Level::DEBUG,
                custom_layer: |_| None,
            }),
    );
}
