use std::fs::File;

use wgpu::*;

use crate::RenderCtx;

pub fn save_to_png(ctx: &RenderCtx, texture: &Texture) {
    assert!(texture.format() == TextureFormat::Rgba8Unorm);
    assert!(texture.width().is_multiple_of(256));
    assert!(texture.height().is_multiple_of(256));

    let num_bytes = (texture.width() as usize) * (texture.height() as usize) * 4;
    let output_staging_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("render_texture_output_buffer"),
        size: num_bytes as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut encoder = ctx
        .device
        .create_command_encoder(&CommandEncoderDescriptor {
            label: Some("download_render_encoder"),
        });
    encoder.copy_texture_to_buffer(
        TexelCopyTextureInfo {
            texture,
            mip_level: 0,
            origin: Origin3d::ZERO,
            aspect: TextureAspect::All,
        },
        TexelCopyBufferInfo {
            buffer: &output_staging_buffer,
            layout: TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(texture.width() * 4),
                rows_per_image: Some(texture.height()),
            },
        },
        Extent3d {
            width: texture.width(),
            height: texture.height(),
            depth_or_array_layers: 1,
        },
    );
    let submission = ctx.queue.submit([encoder.finish()]);

    output_staging_buffer
        .slice(..)
        .map_async(MapMode::Read, |_| ());
    ctx.device
        .poll(wgpu::wgt::PollType::Wait {
            submission_index: Some(submission),
            timeout: None,
        })
        .expect("Failed to wait for texture copy submission when saving to PNG");
    let mapped_buffer = output_staging_buffer.get_mapped_range(..);

    let mut png_encoder = png::Encoder::new(
        File::create("out.png").unwrap(),
        texture.width(),
        texture.height(),
    );
    png_encoder.set_color(png::ColorType::Rgba);
    let mut png_writer = png_encoder.write_header().unwrap();
    png_writer.write_image_data(&mapped_buffer).unwrap();
    png_writer.finish().unwrap();

    drop(mapped_buffer);
    output_staging_buffer.unmap();
}
