# Introduction

This guide demonstrates how to use the wgpu crate to interface with your GPU from a desktop application.

It will teach you the basics of graphics programming in the sense that you will know how to draw objects on the screen.
However this guide doesn't cover actual graphics programming techniques, such as loading a 3D model or adding realistic lighting to a scene.
It is structurally based off and has paragraphs copied from tomaka's Vulkano guide https://github.com/vulkano-rs/vulkano-www

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
However unlike the other new low level graphics APIs (Vulkan/Metal/DX12) it is memory safe for obvious security reasons.

## Dawn

[Dawn](https://dawn.googlesource.com/dawn) is Google's open source implementation of WebGPU, written in C++, it will be used by chrome/chromium.

It's worth knowing it exists, but there is no need to discuss it further.

## wgpu

wgpu is another open source implementation of WebGPU, this time written in Rust, and likely to be used in Firefox.
It is split into two crates:
*   [wgpu-native crate](https://crates.io/crates/wgpu-native) - [wgpu repo](https://github.com/gfx-rs/wgpu) - The actual implementation, provides a C API so firefox can use it as its WebGPU implementation.
*   [wgpu crate](https://crates.io/crates/wgpu) - [wgpu-rs repo](https://github.com/gfx-rs/wgpu-rs) - Wraps wgpu-native to provide a rusty API. Currently only usable for desktop applications but the idea is to allow targeting the web via this crate as well.

You might notice the github repo naming doesn't match up with the crate names.
This will be resolved when wgpu-native is moved to Mozilla Central (the firefox repository).

In this guide we'll only be using the wgpu crate.

## Starting a new project

First, create a new crate:
`cargo new learn-wgpu`

And then add the wgpu dependency to your `Cargo.toml`: `wgpu = { version = "0.3", git = "https://github.com/gfx-rs/wgpu-rs", rev = "17077f49b5a5c0ee65c9733e8fb262036cf5d706" }`

I use a recent git commit for this tutorial and recommend you use the same commit to avoid any differences in the API.
If you start to notice differences in the API, check back here, maybe I updated the commit.
