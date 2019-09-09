# Creating a buffer

When using wgpu, you will very often need the GPU to read or write data in memory. In fact
there isn't much point in using the GPU otherwise, as there is nothing you can do with the results
of its work except write them to memory.

In order for the GPU to be able to access some data (either for reading, writing or both), we
first need to create a ***buffer*** object and put the data in it.


```rust
let numbers = [4u32, 13];
let staging_buffer = device
    .create_buffer_mapped(
        numbers.len(),
        wgpu::BufferUsage::MAP_READ
            | wgpu::BufferUsage::COPY_DST
            | wgpu::BufferUsage::COPY_SRC,
    )
    .fill_from_slice(&numbers);
```

TODO: Give definition, so I can refer to arg names.

We have to indicate several things when creating the buffer.
The first parameter is the how many elements will be in the buffer.
This means the total number of bytes of the buffer will be length * the size of the data type being stored.

The second parameter indicates [which purpose we are creating the buffer](https://docs.rs/wgpu/0.3.0/wgpu/struct.BufferUsage.html)
for, which can help the implementation perform some optimizations.
Trying to use a buffer in a way that wasn't indicated in its constructor will result in an error.
It is always valid to use `BufferUsage::all()` to allow any type of usage, however this will be much slower than specifying its exact usages.

`create_buffer_mapped` only returns a `CreateBufferMapped`, we then have to call fill_from_slice on it to store our data and get back a `Buffer`.

Make sure you specify the data type in your array otherwise you will be left with whatever datatype rust gives your array and any offsets you give in later code about the array may be wrong.

> **Note**: In a real application you shouldn't create buffers with only 4 bytes of data. Although
> buffers aren't expensive, you should try to group as much data as you can in the same buffer.

## From_data and from_iter

In the example above we create a buffer that contains the value `12`, which is of type `i32`.
But you can put any type you want in a buffer, there is no restriction. You can, for example, write
this:

```rust
struct MyStruct {
    a: u32,
    b: bool,
}

let data = MyStruct { a: 5, b: true };

let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(),
                                            data).unwrap();
```

> **Note**: While you can put any type you want in a buffer, using a type that doesn't implement
> the `Send`, `Sync` and `Copy` traits or that isn't `'static` will restrict what you can do with
> that buffer.

While it is sometimes useful to use a buffer that contains a single struct, in practice it is very
common to put an array of values inside of a buffer. You can, for example, put an array of fifty
`i32`s in a buffer with the `CpuAccessibleBuffer::from_data` function.

However in practice it is also very common to not know the size of the array at compile-time. In
order to handle this, `CpuAccessibleBuffer` provides a `from_iter` constructor that takes an
iterator to the data as last parameter, instead of the data itself.

In the example below, we create a buffer that contains the value `5` of type `u8`, 128 times. The
type of the content of the buffer is `[u8]`, which, in Rust, represents an array of `u8`s whose size
is only known at runtime.

```rust
let iter = (0 .. 128).map(|_| 5u8);
let buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                            iter).unwrap();
```

You now know how to create a `CpuAccessibleBuffer`.
Keep in mind that `from_data` and `from_iter` are specific to the `CpuAccessibleBuffer`. Each type
of buffer has its own constructors, sometimes similar but sometimes different.

## Reading and writing the contents of a buffer

Once a `CpuAccessibleBuffer` is created, you can access its content with the `read()` or `write()`
methods. Using `read()` will grant you shared access to the content of the buffer, and using
`write()` will grant you exclusive access. This is similar to using a `RwLock`.

For example if `buffer` contains a `MyStruct` (see above):

```rust
let mut content = buffer.write().unwrap();
// `content` implements `DerefMut` whose target is of type `MyStruct` (the content of the buffer)
content.a *= 2;
content.b = false;
```

Alternatively, suppose that the content of `buffer` is of type `[u8]` (like with the example that
uses `from_iter`):

```rust
let mut content = buffer.write().unwrap();
// this time `content` derefs to `[u8]`
content[12] = 83;
content[7] = 3;
```

Just like the constructors, keep in mind that being able to read/write the content of the buffer
like this is specific to the `CpuAccessibleBuffer`. Other kinds of buffers (for example the
`ImmutableBuffer`) do not provide such methods.

Next: [Example operation](/guide/example-operation)
