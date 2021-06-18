use wgpu::{Device, Buffer, BufferUsage, VertexBufferLayout, BufferAddress, Queue, RenderPass, IndexFormat};
use crate::vertex::chunk_vertex::ChunkVertex;
use wgpu::util::{BufferInitDescriptor, DeviceExt};
use crate::vertex::chunk_vertex_buffer_data::ChunkVertexBufferData;

pub struct ChunkVertexBuffer{
    pub(crate) vertecies: Vec<ChunkVertex>,
    pub indecies: Vec<u16>,
    pub(crate) buffer: Buffer,
    pub index_buffer: Buffer
}

impl ChunkVertexBuffer {
    pub fn new_from_data(device:&Device,data:ChunkVertexBufferData) -> ChunkVertexBuffer{
        return ChunkVertexBuffer::new(device,data.vertecies,data.indecies,false);
    }

    pub fn new(device:&Device,vertecies: Vec<ChunkVertex>,indecies:Vec<u16>,edit:bool) -> ChunkVertexBuffer {

        let mut bytes : Vec<u8> = vec![];

        for v in vertecies.iter() {
            bytes.extend_from_slice(v.x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.z.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.u.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.v.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_x.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_y.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.n_z.to_le_bytes().as_ref());
            bytes.extend_from_slice(v.light_level.to_le_bytes().as_ref());
        }

        let buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: &*bytes,
            usage: if !edit {BufferUsage::VERTEX} else {BufferUsage::VERTEX | BufferUsage::COPY_DST}
        });

        let mut bytes_index : Vec<u8> = vec![];

        for v in indecies.iter() {
            bytes_index.extend_from_slice(v.to_le_bytes().as_ref());
        }

        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: &*bytes_index,
            usage: BufferUsage::INDEX
        });

        return ChunkVertexBuffer { vertecies, indecies, buffer, index_buffer };
    }

    pub fn desc() -> VertexBufferLayout<'static>{
        wgpu::VertexBufferLayout {
            array_stride: (9 * std::mem::size_of::<f32>()) as wgpu::BufferAddress, // 1.
            step_mode: wgpu::InputStepMode::Vertex, // 2.
            attributes: &[ // 3.
                wgpu::VertexAttribute {
                    offset: 0, // 4.
                    shader_location: 0, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;3]>() as BufferAddress, // 4.
                    shader_location: 1, // 5.
                    format: wgpu::VertexFormat::Float32x2, // 6.
                },wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;5]>() as BufferAddress, // 4.
                    shader_location: 2, // 5.
                    format: wgpu::VertexFormat::Float32x3, // 6.
                },wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32;8]>() as BufferAddress, // 4.
                    shader_location: 3, // 5.
                    format: wgpu::VertexFormat::Float32, // 6.
                }
            ]
        }
    }

    pub fn edit_data(&self,data: &Vec<f32>,queue:&Queue) {
        let mut bytes : Vec<u8> = vec![];

        for v in data.iter() {
            bytes.extend_from_slice(v.to_le_bytes().as_ref());
        }
        queue.write_buffer(&self.buffer,0,&*bytes);
    }

    pub fn render<'a>(&'a self, render_pass:&mut RenderPass<'a>) {
        render_pass.set_vertex_buffer(0,self.buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..),IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.indecies.len() as u32,0,0..1);
    }
}