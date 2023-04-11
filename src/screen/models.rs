
pub struct Models {
    player:u32,
}

pub fn load_models() -> Models {
    let gltf_box = gltf::Gltf::open("assets/models/cube_test.glb");
    Models {
        player: 0,
    }
}
