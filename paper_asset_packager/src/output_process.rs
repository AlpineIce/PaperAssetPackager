use std::{fs, os::unix::fs::FileExt};
use super::common;

#[repr(C)]
struct MeshWriteData {
    vertex_stride: u32,
    index_stride: u32,
    vbo_offset: u64,
    ibo_offset: u64,
    vbo_size: u64,
    ibo_size: u64
}

#[repr(C)]
struct LODWriteData {
    screen_size: f32,
    mesh_count: u32,
    meshes_location: u64
}

#[repr(C)]
struct ModelWriteData {
    vb_location: u64,
    ib_location: u64,
    vb_size: u64,
    ib_size: u64,
    lod_count: u64,
    lods_location: u64,
    name_size: u64,
    aabb: common::AABB
}

fn write_failure(err: std::io::Error) {
    panic!("Failed to write generic model data: {}", err)
}

fn padded_size(to_pad: u64, alignment: u64) -> u64 {
    (to_pad + (alignment - 1)) & !(alignment - 1)
}

pub fn write_glb(file: &fs::File, mut model_data: common::ModelData, offset: &mut u64) {
    //model data location
    let model_data_location = *offset;
    *offset += std::mem::size_of::<ModelWriteData>() as u64;

    //write model name attribute directly after the model data so name is guaranteed to always be directly after
    let model_name_size = model_data.model_name.as_bytes().len() as u64;
    match file.write_at(model_data.model_name.as_bytes(), *offset) {
        Ok(_v) => {},
        Err(err) => write_failure(err)
    }
    *offset += padded_size(model_data.model_name.as_bytes().len() as u64, 8); //pad to 8 bytes

    //initialize VB and IB
    let mut vb: Vec<u8> = Vec::new();
    let mut ib: Vec<u8> = Vec::new();

    //allocate LOD space
    let lod_data_location = *offset;
    let lod_data_size = model_data.lods.len() * std::mem::size_of::<LODWriteData>();
    *offset += lod_data_size as u64;

    //write LOD headers
    for lod_index in 0..model_data.lods.len() {
        //allocate mesh space
        let mesh_data_location = *offset;
        let mesh_data_size = model_data.lods[lod_index].meshes.len() * std::mem::size_of::<MeshWriteData>();
        *offset += mesh_data_size as u64;
        
        //iterate meshes
        for mesh_index in 0..model_data.lods[lod_index].meshes.len() {
            //get mesh data
            let mesh_write_data = MeshWriteData {
                vertex_stride: model_data.lods[lod_index].meshes[mesh_index].vertex_stride,
                index_stride: model_data.lods[lod_index].meshes[mesh_index].index_stride,
                vbo_offset: vb.len() as u64,
                ibo_offset: ib.len() as u64,
                vbo_size: model_data.lods[lod_index].meshes[mesh_index].vertex_data.len() as u64,
                ibo_size: model_data.lods[lod_index].meshes[mesh_index].index_data.len() as u64
            };

            //write mesh data
            match file.write_at(common::as_slice::<MeshWriteData>(&mesh_write_data), mesh_data_location + (std::mem::size_of::<MeshWriteData>() * mesh_index) as u64) {
                Ok(_v) => {},
                Err(err) => panic!("Failed to write header entries with error: {}", err)
            }

            //push vertex/index data
            vb.append(&mut model_data.lods[lod_index].meshes[mesh_index].vertex_data);
            ib.append(&mut model_data.lods[lod_index].meshes[mesh_index].index_data);
        }

        let lod_write_data = LODWriteData {
            screen_size: model_data.lods[lod_index].screen_size,
            mesh_count: model_data.lods[lod_index].meshes.len() as u32,
            meshes_location: *offset
        };

        //write LOD data
        match file.write_at(common::as_slice::<LODWriteData>(&lod_write_data), lod_data_location + (std::mem::size_of::<LODWriteData>() * lod_index) as u64) {
            Ok(_v) => {},
            Err(err) => panic!("Failed to write header entries with error: {}", err)
        }
    }

    //write VB and IB
    let vb_location = *offset;
    match file.write_at(&vb, *offset) {
        Ok(_v) => {},
        Err(err) => panic!("Failed to write header entries with error: {}", err)
    }
    *offset += vb.len() as u64;

    let ib_location = *offset;
    match file.write_at(&ib, *offset) {
        Ok(_v) => {},
        Err(err) => panic!("Failed to write header entries with error: {}", err)
    }
    *offset += ib.len() as u64;
    

    //gather mesh data
    let model_write_data = ModelWriteData {
        vb_location: vb_location,
        ib_location: ib_location,
        vb_size: vb.len() as u64,
        ib_size: ib.len() as u64,
        lod_count: model_data.lods.len() as u64,
        lods_location: lod_data_location,
        name_size: model_name_size,
        aabb: model_data.bounds
    };

    //write model data
    match file.write_at(common::as_slice::<ModelWriteData>(&model_write_data), model_data_location) {
        Ok(_v) => {},
        Err(err) => panic!("Failed to write header entries with error: {}", err)
    }

}