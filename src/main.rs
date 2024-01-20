#![no_std]
// "#![no_std]" shows that the rust standard library will not be used
#![no_main]
// When developing without the standard library the application also has no access to the rust- and the c-runtime (crt0 =^ C runtome 0).
// Both this two runtimes would normally start the process and create a stack, set Nullpointer guards and set the args to the registers.fn main() {
// Also the runtime would the main function. Since there is no runtime we can also not use a main function.fn main() {
// The solution is to implement a custom start element to start the programm
mod vga_buffer;

use core::panic::PanicInfo;
/** The new start function is annotated with #[no_mangle] to assure that the name of the compiled function is still "_start" and not some generated Code
This is important since "_start" is the default entrypoint into programs on most systems
marking the function with 'pub extern "C"' is necessary to assure the compiler uses the "C Calling Convention"
A calling convention is the way programs / subroutines a called on a low-level and how parameters to these programs are handled
*/
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //start address of the vga buffer which will be displayed
    vga_buffer::print_something();

    loop {}
}

/** when the standard library is used it provides an panic_handler.
Because it is not used here an own panic_handler funciton needs to be implemented.
this one just loops.
*/
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}