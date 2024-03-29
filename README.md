# limine-protocol
This is a binding crate for the be-a-utiful [Limine Protocol Version 3](https://github.com/limine-bootloader/limine/blob/trunk/PROTOCOL.md)
It is intended to be ergonomic to the point expected by OS Devs, but hopefully it will help others get into the hobby. (I swear the pain is worth it, it's great)
Tests are intended to ensure that there is no UB in the implementations themselves, and that it is, indeed, the fault of the bootloader. (Bring it on, Limine devs!)

# Example
See the `example` directory for a simple example

# Changelog
* 0.5.0
    * Fix request ID for Framebuffer Request
* 0.4.3
    * Document possible need of `#[used]`
* 0.4.2
    * Fix visibility issue
* 0.4.1
    * Add more documentation and an example
* 0.4.0
    * Add `get_response` to everything to reduce boilerplate
    * Make the `UnsafeCell` in `Request<T>` not public
    * Add more clippy lints
* 0.3.2
    * Removed the feature `core_c_str`, as it has been stabilized as of 1.64.0
    * Changed offset field of struct `HHDMResponse` to `*mut ()` from `u64`
* 0.3.1
    * I left something old in, whoops. Gone now!
* 0.3.0
    * Re-export requests and responses. That's right, you won't have to type `request::SomeRequest` anymore!
* 0.2.1
    * Fix a type
* 0.2.0
    * Add const default for all requests
    * Rename `LimineRequest` to `Request` and `into_request` into plain ol' `into`
    * Added a new macro for making new requests
    * Add a README.md
    * Renamed Info{Request, Response} to BootloaderInfo{Request, Response}
    * Updated Terminal docs 
    * Added an example
* 0.1.0
    * Add everything : ^)