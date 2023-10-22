use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
    }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn spawn_player(mut commands: Commands, assets: Res<AssetServer>) {
    let player = (
        SceneBundle {
            scene: assets.load("models/Robozin.glb#Scene0"),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        Speed(2.5),
        Player,
        Name::new("Player"),
    );

    commands.spawn(player);
}

fn player_movement(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
) {
    for (mut player_transform, player_speed) in player_q.iter_mut() {
        let mut direction = Vec3::ZERO;
        direction.y = 0.0;

        if input.pressed(KeyCode::W) {
            direction.x += -1.0;
        }

        if input.pressed(KeyCode::S) {
            direction.x += 1.0;
        }

        if input.pressed(KeyCode::A) {
            direction.z += 1.0;
        }

        if input.pressed(KeyCode::D) {
            direction.z += -1.0;
        }

        let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
        player_transform.translation += movement;

        //rotate the player to the movement direction
        if direction.length_squared() > 0.0 {
            player_transform.look_to(direction, Vec3::Y)
        }
    }
}
