#[derive(Clone)]
pub struct ChunkVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub u: f32,
    pub v: f32,
    pub n_x: f32,
    pub n_y: f32,
    pub n_z: f32,
    pub light_level: f32
}

impl ChunkVertex {
    pub fn new(x:f32,y:f32,z:f32,u:f32,v:f32,n_x: f32,n_y: f32,n_z: f32,light_level:f32) -> ChunkVertex {
        return ChunkVertex {
            x,
            y,
            z,
            u,
            v,
            n_x,
            n_y,
            n_z,
            light_level
        };
    }
}

