use std::{fs, ptr};

use gltf;

pub struct MaterialMesh {
    index_stride: u32,
    index_data: Vec<u8>,
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

//count, stride
fn get_vertex_sizing(attributes: gltf::mesh::iter::Attributes) -> (u32, u32, Vec<u32>) {
    let mut vertex_count: u32 = 0;
    let mut vertex_stride: u32 = 0;
    let mut offsets: Vec<u32> = Vec::new();
    for attribute in attributes {
        offsets.push(vertex_stride);
        vertex_stride += attribute.1.data_type().size() as u32;
        vertex_count = std::cmp::max(vertex_count, attribute.1.count() as u32);
    }
    
    (vertex_count, vertex_stride, offsets)
}

pub fn process_glb(file_dir: &std::path::PathBuf) -> Option<ModelData> {
    //process gltf
    let gltf_data = match gltf::import(file_dir) {
        Ok(v) => v,
        Err(_err) => return None
    };

    //get buffer data pointer
    let data_ptr = gltf_data.1[0].as_ptr();

    //iterate scenes
    for scene in gltf_data.0.scenes() {
        //use scene name as model name
        let model_name: String = match scene.name() {
            Some(v) => v,
            None => "Untitled"
        }.to_string();

        //print scene name
        println!("Input processing: {}", model_name);

        if model_name == "Scene" {
            panic!("Please dont use default scene names");
        }
    
        //iterate nodes (LODs in this context)
        for node in gltf_data.0.nodes() {
            //verify node is mesh
            let mesh = match node.mesh() {
                Some(v) => v,
                None => {
                    println!("  Skipping node that isnt mesh");
                    continue;
                }
            };
            
            //print node name
            match node.name() {
                Some(v) => println!("  Processing node: {}", v),
                None => println!("  Processing LOD: Untitled")
            }

            //create list of meshes
            let mut meshes: Vec<MaterialMesh> = Vec::new();
            
            //iterate primitives
            for primitive in mesh.primitives() {
                //get material name
                let mat_name = match primitive.material().name() {
                    Some(v) => v,
                    None => "Untitled"
                };

                //print material name
                println!("    Processing LOD mesh on material: {}", mat_name);

                //get index accessor
                let indices_access = match primitive.indices() {
                    Some(v) => v,
                    None => {
                        println!("  Skipping primitive with no index data accessor");
                        continue;
                    }
                };

                //get buffer view
                let indices_view = match indices_access.view() {
                    Some(v) => v,
                    None => {
                        println!("  Skipping primitive with no index data view");
                        continue;
                    }
                };

                //get index data
                let index_stride = indices_access.data_type().size();
                let index_count = indices_access.count();
                let indices_buffer_offset = indices_view.offset();

                //create index buffer
                let mut index_buffer: Vec<u8> = vec![0; index_count * index_stride];
                let a = index_buffer.as_ptr();

                //copy data into index buffer
                unsafe { ptr::copy_nonoverlapping(data_ptr.offset(indices_buffer_offset as isize), index_buffer.as_mut_ptr(), index_buffer.len()) };

                //initialize vertex buffer and sizes
                let (vertex_count, vertex_stride, offsets) = get_vertex_sizing(primitive.attributes());
                let mut vertex_buffer: Vec<u8> = vec![0; (vertex_count * vertex_stride) as usize];

                //get attributes
                for index in 0..vertex_count - 1 {
                    //probaby time to commit this stuff
                }

                //push back mesh
                meshes.push(MaterialMesh {
                    index_stride: index_stride as u32,
                    index_data: index_buffer,
                    vertex_stride: vertex_stride,
                    vertex_data: vertex_buffer
                });
            }
        }
    }

    //iterate meshes (LODs in this context)
    for mesh in gltf_data.0.meshes() {
        

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
    }

    None
}