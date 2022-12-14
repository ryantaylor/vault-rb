use magnus::{define_module, function, prelude::*, Error};
use rb_allocator::ruby_global_allocator;
use rb_sys::ruby_abi_version;

// Ensure that the Ruby C ABI is being used.
ruby_abi_version!();

// Inform Ruby's GC about memory allocations.
ruby_global_allocator!();

fn hello(subject: String) -> String {
    format!("Hello from Rust, {}!", subject)
}

#[magnus::init]
fn init() -> Result<(), Error> {
    let module = define_module("Vault")?;
    module.define_singleton_method("hello", function!(hello, 1))?;
    Ok(())
}
