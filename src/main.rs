mod ctx;
mod render;
mod save_to_png;
use std::time::Instant;

pub use ctx::{RenderCtx, create_ctx};
pub use save_to_png::save_to_png;

use crate::render::render;

async fn async_main() {
    let t1 = Instant::now();
    // Create context
    let ctx = create_ctx().await;
    let t2 = Instant::now();
    println!("Setup context in {} seconds", (t2 - t1).as_secs_f32());
    // Render to texture
    let texture = render(&ctx);
    let t3 = Instant::now();
    println!("Rendered texture in {} seconds", (t3 - t2).as_secs_f32());
    // Save texture to PNG file
    save_to_png(&ctx, &texture);
    let t4 = Instant::now();
    println!(
        "Saved to PNG in {} seconds, for {} total",
        (t4 - t3).as_secs_f32(),
        (t4 - t1).as_secs_f32()
    );
}

fn main() {
    // Run async main function
    pollster::block_on(async_main());
}
