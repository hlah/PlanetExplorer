use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};

#[derive(Component)]
pub struct Planet {
    radius: f32,
}

impl Planet {
    pub fn new(radius: f32) -> Self {
        Self { radius }
    }
}

pub fn planet_added_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    planets: Query<(Entity, &Planet), Added<Planet>>,
) {
    for (entity, planet) in planets.iter() {
        build_planet(&mut commands, &mut meshes, &mut materials, entity, planet);
    }
}

fn build_planet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    entity: Entity,
    planet: &Planet,
) {
    info!("Building planet!!!");

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, initial_vertices());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, initial_vertices());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; 12]);
    mesh.set_indices(Some(Indices::U32(initial_triangles())));

    commands.entity(entity).insert_bundle(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(StandardMaterial {
            base_color: Color::RED,
            reflectance: 0.0,
            metallic: 0.0,
            perceptual_roughness: 0.5,
            ..default()
        }),
        ..default()
    });
}

fn initial_vertices() -> Vec<[f32; 3]> {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

    #[rustfmt::skip]
    let vertices = vec![
        Vec3::new(-1.0,  phi,  0.0).normalize().into(),
        Vec3::new( 1.0,  phi,  0.0).normalize().into(),
        Vec3::new(-1.0, -phi,  0.0).normalize().into(),
        Vec3::new( 1.0, -phi,  0.0).normalize().into(),
        Vec3::new( 0.0, -1.0,  phi).normalize().into(),
        Vec3::new( 0.0,  1.0,  phi).normalize().into(),
        Vec3::new( 0.0, -1.0, -phi).normalize().into(),
        Vec3::new( 0.0,  1.0, -phi).normalize().into(),
        Vec3::new( phi,  0.0, -1.0).normalize().into(),
        Vec3::new( phi,  0.0,  1.0).normalize().into(),
        Vec3::new(-phi,  0.0, -1.0).normalize().into(),
        Vec3::new(-phi,  0.0,  1.0).normalize().into(),
    ];
    vertices
}

fn initial_triangles() -> Vec<u32> {
    #[rustfmt::skip]
    let triangles = vec![
         0, 11,  5,
         0,  5,  1,
         0,  1,  7,
         0,  7, 10,
         0, 10, 11,
         1,  5,  9,
         5, 11,  4,
        11, 10,  2,
        10,  7,  6,
         7,  1,  8,
         3,  9,  4,
         3,  4,  2,
         3,  2,  6,
         3,  6,  8,
         3,  8,  9,
         4,  9,  5,
         2,  4, 11,
         6,  2, 10,
         8,  6,  7,
         9,  8,  1,
    ];
    triangles
}
