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

    app
        .add_systems(Startup, setup)
        .add_systems(Update, sprite_movement)
.run();
}


#[derive(Component)]
struct Direction (f32,f32);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>,    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,) {
    commands.spawn(Camera2d);

    commands.spawn((
        Sprite::from_image(asset_server.load("icon.png")),
        Transform::from_xyz(0., 0., 0.),
        Direction(1.,1.),
    ));

    let shapes = [
        meshes.add(Circle::new(50.0)),
        meshes.add(CircularSector::new(50.0, 1.0)),
        meshes.add(CircularSegment::new(50.0, 1.25)),
        meshes.add(Ellipse::new(25.0, 50.0)),
        meshes.add(Annulus::new(25.0, 50.0)),
        meshes.add(Capsule2d::new(25.0, 50.0)),
        meshes.add(Rhombus::new(75.0, 100.0)),
        meshes.add(Rectangle::new(50.0, 100.0)),
        meshes.add(RegularPolygon::new(50.0, 6)),
        meshes.add(Triangle2d::new(
            Vec2::Y * 50.0,
            Vec2::new(-50.0, -50.0),
            Vec2::new(50.0, -50.0),
        )),
    ];
    let num_shapes = shapes.len();

    for (i, shape) in shapes.into_iter().enumerate() {
        // Distribute colors evenly across the rainbow.
        let color = Color::hsl(360. * i as f32 / num_shapes as f32, 0.95, 0.7);

        const X_EXTENT: f32 = 900.;
        commands.spawn((
            Mesh2d(shape),
            MeshMaterial2d(materials.add(color)),
            Transform::from_xyz(
                // Distribute shapes from -X_EXTENT/2 to +X_EXTENT/2.
                -X_EXTENT / 2. + i as f32 / (num_shapes - 1) as f32 * X_EXTENT,
                100.0,
                0.0,
            ),
            Direction(1.,0.5),
        ));
    }
}

/// The sprite is animated by changing its translation depending on the time that has passed since
/// the last frame.
fn sprite_movement(time: Res<Time>, mut sprite_position: Query<(&mut Direction, &mut Transform)>) {
    for (mut direction, mut transform) in &mut sprite_position {
        
            transform.translation.x += direction.0 * 150. * time.delta_secs();
            transform.translation.y += direction.1 * 150. * time.delta_secs();
        

        if transform.translation.x > 500. || transform.translation.x < -500.{
            direction.0 *= -1.; 
        }
        if transform.translation.y > 500. || transform.translation.y < -500.{
            direction.1 *= -1.; 
        }
    }
}