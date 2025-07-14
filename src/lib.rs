use bevy::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn bevy_init() {
    let mut app = bevy::app::App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            // provide the ID selector string here
            canvas: Some("#snake_bevy_canvas".into()),
            resolution:
                bevy::window::WindowResolution::new(1000., 1000.).with_scale_factor_override(1.0),
            // ... any other window properties ...
            ..default()
        }),
        ..default()
    }));

    app.add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                sprite_movement,
                handle_jump.run_if(bevy::input::common_conditions::input_just_pressed(
                    KeyCode::Space,
                )),
            ),
        )
        .run();
}

#[derive(Component)]
struct FoodPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
enum SnakeDirection {
    Up,
    Down,
    Left,
    Right,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    let color = Color::hsl(300., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
        Transform::from_xyz(0., 0., 0.),
        MeshMaterial2d(materials.add(color)),
        SnakeDirection::Down,
    ));

    let color = Color::hsl(2., 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.0))),
        MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., 100.0, 0.0),
        FoodPosition { x: 50, y: 50 },
    ));
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(
    time: Res<Time>,
    mut sprite_position: Query<(&mut SnakeDirection, &mut Transform)>,
) {
    for (mut direction, mut transform) in &mut sprite_position {
        transform.translation.x += direction.x * 50. * time.delta_secs();
        transform.translation.y += direction.y * 50. * time.delta_secs();

        if transform.translation.x > 500. || transform.translation.x < -500. {
            direction.x *= -1.;
        }
        if transform.translation.y > 500. || transform.translation.y < -500. {
            direction.y *= -1.;
        }
    }
}

fn handle_jump(mut sprite_position: Query<(&mut SnakeDirection, &mut Transform)>) {
    for (mut direction, mut _transform) in &mut sprite_position {
        direction.x *= -1.;
    }
}
