# Initialization

## Creating an instance

Before you can start using wgpu, you need to create an `Instance`. 
Its quite boring, dont worry it gets better.

```rust
let instance = wgpu::Instance::new();
```

Before going further you can try your code by running:

```bash
cargo run
```

## Request adapter

You can now use your instance to create to create an `Adapter`.

```rust
let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
    power_preference: wgpu::PowerPreference::Default,
});
```

I don't really understand the difference between an adapter and a device, guess its time to ask some questions.

## Request device

You can now use your adapter to create to create a `Device`.

```rust
let mut device = adapter.request_device(&wgpu::DeviceDescriptor {
    extensions: wgpu::Extensions {
        anisotropic_filtering: false,
    },
    limits: wgpu::Limits::default(),
});
```

The machine you run your program on may have multiple devices that can act as a GPU. Before we can
ask a video card to perform some operations, we have to enumerate all the *physical device*s that
support Vulkan and choose which one we are going to use for this operation.

In reality a physical device can be a dedicated graphics card, but also an integrated graphics
processor or a software implementation. It can be basically anything that allows running Vulkan
operations.

As of the writing of this guide, it is not yet possible to use multiple devices simultaneously
in an efficient way (eg. SLI/Crossfire). You *can* use multiple devices simultaneously in the same
program, but there is not much point in doing so because you cannot share anything between them.
Consequently the best thing to do in practice is to choose one physical device which is going to run
everything:

```rust
use vulkano::instance::PhysicalDevice;

let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
```

The `enumerate` function returns an iterator to the list of available physical devices.
We call `next` on it to return the first device, if any. Note that the first device is not
necessarily the best device. In a real program you probably want to leave the choice to the user.

Keep in mind that the list of physical devices can be empty. This happens if Vulkan is installed
on the system, but none of the physical devices of the machine are capable of supporting Vulkan. In
a real-world application you are encouraged to handle this situation properly as well.
