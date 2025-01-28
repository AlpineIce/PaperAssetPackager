use std::fs;

use gltf;

pub struct ModelData {
    i: u32
}

pub fn process_glb(file_dir: &std::path::PathBuf) -> Option<ModelData> {
    //process gltf
    let gltf_data = match gltf::import(file_dir) {
        Ok(v) => v,
        Err(_err) => return None
    };

    for scene in gltf_data.0.scenes() {
        for node in scene.nodes() {
            node.
        }
    }

    let model_data = ModelData {
        i: 63
    };

    Some(model_data)
}