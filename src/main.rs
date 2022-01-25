use bevy::{
    app::App,
    core_pipeline::ClearColor,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::Commands,
    render::{camera::OrthographicCameraBundle, color::Color},
    window::WindowDescriptor,
    DefaultPlugins,
};
use bevy_prototype_lyon::plugin::ShapePlugin;

mod rectangles;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rectangle canvas benchmark".to_string(),
            width: 1000.,
            height: 765.25,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(ShapePlugin)
        .add_startup_system(setup_system)
        .add_startup_system(rectangles::setup)
        .add_system(rectangles::bounds_updater)
        .add_system(rectangles::movement)
        .add_system(rectangles::collision_detection)
        .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
