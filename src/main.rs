use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    window::WindowMode,
};
use bevy_prototype_lyon::plugin::ShapePlugin;

mod rectangles;

fn main() {
    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Rectangle canvas benchmark".to_string(),
        width: 1000.,
        height: 765.25,
        ..default()
    });
    app.insert_resource(ClearColor(Color::WHITE));
    app.add_plugins(DefaultPlugins);
    app.add_plugin(ShapePlugin);
    app.add_plugin(rectangles::RectanglesPlugin);
    app.add_startup_system(setup_cameras);
    app.add_system(full_screen_toggle);

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}

fn full_screen_toggle(mut windows: ResMut<Windows>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_released(KeyCode::F) {
        let window = windows.get_primary_mut().unwrap();
        window.set_mode(if window.mode() == WindowMode::Windowed {
            WindowMode::BorderlessFullscreen
        } else {
            WindowMode::Windowed
        });
    }
}
