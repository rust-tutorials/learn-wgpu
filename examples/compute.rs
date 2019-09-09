fn main() {
    let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::Default,
        backends: wgpu::BackendBit::PRIMARY,
    }).unwrap();

    let device = adapter.request_device(&wgpu::DeviceDescriptor {
        extensions: wgpu::Extensions {
            anisotropic_filtering: false,
        },
        limits: wgpu::Limits::default(),
    });

    let numbers = [4u32, 13];
    let staging_buffer = device
        .create_buffer_mapped(
            numbers.len(),
            wgpu::BufferUsage::MAP_READ
                | wgpu::BufferUsage::COPY_DST
                | wgpu::BufferUsage::COPY_SRC,
        )
        .fill_from_slice(&numbers);
}
