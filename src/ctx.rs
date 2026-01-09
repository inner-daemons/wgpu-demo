use wgpu::*;

pub struct RenderCtx {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub async fn create_ctx() -> RenderCtx {
    let instance = Instance::new(&Default::default());
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("No suitable adapter found");
    let (device, queue) = adapter.request_device(&Default::default()).await.unwrap();

    RenderCtx {
        instance,
        adapter,
        device,
        queue,
    }
}
