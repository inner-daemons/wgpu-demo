//! Stuff related to wgpu context creation & setup

use wgpu::*;

pub struct RenderCtx {
    pub instance: Instance,
    pub adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
}

pub async fn create_ctx() -> RenderCtx {
    // Create API instance
    let instance = Instance::new(Default::default());
    // Select a physical device and backend (e.g. RTX 3060 on Vulkan)
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        })
        .await
        .expect("No suitable adapter found");
    // Request a handle to that device and queue to submit work to
    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            required_features: Features::EXPERIMENTAL_MESH_SHADER,
            experimental_features: unsafe { ExperimentalFeatures::enabled() },
            required_limits: Limits::defaults().using_recommended_minimum_mesh_shader_values(),
            ..Default::default()
        })
        .await
        .unwrap();

    RenderCtx {
        instance,
        adapter,
        device,
        queue,
    }
}
