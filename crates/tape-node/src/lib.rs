#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use std::thread;
use napi::{
    bindgen_prelude::Result,
    JsFunction,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode, ErrorStrategy},
};

mod record;
mod act;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

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
