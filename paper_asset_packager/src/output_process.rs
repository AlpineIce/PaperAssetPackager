use std::fs;
use super::common;

struct ModelWriteData {
    vb_location: u64,
    ib_location: u64,
    name_location: u64,
    aabb_location: u64,
    lods_location: u64,
    lod_count: u64 //this would never be 64 bits but it looks nice
}

//impl ModelWriteData {
    
    pub fn as_slice<T>(data: &T) -> &[u8; std::mem::size_of::<T>()] {
        unsafe {
            &*(data as *const T as *const [u8; std::mem::size_of::<T>()])
        }
    }
//}

pub fn write_glb(file: &fs::File, model_data: common::ModelData, offset: u64) -> u64 {


    //file.write_at(buf, offset)
    0
}