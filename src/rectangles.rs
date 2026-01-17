use std::cmp::max;

use bevy::{
    prelude::*,
    window::{PrimaryWindow, WindowResized},
};
use rand::{
    Rng, SeedableRng,
    distr::{Distribution, Uniform},
};
use rand_xoshiro::Xoshiro256PlusPlus;

pub struct RectanglesPlugin;

impl Plugin for RectanglesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Stats>();
        app.init_resource::<PseudoRng>();
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (bounds_updater, movement, collision_detection).chain(),
        );
        app.add_systems(Update, mouse_handler.ambiguous_with(bounds_updater));
    }
}

#[derive(Resource)]
pub struct Stats {
    pub count: u32,
}

impl Default for Stats {
    fn default() -> Self {
        Self { count: 250 }
    }
}

#[derive(Resource)]
pub struct PseudoRng(Xoshiro256PlusPlus);

impl Default for PseudoRng {
    fn default() -> Self {
        Self(Xoshiro256PlusPlus::seed_from_u64(395_992_934_456_271))
    }
}

#[derive(Component, Debug)]
pub struct RectangleObject {
    velocity: f32,
    width: f32,
    teleport_target: f32,
}

fn setup(
    mut commands: Commands,
    window: Query<&Window, With<PrimaryWindow>>,
    stats: Res<Stats>,
    mut rng: ResMut<PseudoRng>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    spawn_rectangles(&mut commands, window, &mut rng.0, stats.count);
}

pub fn mouse_handler(
    mut commands: Commands,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    window: Query<&Window, With<PrimaryWindow>>,
    mut stats: ResMut<Stats>,
    rectangles: Query<Entity, With<RectangleObject>>,
    mut rng: ResMut<PseudoRng>,
) {
    let Ok(window) = window.single() else {
        return;
    };

    let old = stats.count;
    if mouse_button_input.just_released(MouseButton::Left) {
        stats.count = max(1, stats.count * 2);
        spawn_rectangles(&mut commands, window, &mut rng.0, stats.count - old);
    }
    if mouse_button_input.just_released(MouseButton::Right) {
        stats.count /= 2;
        despawn_rectangles(&mut commands, rectangles, old - stats.count);
    }
}

fn spawn_rectangles(
    commands: &mut Commands,
    window: &Window,
    rng: &mut Xoshiro256PlusPlus,
    num: u32,
) {
    let (width, height) = (window.width(), window.height());
    let teleport_target = -(width / 2.);

    for _ in 0..num {
        let dimensions = Vec2::splat(rng.random::<f32>().mul_add(40., 10.));
        commands
            .spawn((
                RectangleObject {
                    velocity: rng.random_range(60.0..120.0),
                    width: dimensions.x,
                    teleport_target: teleport_target - dimensions.x,
                },
                Sprite {
                    color: Color::BLACK,
                    custom_size: Some(dimensions),
                    ..default()
                },
                Transform::from_xyz(
                    (rng.random::<f32>() - 0.5) * width,
                    (rng.random::<f32>() - 0.5) * height,
                    rng.random::<f32>(),
                ),
            ))
            .with_children(|children| {
                children.spawn((
                    Sprite {
                        color: Color::WHITE,
                        custom_size: Some(dimensions - Vec2::splat(3.)),
                        ..default()
                    },
                    Transform::from_xyz(0., 0., f32::EPSILON),
                ));
            });
    }
}

fn despawn_rectangles(
    commands: &mut Commands,
    rectangles: Query<Entity, With<RectangleObject>>,
    num: u32,
) {
    for r in rectangles.iter().take(num as usize) {
        commands.entity(r).despawn();
    }
}

pub fn bounds_updater(
    window: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut resize_event: MessageReader<WindowResized>,
    mut rectangles_query: Query<(&mut RectangleObject, &mut Transform)>,
    mut rng: ResMut<PseudoRng>,
    mut prev_window_height: Local<f32>,
) {
    let Ok((window_id, window)) = window.single() else {
        return;
    };
    if *prev_window_height == 0. {
        *prev_window_height = window.height();
    }
    let Some(e) = resize_event.read().filter(|e| e.window == window_id).last() else {
        return;
    };

    let teleport_target = -(e.width / 2.);
    let rng = &mut rng.0;
    let height_bound = e.height / 2.;
    let respawn_probability = 1. - (*prev_window_height / e.height);
    let target = if respawn_probability > 0. {
        let prev_height_bound = *prev_window_height / 2.;
        Some(Uniform::new(prev_height_bound, height_bound).unwrap())
    } else {
        None
    };
    for (mut r, mut t) in &mut rectangles_query {
        r.teleport_target = teleport_target - r.width;
        if let Some(target) = target {
            if rng.random::<f32>() < respawn_probability {
                t.translation.y = target.sample(rng) * if rng.random_bool(0.5) { 1. } else { -1. };
            }
        } else if t.translation.y <= -height_bound || height_bound <= t.translation.y {
            t.translation.y = (rng.random::<f32>() - 0.5) * e.height;
        }
    }
    *prev_window_height = e.height;
}

fn movement(time: Res<Time>, mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query
        .par_iter_mut()
        .for_each(|(r, mut transform)| {
            transform.translation.x -= r.velocity * time.delta_secs();
        });
}

fn collision_detection(mut rectangles_query: Query<(&RectangleObject, &mut Transform)>) {
    rectangles_query
        .par_iter_mut()
        .for_each(|(r, mut transform)| {
            if transform.translation.x < r.teleport_target {
                transform.translation.x *= -1.;
            }
        });
}
