pub struct MaterialMesh {
    pub index_stride: u32,
    pub index_data: Vec<u8>,
    pub vertex_stride: u32,
    pub vertex_data: Vec<u8>
}

pub struct LOD {
    pub screen_size: f32, //values closer to 1 result in a sooner LOD change, values closer to 0 will result in a wider viewing range
    pub meshes: Vec<MaterialMesh> //material index in order of vec indices
}

pub struct AABB {
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
    pub min_z: f32,
    pub max_z: f32
}

impl AABB {
    pub fn fit_to_max(&mut self, other: &AABB) {
        self.min_x = f32::min(self.min_x, other.min_x);
        self.max_x = f32::max(self.max_x, other.max_x);
        self.min_y = f32::min(self.min_y, other.min_y);
        self.max_y = f32::max(self.max_y, other.max_y);
        self.min_z = f32::min(self.min_z, other.min_z);
        self.max_z = f32::max(self.max_z, other.max_z);
    }
}

pub struct ModelData {
    pub model_name: String,
    pub bounds: AABB,
    pub lods: Vec<LOD>
}