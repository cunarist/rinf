# libvips-rust-bindings
Rust bindings for libvips. Generated from `version 8.14.5`.

This is a safe wrapper for [libvips](https://libvips.github.io/libvips/) C library. It is made on top of the C API and based on the introspection API results.

This crate itself is not documented, but it has no logic or special behavior in comparison to libvips itself. All calls and types described in the official libvips [docs](https://libvips.github.io/libvips/API/current/) are just translated to rust types. All defaults also respected.

## How the crate was written

As a first step, it runs the bindgen to generate unsafe calls to the C libvips library. After this is generated, a C code is compiled and executed. This code introspects the operations and outputs them as text. This text is parsed and then generates the `error.rs` and the `ops.rs` modules.

Those are basically safe wrappers on top of the also genereated bindings. Though not widely tested, all the memory cleaning should be working as expected. Important to note that all "vips" prefixes in the naming were removed from the operations's names.

Both the bindings and the generated operations were pushed to crates.io with most of optional dependencies from libvips included. Be careful when calling functions that are dependent on those sub-dependencies (most of them format related).

### Contributing

Everything in ops.rs and error.rs (and of course bindings.rs) is generated programmatically. You need to make changes for these files to the builder for these. Then, run the following shell scripts from the `generator` directory.

```
$ ./build.sh     # Builds the libvips-builder docker image
$ ./generate.sh  # Actually generates the bindings
```

## A note to the maintainers

The publication of the create is done manually and requires that the _Cargo.lock_ version be updated in the repo after this has been done. This can be changed once github actions have been added to the repo in order to publish after merge.

## How to use it

The main entity from this crate is the `VipsApp` struct. It doesn't store any information, but as long as it is not dropped, vips should be working as expected.

Vips needs to be initialized and shut down, this struct does this job, though you don't have to shut it down, it will be done automatically when the variable holding the value of a `VipsApp` struct is droped.

Not all functions were implemented, so if you need some that are not yet there, feel free to open a PR or an issue (it is pretty straight forward to add the ones that needs to be manual).

Many vips operations have optional arguments. The ones that have have been implemented with too variants by this crate. Basically there'll be a regular call with only the required parameters and an additional with the suffix `with_opts` which will take a struct holding the defaults. 

The structs's names for those defaults are named after the operation name in `class case` plus the suffix `Options`. All the struct implements the `Default` trait, so you can construct them like this for example: 

```rust
let options = ops::Composite2Options {
    x: 10,
    y: 10,
    .. Composite2Options::default()
}
```

In the moment the error messages are not being appended to the errors themselves. They're in the libvips error buffer. The error buffer operations are implented inside the `VipsApps` struct. 

Most (if not all) vips operations don't mutate the `VipsImage` object, so they'll return a new object for this. The implementation of `VipsImage` in this crate takes care of freeing the internal pointer after it is dropped. <span style="color:red">Be aware that the VipsImage object is not thread safe in the moment.</span> I'll investigate what is happening and provide a solution for it in the future. 

### Example

```rust
use libvips::{ops, VipsImage, VipsApp};

fn main() {
    // this initializes the libvips library. it has to live as long as the application lives (or as long as you want to use the library within your app)
    // you can't have multiple objects of this type and when it is dropped it will call the libvips functions to free all internal structures.
    let app = VipsApp::new("Test Libvips", false).expect("Cannot initialize libvips");
    //set number of threads in libvips's threadpool
    app.concurrency_set(2);
    // loads an image from file
    let image = VipsImage::new_from_file("test.png").unwrap();

    // will resize the image and return a new instance.
    // libvips works most of the time with immutable objects, so it will return a new object
    // the VipsImage struct implements Drop, which will free the memory
    let resized = ops::resize(&image, 0.5).unwrap();

    //optional parameters
    let options = ops::JpegsaveOptions {
        q: 90,
        background: vec![255.0],
        strip: true,
        optimize_coding: true,
        optimize_scans: true,
        interlace: true,
        ..ops::JpegsaveOptions::default()
    };

    // alternatively you can use `jpegsave` that will use the default options
    match ops::jpegsave_with_opts(&resized, "output.jpeg",  &options) {
        Err(_) => println!("error: {}", app.error_buffer().unwrap()),
        Ok(_) => println!("Great Success!")
    }
}
```
