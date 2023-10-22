use bevy::prelude::*;
use rand;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

#[derive(Component)]
struct Enemy;

fn spawn_enemy(mut commands: Commands, assets: Res<AssetServer>) {
    let enemy = (
        SceneBundle {
            scene: assets.load("models/Robozin.glb#Scene0"),
            transform: Transform::from_xyz(
                rand::random::<f32>() * (15.0 * rand::random::<f32>()),
                0.5,
                rand::random::<f32>() * (15.0 * rand::random::<f32>()),
            ),
            ..default()
        },
        Name::new("Enemy"),
    );

    commands.spawn(enemy);
}
