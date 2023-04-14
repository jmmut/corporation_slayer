use crate::common::AnyError;
use gltf::Semantic;
use macroquad::models::Vertex;
use macroquad::prelude::{Mesh, Vec2, Vec3, BLACK};
use std::iter::Map;

pub struct Models {
    pub player: Model,
}

pub type Model = Mesh;

pub fn load_models() -> Result<Models, AnyError> {
    let path = "assets/models/cube_test.glb";
    let mut meshes = gltf_to_meshes(path)?;
    assert!(
        meshes.len() >= 1,
        "expected 1 or more meshes loaded from {}",
        path
    );
    Ok(Models {
        player: meshes.remove(0),
    })
}

fn gltf_to_meshes(path: &str) -> Result<Vec<Mesh>, AnyError> {
    return Ok(Vec::new());
    // let (document, buffers, _images) = gltf::import(path)?;
    // let mut meshes = Vec::<Mesh>::new();
    // for mesh in document.meshes() {
    //     for primitive in mesh.primitives() {
    //         let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
    //
    //         if let Some(_accessor) = primitive.get(&Semantic::Positions) {
    //             let positions: Vec<[f32; 3]> = reader
    //                 .read_positions()
    //                 .expect("Position primitives should be present")
    //                 .collect();
    //             let vertices = positions
    //                 .iter()
    //                 // .zip(normals.iter())
    //                 // .zip(tex_coords.into_f32().iter())
    //                 // .map(|((&[x, y, z], &[nx, ny, nz]), &[u, v])| Vertex::new(x, y, z, u, v, BLACK))
    //                 .map(|&[x, y, z]| Vertex {
    //                     position: Vec3::new(x, y, z),
    //                     uv: Vec2::new(0.0, 0.0),
    //                     color: BLACK,
    //                 })
    //                 .collect::<Vec<_>>();
    //
    //             let nested_iterator: Option<Map<_, _>> = reader
    //                 .read_indices()
    //                 .map(|indices| indices.into_u32().map(|i| i as u16));
    //             let indices = nested_iterator.expect("").collect::<Vec<_>>();
    //             let mesh = Mesh {
    //                 vertices,
    //                 indices,
    //                 texture: None,
    //             };
    //             meshes.push(mesh);
    //         }
    //     }
    // }
    // Ok(meshes)
}
