# Initialization

## Request adapter

The first step is to create an `Adapter`.
The adapter corresponds to a specific physical GPU in the machine your code is running on.
We specify a `PowerPreference` allowing us to give preference to the type of GPU we are given access to. e.g. `PowerPreference::LowPower` would likely give us an integrated GPU. While `PowerPrefernce::HighPerformance` would likely give us a discrete GPU.

```rust
let adapter = wgpu::Adapter::request(&wgpu::RequestAdapterOptions {
    power_preference: wgpu::PowerPreference::Default,
    backends: wgpu::BackendBit::PRIMARY,
}).unwrap();
```

## Request device

You can now use your adapter to create a `Device`.
The `Device` represents an open channel of communication with the physical device (the `Adapter`).
Most of the wgpu API is accessed via various methods on the `Device`, so every part of your graphics code will likely need to access it.

```rust
let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
    extensions: wgpu::Extensions {
        anisotropic_filtering: false,
    },
    limits: wgpu::Limits::default(),
});
```

