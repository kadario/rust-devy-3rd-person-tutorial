use bevy::prelude::*;
use bevy_third_person_camera::*;
use player::PlayerPlugin;
use camera::CameraPlugin;
use world::WorldPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod camera;
mod world;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins, 
            PlayerPlugin, 
            CameraPlugin,
            WorldPlugin,
            ThirdPersonCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .run();
}