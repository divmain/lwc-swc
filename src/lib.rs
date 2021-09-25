#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate swc_common;
// Pull in the platform-appropriate allocator.
extern crate swc_node_base;

use napi::{JsObject, Result};

mod transpile_2015;

#[cfg(all(
  any(windows, unix),
  target_arch = "x86_64",
  not(target_env = "musl"),
  not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("transpile", transpile_2015::transpile_async)?;
  Ok(())
}
