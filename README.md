## Rust-OS

This is a simple operating system written in Rust. It is based on the [Blog from Philipp Oppermann](https://os.phil-opp.com/) tutorial.

### Build

To build the project you need to add the right compile target to rustup to avoid getting linker errors.
These linker errors are caused because the compiler whant to compile to the host systeme and use the C-runtime which it should not
````bash
rustup target add thumbv7em-none-eabihf
````

Which specifies that the target is a 32-bit ARM architecture (thumbv7), that it doesn’t use an operating system (none), and that float instructions are handled by the hardware (eabihf).

