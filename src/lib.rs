#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;
extern crate swc_common;
// Pull in the platform-appropriate allocator.
extern crate swc_node_base;

use napi::{JsObject, Result};

mod transpile_2015;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("transpile", transpile_2015::transpile_async)?;
  Ok(())
}
