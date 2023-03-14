use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};
use bevy_prototype_lyon::plugin::ShapePlugin;

mod rectangles;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Rectangle canvas benchmark".to_string(),
            resolution: WindowResolution::new(1000., 765.25),
            ..default()
        }),
        ..default()
    }));
    app.insert_resource(ClearColor(Color::WHITE));
    app.add_plugin(ShapePlugin);
    app.add_plugin(rectangles::RectanglesPlugin);
    app.add_startup_system(setup_cameras);
    app.add_system(full_screen_toggle.run_if(pressed_f));

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_plugin(LogDiagnosticsPlugin::default());
        for schedule in [
            CoreSchedule::Startup,
            CoreSchedule::Main,
            CoreSchedule::Outer,
            CoreSchedule::FixedUpdate,
        ] {
            app.edit_schedule(schedule, |schedule| {
                schedule.set_build_settings(ScheduleBuildSettings {
                    ambiguity_detection: LogLevel::Warn,
                    ..default()
                });
            });
        }
    }

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn pressed_f(keyboard_input: Res<Input<KeyCode>>) -> bool {
    keyboard_input.just_released(KeyCode::F)
}

fn full_screen_toggle(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::BorderlessFullscreen
    } else {
        WindowMode::Windowed
    };
}
