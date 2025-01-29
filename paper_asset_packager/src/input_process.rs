use std::fs;

use gltf;

pub struct MaterialMesh {
    indices: Vec<f32>,
    vertex_stride: u32,
    vertex_data: Vec<u8>
}

pub struct LOD {
    screen_size: f32, //values closer to 1 result in a sooner LOD change, values closer to 0 will result in a wider viewing range
    meshes: Vec<MaterialMesh> //material index in order of vec indices
    
}

pub struct AABB {
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
    min_z: f32,
    max_z: f32
}

pub struct ModelData {
    model_name: String,
    bounds: AABB,
    lods: Vec<LOD>
}

pub fn process_glb(file_dir: &std::path::PathBuf) -> Option<ModelData> {
    //process gltf
    let gltf_data = match gltf::import(file_dir) {
        Ok(v) => v,
        Err(_err) => return None
    };

    //iterate meshes (LODs in this context)
    for mesh in gltf_data.0.meshes() {
        //get mesh name
        let mesh_name = match mesh.name() {
            Some(v) => v,
            None => "Untitled"
        };

        //get primitive data
        for primitive in mesh.primitives() {
            let material_index = match primitive.material().index() {
                Some(v) => v,
                None => 0
            };
            let bounds = primitive.bounding_box();
            for attribute in primitive.attributes() {
                let data_type = attribute.1.data_type();
                let size = attribute.1.size();

                let a = match attribute.1.view() {
                    Some(v) => v,
                    None => continue
                };

            }

            let aabb = AABB {
                min_x: bounds.min[0],
                max_x: bounds.max[0],
                min_y: bounds.min[1],
                max_y: bounds.max[1],
                min_z: bounds.min[2],
                max_z: bounds.max[2]
            };
        }

        let a = 3;
    }

    Some(model_data)
}