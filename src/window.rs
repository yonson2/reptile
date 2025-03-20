#[allow(unused_imports)]
use bevy::asset::AssetMetaCheck;

use bevy::{log::LogPlugin, prelude::*, window::WindowResolution};

pub(super) fn plugin(app: &mut App) {
    #[allow(unused_mut)]
    let mut plugins = DefaultPlugins
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
        });

    // Only set AssetPlugin for wasm32 target
    #[cfg(target_arch = "wasm32")]
    {
        plugins = plugins.set(AssetPlugin {
            meta_check: AssetMetaCheck::Never,
            ..default()
        });
    }

    app.add_plugins(plugins);
}
