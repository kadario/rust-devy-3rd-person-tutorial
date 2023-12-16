use bevy::prelude::*;
use bevy_third_person_camera::*;

// const SPEED: f32 = 2.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, spawn_player)
      .add_systems(Update, player_movement);
  }
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Speed(f32);

fn player_movement(
  keys: Res<Input<KeyCode>>,
  time: Res<Time>,
  mut player_q: Query<(&mut Transform, &Speed), With<Player>>,
  cam_q: Query<&Transform, (With<Camera>, Without<Player>)>
) {
  for (mut player_transform , player_speed) in player_q.iter_mut() {
    let cam = match cam_q.get_single() {
      Ok(c) => c,
      Err(e) => Err(format!("Camera error {}", e)).unwrap()
    };

    let mut direction = Vec3::ZERO;

    // println!("player_speed {:09.3}", player_speed);

    // forward
    if keys.pressed(KeyCode::W) {
      direction += cam.forward();
    }

    // back
    if keys.pressed(KeyCode::S) {
      direction += cam.back();
    }

    // left
    if keys.pressed(KeyCode::A) {
      direction += cam.left();
    }

    // right
    if keys.pressed(KeyCode::D) {
      direction += cam.right();
    }

    direction.y = 0.;

    let movement = direction.normalize_or_zero() * player_speed.0 * time.delta_seconds();
    player_transform.translation += movement;

    // rotate player to direction he moving to
    if direction.length_squared() > 0.0 {
      player_transform.look_to(direction, Vec3::Y)
    }

  }
}

fn spawn_player(
  mut commands: Commands,
  assets: Res<AssetServer>,
  // mut meshes: ResMut<Assets<Mesh>>,
  // mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let flashlight = (SpotLightBundle {
    spot_light: SpotLight {
      color: Color::rgba(1.0, 1.0, 0.47, 1.0 ),
      range: 10.0,
      intensity: 4000.0,
      outer_angle: 0.5,
      inner_angle: 0.4,
      shadows_enabled: true,
      ..default()
    },
    transform: Transform::from_xyz(0.0, 0.25, -0.2),
    ..default()
  }, Name::new("Flashlight"));
  let player = (
    SceneBundle {
      scene: assets.load("Player.gltf#Scene0"),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
    },
    Speed(2.5),
    Player,
    ThirdPersonCameraTarget,
    Name::new("Player"),
  );

  commands.spawn(player).with_children(|parent| {
    parent.spawn(flashlight);
  });
}