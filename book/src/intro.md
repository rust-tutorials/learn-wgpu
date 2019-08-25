# Introduction

This guide demonstrates how to use the wgpu crate to interface with your GPU from a desktop application.

It will teach you the basics of graphics programming in the sense that you will know how to draw objects on the screen.
However this guide doesn't cover actual graphics programming techniques, such as loading a 3D model or adding realistic lighting to a scene.

If you have any problems the book is on [GitHub](https://github.com/rust-tutorials/learn-wgpu), so
[open an issue](https://github.com/rust-tutorials/learn-wgpu/issues). You can also join the
[Rust Programming Community Discord](https://discordapp.com/invite/aVESxV8) and say stuff in the
`#games-and-graphics` channel.

## WebGPU

When you create a program (either in Rust or any other programming language) and run it, the
program's instructions are executed by the ***CPU*** (Central Processing Unit).

But some computers also usually have a ***video card*** plugged in them. This video card has its
own microprocessor called the ***GPU*** (Graphics Processing Unit) or the ***graphics processor***.
It can be seen more or less as a secondary machine within your main machine. Your monitor is
generally plugged in to your video card if you have one.

WebGPU is a web [API](https://gpuweb.github.io/gpuweb/) that the browsers are currently implementing.
It is lower level than the existing webgl API, as engine developers are always after more control of the GPU.
However unlike the other new low level graphics APIs (vulkan/metal/DX12) it is memory safe for obvious security reasons.

## wgpu

wgpu is an implementation of WebGPU.
It is split into two crates:
*   [wgpu-native crate](https://crates.io/crates/wgpu-native) - [wgpu repo](https://github.com/gfx-rs/wgpu) - The actual implementation, provides a C API so firefox can use it as its WebGPU implementation.
*   [wgpu crate](https://crates.io/crates/wgpu) - [wgpu-rs repo](https://github.com/gfx-rs/wgpu-rs) - Wraps wgpu-native to provide a rusty API. Currenly only usable for desktop applications. This is what you'll be using.

You might notice the github repo naming doesn't match up with the crate names.
This will be resolved when wgpu-native is moved to Mozilla Central (the firefox repository).

One day the wgpu crate will support running in a browser but that day is not today.

## Starting a new project

First, create a new crate:
`cargo new learn-wgpu`

And then add wgpu to your `Cargo.toml`, you will need to specify a backend you have available to you:
*   `wgpu = { version = 0.3, features = ["vulkan"] }` - Windows, Linux and Android
*   `wgpu = { version = 0.3, features = ["metal"] }` - macOS and iOS
*   `wgpu = { version = 0.3, features = ["dx12"] }` - Windows
*   `wgpu = { version = 0.3, features = ["dx11"] }` - Old Windows. But has more bugs.
*   `wgpu = { version = 0.3, features = ["gl"] }` - Everything! But has more bugs.

As you progress through the guide, add each provided block of rust code to your `main` function.