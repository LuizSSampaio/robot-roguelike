use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_floor, spawn_ambient_light));
    }
}

fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let floor = (
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane::from_size(30.0))),
            material: materials.add(Color::rgb(0.18, 0.0, 0.0).into()),
            ..default()
        },
        Name::new("Floor"),
    );

    commands.spawn(floor);
}

fn spawn_ambient_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb(1.0, 1.0, 1.0),
        brightness: 1.0,
    });
}
