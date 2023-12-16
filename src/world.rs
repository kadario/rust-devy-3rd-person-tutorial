use bevy::prelude::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, (
      spawn_floor, 
      spawn_light,
      spawn_object
    ));
  }
}

fn spawn_light(mut commands: Commands) {
  let light = PointLightBundle {
      point_light: PointLight {
          intensity: 2000.0,
          ..default()
      },
      transform: Transform::from_xyz(0.0, 5.0, 0.0),
      ..default()
  };

  commands.spawn(light);
}

fn spawn_floor(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let floor = PbrBundle {
      mesh: meshes.add(Mesh::from(shape::Plane::from_size(15.))),
      material: materials.add(Color::DARK_GREEN.into()),
      ..default()
  };

  commands.spawn(floor);
}

fn spawn_object(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  let mut create_cube = |size: f32, color: Color, xyz: (f32, f32, f32), name: String| -> (PbrBundle, Name) {
    (
      PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube::new(size))),
        material: materials.add(color.into()),
        transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
        ..default()
      },
      Name::new(name)
    )
  };

  commands.spawn(create_cube(
    4.0, 
    Color::BLUE, 
    (-5.0, 2.0, 5.0), 
    "Blue Cube".to_string()
  ));

  commands.spawn(create_cube(
    2.0, 
    Color::RED, 
    (6.0, 1.0, -6.0), 
    "Red Cube".to_string()
  ));

}