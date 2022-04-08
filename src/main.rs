use bevy::{
    app::App,
    core_pipeline::ClearColor,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    ecs::system::{Commands, Res, ResMut},
    input::{keyboard::KeyCode, Input},
    render::{camera::OrthographicCameraBundle, color::Color},
    ui::entity::UiCameraBundle,
    window::{WindowDescriptor, WindowMode, Windows},
    DefaultPlugins,
};
use bevy_prototype_lyon::plugin::ShapePlugin;
use bevy_screen_diags::ScreenDiagsPlugin;

mod rectangles;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    let mut app = App::new();

    app.insert_resource(WindowDescriptor {
        title: "Rectangle canvas benchmark".to_string(),
        width: 1000.,
        height: 765.25,
        ..Default::default()
    })
    .insert_resource(ClearColor(Color::WHITE))
    .add_plugins(DefaultPlugins)
    .add_plugin(ScreenDiagsPlugin)
    .add_plugin(ShapePlugin)
    .add_plugin(rectangles::RectanglesPlugin)
    .add_startup_system(setup_cameras)
    .add_system(full_screen_toggle);

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}

fn setup_cameras(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
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
