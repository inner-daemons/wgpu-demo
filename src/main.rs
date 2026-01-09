mod ctx;
mod render;
mod save_to_png;
pub use ctx::{RenderCtx, create_ctx};
pub use save_to_png::save_to_png;

use crate::render::render;

async fn async_main() {
    println!("Hello, world!");
    let ctx = create_ctx().await;
    let texture = render(&ctx);
    save_to_png(&ctx, &texture);
}

fn main() {
    pollster::block_on(async_main());
}
