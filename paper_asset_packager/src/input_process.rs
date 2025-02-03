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

impl AABB {
    fn fit_to_max(&mut self, other: &AABB) {
        self.min_x = f32::max(self.min_x, other.min_x);
        self.max_x = f32::max(self.max_x, other.max_x);
        self.min_y = f32::max(self.min_y, other.min_y);
        self.max_y = f32::max(self.max_y, other.max_y);
        self.min_z = f32::max(self.min_z, other.min_z);
        self.max_z = f32::max(self.max_z, other.max_z);
    }
}

pub struct ModelData {
    model_name: String,
    bounds: AABB,
    lods: Vec<LOD>
}

struct VertexSizing {
    count: usize,
    stride: usize,
    offsets: Vec<usize>,
    attribute_sizes: Vec<usize>,
    pointers: Vec<*const u8>
}

impl VertexSizing {
    fn get_bytes_size(&self) -> usize {
        self.count * self.stride
    }

    fn get_indexed_attribute_location(&self, vertex: usize, attribute: usize) -> isize {
        ((vertex * self.stride) + self.offsets[attribute]) as isize
    }
    
    fn get_location_ptr(&self, vertex: usize, attribute: usize) -> *const u8 {
        unsafe { self.pointers[attribute].offset(self.get_indexed_attribute_location(vertex, attribute)) }
    }
}

//count, stride
fn get_vertex_sizing(attributes: gltf::mesh::iter::Attributes, data_ptr: *const u8) -> Option<VertexSizing> {
    let mut vertex_count: usize = 0;
    let mut vertex_stride: usize = 0;
    let mut offsets: Vec<usize> = Vec::new();
    let mut sizes: Vec<usize> = Vec::new();
    let mut data_pointers: Vec<*const u8> = Vec::new();
    for attribute in attributes {
        //get buffer view
        let view = match attribute.1.view() {
            Some(v) => v,
            None => return None
        };
        
        //get vertex data offsets
        offsets.push(vertex_stride);
        sizes.push(attribute.1.data_type().size());
        data_pointers.push(unsafe { data_ptr.offset(view.offset() as isize) });
        vertex_stride += attribute.1.data_type().size();
        vertex_count = std::cmp::max(vertex_count, attribute.1.count()); //not very elegant, -1000 aura
    }
    
    Some(VertexSizing {
        count: vertex_count,
        stride: vertex_stride,
        offsets: offsets,
        attribute_sizes: sizes,
        pointers: data_pointers
    })
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

        //initialize AABB
        let mut aabb = AABB {
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
            min_z: 0.0,
            max_z: 0.0
        };
        
        //initialize LOD data vec
        let mut lods: Vec<LOD> = Vec::new();
    
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
                
                //AABB processing
                let mesh_bounds = primitive.bounding_box();
                let mesh_aabb = AABB {
                    min_x: mesh_bounds.min[0],
                    max_x: mesh_bounds.max[0],
                    min_y: mesh_bounds.min[1],
                    max_y: mesh_bounds.max[1],
                    min_z: mesh_bounds.min[2],
                    max_z: mesh_bounds.max[2]
                };
                aabb.fit_to_max(&mesh_aabb);

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

                //copy data into index buffer
                unsafe { ptr::copy_nonoverlapping(data_ptr.offset(indices_buffer_offset as isize), index_buffer.as_mut_ptr(), index_buffer.len()) };

                //initialize vertex buffer and sizes
                let vertex_sizing = match get_vertex_sizing(primitive.attributes(), data_ptr) {
                    Some(v) => v,
                    None => {
                        println!("  Skipping primitive with no vertex data view");
                        continue;
                    }
                };
                let mut vertex_buffer: Vec<u8> = vec![0; vertex_sizing.get_bytes_size()];

                //let (a, b) = (index_buffer.as_ptr(), vertex_buffer.as_ptr()); //debug helper line
                
                //get attributes
                for i in 0..vertex_sizing.count {
                    for j in 0..vertex_sizing.offsets.len() {
                        unsafe {
                            ptr::copy_nonoverlapping(
                                vertex_sizing.get_location_ptr(i, j),
                                vertex_buffer.as_mut_ptr().offset(vertex_sizing.get_indexed_attribute_location(i, j)),
                                vertex_sizing.attribute_sizes[j]
                            );
                        }
                    }
                }

                //push back mesh
                meshes.push(MaterialMesh {
                    index_stride: index_stride as u32,
                    index_data: index_buffer,
                    vertex_stride: vertex_sizing.stride as u32,
                    vertex_data: vertex_buffer
                });
            }

            //add meshes to new LOD
            lods.push( LOD {
                screen_size: 1.0, //TODO
                meshes: meshes
            });
        }

        return Some(ModelData {
            model_name: model_name,
            bounds: aabb,
            lods: lods
        })
    }

    None
}