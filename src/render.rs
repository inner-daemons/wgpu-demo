use wgpu::*;

use crate::RenderCtx;

pub fn render(ctx: &RenderCtx) -> Texture {
    const FORMAT: TextureFormat = TextureFormat::Rgba8Unorm;
    const TEXTURE_DIM: u32 = 4096;

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
        let render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("main_render_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: &texture_view,
                depth_slice: None,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::GREEN),
                    store: StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        // Render
    }

    ctx.queue.submit([encoder.finish()]);
    texture
}
