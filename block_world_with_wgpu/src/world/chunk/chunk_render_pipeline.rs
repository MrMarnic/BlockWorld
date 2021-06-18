use rust_game_library_wgpu::pipeline::pipeline::{RenderPipelineGroup, RenderPipelineGroupBuilder};
use wgpu::{Buffer, SwapChainDescriptor, Device, BindGroupLayoutDescriptor, BindGroupLayoutEntry, ShaderStage, BindingType, BindingResource, BindGroupDescriptor, BindGroupEntry, PrimitiveTopology, BindGroupLayout, PipelineLayoutDescriptor, RenderPipelineDescriptor, FrontFace, ColorWrite, IndexFormat, TextureFormat, CompareFunction, BufferDescriptor, BufferUsage, BindGroup, VertexState, FragmentState, DepthStencilState, StencilState, DepthBiasState, PrimitiveState, PolygonMode, MultisampleState, ColorTargetState, BlendState, BlendComponent, VertexBufferLayout, BufferBindingType};
use rust_game_library_wgpu::objects::camera::Camera;
use rust_game_library_wgpu::objects::vertex_buffer::{VertexBuffer, NormalVertexBuffer};
use rust_game_library_wgpu::objects::depth_texture::DepthTexture;
use crate::vertex::chunk_vertex_buffer::ChunkVertexBuffer;

pub struct ChunkRenderPipelineGroup {
    pub group: RenderPipelineGroup,
    pub light_pos_buffer: Buffer,
    pub light_pos_group: BindGroup
}

impl ChunkRenderPipelineGroup {
    pub fn new(vertex_shader_path : String, fragment_shader_path: String,sc_desc:&SwapChainDescriptor,device:&Device) -> ChunkRenderPipelineGroup{

        let mut group2_builder = RenderPipelineGroupBuilder::empty();
        group2_builder.set_shaders(&device,vertex_shader_path,fragment_shader_path,"vertex_chunk".to_string(),"fragment_chunk".to_string());

        let texture_bind_group_layout = group2_builder.create_texture_bind_group_layout(device);
        group2_builder.bind_groups_layouts.push(texture_bind_group_layout);
        group2_builder.bind_groups_layouts.push(Camera::bind_group_layout(device));
        group2_builder.bind_groups_layouts.push(Camera::transform_bind_group(device));

        let light_pos_buffer = device.create_buffer(&BufferDescriptor {
            label: None,
            size: 512,
            usage: BufferUsage::UNIFORM | BufferUsage::COPY_DST,
            mapped_at_creation: false
        });

        let light_pos_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor { label: None, entries: &[BindGroupLayoutEntry {
            binding: 0,
            visibility: ShaderStage::FRAGMENT,
            ty: BindingType::Buffer { ty: BufferBindingType::Uniform, has_dynamic_offset: false, min_binding_size: None },
            count: None
        }] });

        let light_pos_group = device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &light_pos_layout,
            entries: &[BindGroupEntry { binding: 0, resource: BindingResource::Buffer(light_pos_buffer.as_entire_buffer_binding()) }]
        });

        group2_builder.bind_groups_layouts.push(light_pos_layout);

        let group2 = group2_builder.build(sc_desc,&device,ChunkVertexBuffer::desc(),PrimitiveTopology::TriangleList,true);

        return ChunkRenderPipelineGroup { group: group2, light_pos_buffer, light_pos_group }
    }

    pub fn new_with_shaders(builder:RenderPipelineGroupBuilder,sc_desc:&SwapChainDescriptor,device:&Device,desc:VertexBufferLayout,topology:PrimitiveTopology) -> RenderPipelineGroup {

        let mut layouts: Vec<&BindGroupLayout> = vec![];

        for l in builder.bind_groups_layouts.iter() {
            layouts.push(l);
        }
        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor{
            label: Some("Render Pipeline Layou"),
            bind_group_layouts: &*layouts,
            push_constant_ranges: &[]
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState { module: &builder.vertex_shader.unwrap(), entry_point: "main", buffers: &[desc] },
            fragment: Some(FragmentState{ module: &builder.fragment_shader.unwrap(), entry_point: "main", targets: &[ColorTargetState {
                format: sc_desc.format,
                blend: Some(BlendState {
                    color: BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    },
                    alpha: BlendComponent {
                        src_factor: wgpu::BlendFactor::SrcAlpha,
                        dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                        operation: wgpu::BlendOperation::Add,
                    }
                }),
                write_mask: ColorWrite::ALL
            }] }),
            primitive: PrimitiveState {
                topology,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: None,
                clamp_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false
            },
            depth_stencil: Some(DepthStencilState {
                format: TextureFormat::Depth32Float,
                depth_write_enabled: true,
                depth_compare: CompareFunction::Less,
                stencil: StencilState::default(),
                bias: DepthBiasState {
                    constant: 0,
                    slope_scale: 0.0,
                    clamp: 0.0
                }
            }),
            multisample: MultisampleState {
                count: 1,
                mask: 0,
                alpha_to_coverage_enabled: false
            }
        });

        return RenderPipelineGroup {
            pipeline: render_pipeline,
            bind_groups: builder.bind_groups,
            buffers: builder.buffers,
            vertex_buffers: builder.vertex_buffers
        }
    }
}