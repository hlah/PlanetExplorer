#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct PlanetMaterial {
    max_altitude: f32;
    min_altitude: f32;
    color_mode: u32;
};

[[group(1), binding(0)]]
var<uniform> material: PlanetMaterial;

[[group(2), binding(0)]]
var<uniform> mesh: Mesh;

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] altitude: f32;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

let BLUE = vec3<f32>(0.0, 0.0, 0.4);
let CYAN = vec3<f32>(0.0, 0.8, 0.8);
let GREEN = vec3<f32>(0.0, 0.9, 0.0);
let YELLOW = vec3<f32>(0.8, 0.8, 0.0);
let RED = vec3<f32>(0.9, 0.0, 0.0);
let BROWN = vec3<f32>(0.63, 0.32, 0.18);
let WHITE = vec3<f32>(0.9, 0.9, 0.9);

fn get_altitude_color(normalized_altitude: f32) -> vec3<f32> {
    if (normalized_altitude < 0.15) {
        return mix(BLUE, CYAN, normalized_altitude / 0.15);
    } else if (normalized_altitude < 0.3) {
        return mix(CYAN, GREEN, (normalized_altitude - 0.15) / 0.15);
    } else if (normalized_altitude < 0.45) {
        return mix(GREEN, YELLOW, (normalized_altitude - 0.3) / 0.15);
    } else if (normalized_altitude < 0.6) {
        return mix(YELLOW, RED, (normalized_altitude - 0.45) / 0.15);
    } else if (normalized_altitude < 0.8) {
        return mix(RED, BROWN, (normalized_altitude - 0.6) / 0.2);
    } else {
        return mix(BROWN, WHITE, (normalized_altitude - 0.8) / 0.2);
    }
}

fn get_color(vertex: Vertex, world_position: vec4<f32>) -> vec3<f32> {
    switch(material.color_mode) {
        case 1: {
            let world_normal = mat3x3<f32>(
                mesh.inverse_transpose_model[0].xyz,
                mesh.inverse_transpose_model[1].xyz,
                mesh.inverse_transpose_model[2].xyz
            ) * vertex.normal;
            return (world_normal + 1.0) / 2.0;
        }
        case 2: {
            let normalized_position = normalize(vertex.position);
            let inclination = 1.0 - dot(normalized_position, vertex.normal);
            return vec3<f32>(0.8, 0.0, inclination);
        }
        case 3: {
            let normalized_altitude = (vertex.altitude - material.min_altitude) 
                / (material.max_altitude - material.min_altitude);
            return get_altitude_color(normalized_altitude);
        }
        default: {
            return vec3<f32>(0.0);
        }
    }
}

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);
    let normalized_position = normalize(vertex.position);

    var out: VertexOutput;
    out.clip_position = view.view_proj * world_position;
    out.color = get_color(vertex, world_position);

    return out;
}





struct FragmentInput {
    [[location(0)]] color: vec3<f32>;
};

[[stage(fragment)]]
fn fragment(in: FragmentInput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
