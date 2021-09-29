#![deny(clippy::all)]

mod minify;
mod transpile;

#[macro_use]
extern crate napi_derive;
extern crate swc_common;
// Pull in the platform-appropriate allocator.
extern crate swc_node_base;

use crate::minify::{minify, MinifiedModule};
use crate::transpile::{transpile, TranspiledModule};
use napi::{
  CallContext, Env, Error, JsBuffer, JsBufferValue, JsObject, JsString, Ref, Result, Task,
};
use std::ops::Deref;

#[module_exports]
fn init(mut exports: JsObject) -> Result<()> {
  exports.create_named_method("transpile", transpile_async)?;
  exports.create_named_method("minify", minify_async)?;
  Ok(())
}

#[js_function(2)]
pub fn transpile_async(ctx: CallContext) -> Result<JsObject> {
  let filename = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let source = ctx.get::<JsBuffer>(1)?.into_ref()?;
  let task = TranspileTask(filename, source);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

struct TranspileTask(pub String, pub Ref<JsBufferValue>);

impl Task for TranspileTask {
  type Output = TranspiledModule;
  type JsValue = JsObject;

  fn compute(&mut self) -> Result<Self::Output> {
    let filename = self.0.clone();
    let source = self.1.deref();

    transpile(filename, &source)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    self.1.unref(env)?;

    let mut obj = env.create_object()?;
    obj.set_named_property(
      "filename",
      env.create_string_from_std(output.filename.clone())?,
    )?;
    obj.set_named_property(
      "code",
      env.create_string_from_std(output.transpile_result.code)?,
    )?;
    match output.transpile_result.map {
      Some(_map) => obj.set_named_property("map", env.create_string_from_std(_map)?)?,
      None => obj.set_named_property("map", env.get_null()?)?,
    };

    Ok(obj)
  }

  fn reject(self, env: Env, err: Error) -> Result<Self::JsValue> {
    self.1.unref(env)?;
    Err(err)
  }
}

#[js_function(1)]
pub fn minify_async(ctx: CallContext) -> Result<JsObject> {
  let code = ctx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
  let task = MinifyTask(code);
  let async_task = ctx.env.spawn(task)?;
  Ok(async_task.promise_object())
}

struct MinifyTask(pub String);

impl Task for MinifyTask {
  type Output = MinifiedModule;
  type JsValue = JsObject;

  fn compute(&mut self) -> Result<Self::Output> {
    let code = self.0.clone();
    minify(code)
  }

  fn resolve(self, env: Env, output: Self::Output) -> Result<Self::JsValue> {
    let mut obj = env.create_object()?;
    obj.set_named_property("code", env.create_string_from_std(output.code.clone())?)?;
    match output.map {
      Some(_map) => obj.set_named_property("map", env.create_string_from_std(_map.clone())?)?,
      None => obj.set_named_property("map", env.get_null()?)?,
    };

    Ok(obj)
  }

  fn reject(self, _env: Env, err: Error) -> Result<Self::JsValue> {
    Err(err)
  }
}
