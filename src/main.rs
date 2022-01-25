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

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
    .add_plugin(ShapePlugin)
    .add_plugin(rectangles::RectanglesPlugin)
    .add_startup_system(setup_system);

    if cfg!(debug_assertions) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }

    app.run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
