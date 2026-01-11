use wgpu::*;

use crate::RenderCtx;

pub fn render(ctx: &RenderCtx) -> Texture {
    const FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
    const TEXTURE_DIM: u32 = 1024;

    let texture = ctx.device.create_texture(&TextureDescriptor {
        label: Some("render_texture"),
        size: Extent3d {
            width: TEXTURE_DIM,
            height: TEXTURE_DIM,
            depth_or_array_layers: 1,
        },
        format: FORMAT,
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    // Need a view to tell the GPU which parts of the texture to render to
    let texture_view = texture.create_view(&TextureViewDescriptor {
        label: Some("render_texture_view"),
        array_layer_count: None,
        format: Some(FORMAT),
        dimension: Some(TextureViewDimension::D2),
        usage: Some(TextureUsages::RENDER_ATTACHMENT),
        aspect: TextureAspect::All,
        base_array_layer: 0,
        base_mip_level: 0,
        mip_level_count: None,
    });

    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("render_encoder"),
        });

    {
        // The render pass tells the GPU to prepare to render to specific textures.
        let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("main_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    // Clear the screen background color to debug magenta
                    load: LoadOp::Clear(Color {
                        r: 1.0,
                        g: 0.0,
                        b: 1.0,
                        a: 1.0,
                    }),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        let shader = ctx
            .device
            .create_shader_module(wgpu::include_wgsl!("shader.wgsl"));
        /*let pipeline = ctx
            .device
            .create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: None,
                vertex: VertexState {
                    module: &shader,
                    entry_point: Some("vertex"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: Some(Face::Back),
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: Default::default(),
                fragment: Some(FragmentState {
                    module: &shader,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[Some(ColorTargetState {
                        format: FORMAT,
                        blend: None,
                        write_mask: ColorWrites::ALL,
                    })],
                }),
                multiview_mask: None,
                cache: None,
            });
        render_pass.set_pipeline(&pipeline);
        render_pass.draw(0..3, 0..1);*/
        let pipeline = ctx.device.create_mesh_pipeline(&MeshPipelineDescriptor {
            label: None,
            layout: None,
            task: Some(TaskState {
                module: &shader,
                entry_point: Some("task"),
                compilation_options: Default::default(),
            }),
            mesh: MeshState {
                module: &shader,
                entry_point: Some("mesh"),
                compilation_options: Default::default(),
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fragment"),
                compilation_options: Default::default(),
                targets: &[Some(ColorTargetState {
                    format: FORMAT,
                    blend: None,
                    write_mask: ColorWrites::ALL,
                })],
            }),
            multiview: None,
            cache: None,
        });
        render_pass.set_pipeline(&pipeline);
        render_pass.draw_mesh_tasks(1, 1, 1);
        // Render
    }

    ctx.queue.submit([encoder.finish()]);

    texture
}
