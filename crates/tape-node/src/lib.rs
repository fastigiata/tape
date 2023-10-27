#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::thread;
use napi::{bindgen_prelude::{Result, AsyncTask}, Env, JsFunction, Task, threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode, ErrorStrategy}};

mod record;
mod act;
mod ffi_adapter;

// region callback example
/// the callback will be invoked in another thread in 3 seconds (will arg "hi -- i'm from another thread")
#[napi]
pub fn callback_test(
    #[napi(ts_arg_type = "(v: string) => void")]
    callback: JsFunction
) -> Result<()> {
    let tsfn: ThreadsafeFunction<String, ErrorStrategy::Fatal> = callback
        .create_threadsafe_function(0, |ctx| {
            Ok(vec![ctx.value])
        })?;

    thread::spawn(move || {
        thread::sleep(std::time::Duration::from_secs(3));
        tsfn.call(format!("hi -- i'm from another thread"), ThreadsafeFunctionCallMode::NonBlocking);
    });

    Ok(())
}
// endregion

// region async-task example
struct AsyncGreet {}

impl Task for AsyncGreet {
    type Output = String;
    type JsValue = String;

    fn compute(&mut self) -> Result<Self::Output> {
        thread::sleep(std::time::Duration::from_secs(3));
        Ok(format!("hi -- i'm from async task"))
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output)
    }
}


/// the task will be resolved in 3 seconds (will return "hi -- i'm from async task")
#[napi(ts_return_type = "Promise<string>")]
fn async_task_test() -> AsyncTask<AsyncGreet> {
    AsyncTask::new(AsyncGreet {})
}
// endregion