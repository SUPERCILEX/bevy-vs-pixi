#![allow(clippy::needless_pass_by_value)]

use std::{fmt::Write, time::Duration};

use bevy::{
    app::MainScheduleOrder,
    diagnostic::{Diagnostic, DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
    time::common_conditions::on_timer,
    window::{PrimaryWindow, WindowMode, WindowResolution},
};
use rectangles::Stats;

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
    app.add_plugins(rectangles::RectanglesPlugin);
    app.add_systems(Startup, setup_cameras);
    app.add_systems(Startup, setup_ui);
    app.add_systems(Update, full_screen_toggle.run_if(pressed_f));
    app.add_systems(Update, update_stats.run_if(resource_changed::<Stats>));
    app.add_systems(Update, update_fps.run_if(on_timer(Duration::from_secs(1))));

    app.add_plugins(FrameTimeDiagnosticsPlugin);

    if cfg!(debug_assertions) {
        for schedule in MainScheduleOrder::default().labels {
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

#[derive(Component)]
struct StatsText;

fn setup_cameras(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn setup_ui(mut commands: Commands) {
    let text_style = (
        TextFont {
            font_size: 30.0,
            ..default()
        },
        TextColor(Srgba::hex("a96cff").unwrap().into()),
    );

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                left: Val::Px(0.),
                padding: UiRect::all(Val::Px(5.)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.9)),
        ))
        .with_children(|parent| {
            parent
                .spawn((Text::default(), StatsText))
                .with_child((TextSpan::new("Count: "), text_style.clone()))
                .with_child((TextSpan::new(""), text_style.clone()))
                .with_child((TextSpan::new("\nFPS: "), text_style.clone()))
                .with_child((TextSpan::new("0.00"), text_style));
        });
}

fn pressed_f(keyboard_input: Res<ButtonInput<KeyCode>>) -> bool {
    keyboard_input.just_released(KeyCode::KeyF)
}

fn full_screen_toggle(mut window: Query<&mut Window, With<PrimaryWindow>>) {
    let Ok(mut window) = window.get_single_mut() else {
        return;
    };

    window.mode = if window.mode == WindowMode::Windowed {
        WindowMode::BorderlessFullscreen(MonitorSelection::Current)
    } else {
        WindowMode::Windowed
    };
}

fn update_stats(
    stats: Res<Stats>,
    query: Query<Entity, With<StatsText>>,
    mut writer: TextUiWriter,
) {
    let mut text = writer.text(query.single(), 2);
    text.clear();
    write!(text, "{}", stats.count).unwrap();
}

fn update_fps(
    diagnostics: Res<DiagnosticsStore>,
    query: Query<Entity, With<StatsText>>,
    mut writer: TextUiWriter,
) {
    if let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(Diagnostic::smoothed)
    {
        let mut text = writer.text(query.single(), 4);
        text.clear();
        write!(text, "{fps:.2}").unwrap();
    }
}
