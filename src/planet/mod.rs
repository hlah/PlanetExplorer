pub mod graphics;
pub mod height_map;
pub mod view;

use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use graphics::{PlanetMaterial, ATTRIBUTE_ALTITUDE};
use height_map::HeightMap;
use std::collections::HashMap;
use view::ColorMode;

#[derive(Component)]
pub struct Planet {
    radius: f32,
    min_altitude: f32,
    max_altitude: f32,
    lod_depth: u32,
    height_map: Handle<HeightMap>,
}

#[derive(Component)]
pub struct LoadingPlanet;

impl Planet {
    pub fn new(
        radius: f32,
        min_altitude: f32,
        max_altitude: f32,
        height_map: Handle<HeightMap>,
    ) -> Self {
        Self {
            radius,
            min_altitude,
            max_altitude,
            lod_depth: 11,
            height_map,
        }
    }
}

pub fn planet_added_system(mut commands: Commands, planets: Query<Entity, Added<Planet>>) {
    for entity in planets.iter() {
        commands.entity(entity).insert(LoadingPlanet);
    }
}

pub fn planet_loading_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
    height_maps: Res<Assets<HeightMap>>,
    color_mode: Res<ColorMode>,
    planets: Query<(Entity, &Planet), With<LoadingPlanet>>,
) {
    for (entity, planet) in planets.iter() {
        if let Some(height_map) = height_maps.get(planet.height_map.clone()) {
            build_planet(
                &mut commands,
                &mut meshes,
                &mut materials,
                entity,
                planet,
                &color_mode,
                height_map,
            );
            commands.entity(entity).remove::<LoadingPlanet>();
        }
    }
}

fn build_planet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<PlanetMaterial>,
    entity: Entity,
    planet: &Planet,
    color_mode: &ColorMode,
    height_map: &HeightMap,
) {
    info!("Building planet!!!");

    let (vertices, normals, altitudes, indices) = build_vertices(planet, height_map);

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 0.0]; vertices.len()]);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(ATTRIBUTE_ALTITUDE, altitudes);
    mesh.set_indices(Some(Indices::U32(indices)));

    commands.entity(entity).insert_bundle(MaterialMeshBundle {
        mesh: meshes.add(mesh),
        material: materials.add(PlanetMaterial {
            min_altitude: planet.min_altitude * height_map::HEIGHT_SCALLING,
            max_altitude: planet.max_altitude * height_map::HEIGHT_SCALLING,
            color_mode: color_mode.clone() as u32,
        }),
        ..default()
    });

    info!("Building planet: done");
}

fn build_vertices(
    planet: &Planet,
    height_map: &HeightMap,
) -> (Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<f32>, Vec<u32>) {
    let mut vertices = initial_vertices();
    let mut triangles = initial_triangles();

    for _ in 1..planet.lod_depth {
        let mut mid_vertices = HashMap::<(u32, u32), u32>::new();
        let mut new_triangles = vec![];

        for triangle in triangles {
            let m1 = get_middle_vertex(&mut mid_vertices, &mut vertices, triangle[0], triangle[1]);
            let m2 = get_middle_vertex(&mut mid_vertices, &mut vertices, triangle[0], triangle[2]);
            let m3 = get_middle_vertex(&mut mid_vertices, &mut vertices, triangle[1], triangle[2]);

            new_triangles.push([triangle[0], m1, m2]);
            new_triangles.push([triangle[1], m3, m1]);
            new_triangles.push([triangle[2], m2, m3]);
            new_triangles.push([m1, m3, m2]);
        }

        triangles = new_triangles;
    }

    let indices = triangles.into_iter().flatten().collect();
    let mut normals = vec![];
    let mut vertices_with_height = vec![];
    let mut altitudes = vec![];
    for vertice in vertices {
        let (height, normal) = height_map.fetch_relief_at(vertice, planet.radius);
        normals.push(normal.into());
        vertices_with_height.push((vertice * (planet.radius + height)).into());
        altitudes.push(height);
    }

    (vertices_with_height, normals, altitudes, indices)
}

fn get_middle_vertex(
    mid_vertices: &mut HashMap<(u32, u32), u32>,
    vertices: &mut Vec<Vec3>,
    a: u32,
    b: u32,
) -> u32 {
    if let Some(index) = mid_vertices.get(&(a, b)) {
        *index
    } else if let Some(index) = mid_vertices.get(&(b, a)) {
        *index
    } else {
        let index = vertices.len() as u32;
        let middle_vertice = (vertices[a as usize] + vertices[b as usize]).normalize();
        vertices.push(middle_vertice);
        mid_vertices.insert((a, b), index);
        index
    }
}

fn initial_vertices() -> Vec<Vec3> {
    let phi = (1.0 + 5.0_f32.sqrt()) / 2.0;

    #[rustfmt::skip]
    let vertices = vec![
        Vec3::new(-1.0,  phi,  0.0).normalize(),
        Vec3::new( 1.0,  phi,  0.0).normalize(),
        Vec3::new(-1.0, -phi,  0.0).normalize(),
        Vec3::new( 1.0, -phi,  0.0).normalize(),
        Vec3::new( 0.0, -1.0,  phi).normalize(),
        Vec3::new( 0.0,  1.0,  phi).normalize(),
        Vec3::new( 0.0, -1.0, -phi).normalize(),
        Vec3::new( 0.0,  1.0, -phi).normalize(),
        Vec3::new( phi,  0.0, -1.0).normalize(),
        Vec3::new( phi,  0.0,  1.0).normalize(),
        Vec3::new(-phi,  0.0, -1.0).normalize(),
        Vec3::new(-phi,  0.0,  1.0).normalize(),
    ];
    vertices
}

fn initial_triangles() -> Vec<[u32; 3]> {
    #[rustfmt::skip]
    let triangles = vec![
        [ 0, 11,  5],
        [ 0,  5,  1],
        [ 0,  1,  7],
        [ 0,  7, 10],
        [ 0, 10, 11],
        [ 1,  5,  9],
        [ 5, 11,  4],
        [11, 10,  2],
        [10,  7,  6],
        [ 7,  1,  8],
        [ 3,  9,  4],
        [ 3,  4,  2],
        [ 3,  2,  6],
        [ 3,  6,  8],
        [ 3,  8,  9],
        [ 4,  9,  5],
        [ 2,  4, 11],
        [ 6,  2, 10],
        [ 8,  6,  7],
        [ 9,  8,  1],
    ];
    triangles
}
