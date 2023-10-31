use std::sync::{Arc, Mutex};
use napi::{
    bindgen_prelude::{AsyncTask, Result},
    Env, Error, JsFunction, Status, Task,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use tape_core::canonicalize::Script;
use tape_core::record::Recorder;
use crate::ffi_adapter::{FfiSafeScript};

pub struct AsyncRecord {
    worker: Arc<Mutex<Recorder>>,
}

impl Task for AsyncRecord {
    type Output = Script;
    type JsValue = FfiSafeScript;

    fn compute(&mut self) -> Result<Self::Output> {
        self.worker.lock().unwrap().record_sync()
            .map_err(|_| Error::new(Status::WouldDeadlock, "no stop signal set before calling record_async"))
    }

    fn resolve(&mut self, _env: Env, output: Self::Output) -> Result<Self::JsValue> {
        Ok(output.into())
    }

    fn reject(&mut self, _env: Env, err: Error) -> Result<Self::JsValue> {
        Err(err)
    }
}

#[napi(js_name = "Recorder")]
pub struct NodeRecorder {
    inner: Arc<Mutex<Recorder>>,
}

#[napi]
impl NodeRecorder {
    /// create a new recorder
    /// - record_type: the type of the recorder, can be "keyboard", "mouse" or "both"
    /// - stop_signal: the signal to stop the recorder, can be one of valid 'CanonicalKey'
    #[napi(constructor)]
    pub fn new(
        #[napi(ts_arg_type = "'keyboard' | 'mouse' | 'both'")]
        record_type: String,
        stop_signal: Option<String>,
    ) -> NodeRecorder {
        let worker = Recorder::new(record_type.into(), stop_signal.map(Into::into));

        NodeRecorder { inner: Arc::new(Mutex::new(worker)) }
    }

    /// Set the type of the action to be recorded
    ///
    /// This has no effect on the current recording. (The listener is set once `record_callback` or `record_async` is called.)
    #[napi]
    pub fn set_record_type(
        &self,
        #[napi(ts_arg_type = "'keyboard' | 'mouse' | 'both'")]
        record_type: String,
    ) {
        self.inner.lock().unwrap().set_record_type(record_type.into());
    }

    /// Set the key that stops the recording
    ///
    /// This has no effect on the current recording. (The signal is copied once `record_callback` or `record_async` is called.)
    #[napi]
    pub fn set_stop_signal(&self, stop_signal: Option<String>) {
        self.inner.lock().unwrap().set_stop_signal(stop_signal.map(Into::into));
    }

    /// Start recording (The record will stop when the stop signal is received,
    /// you can also use the `finish` to interrupt the recording manually).
    ///
    /// This will run in a separate thread (created by `std::thread::spawn`), so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    ///
    /// ---
    /// see `record_async` for promise-like usage
    #[napi]
    pub fn record_callback(
        &self,
        #[napi(ts_arg_type = "(v: FfiSafeScript) => void")]
        on_finish: JsFunction,
    ) -> Result<()> {
        let tsfn: ThreadsafeFunction<FfiSafeScript, ErrorStrategy::Fatal> = on_finish
            .create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?;

        self.inner.lock().unwrap().record(Some(Box::new(move |script| {
            tsfn.call(script.into(), ThreadsafeFunctionCallMode::NonBlocking);
        })));

        Ok(())
    }

    /// Interrupt the recording started by `record_callback`
    #[napi]
    pub fn finish(&self) -> Result<()> {
        self.inner.lock().unwrap().finish();

        Ok(())
    }

    /// Start recording (The record will not stop until the stop signal is received,
    /// that is, you have to set the stop signal before calling this function or it will throw an error directly).
    ///
    /// This will run in a separate thread (created by `libuv`), so it will not block the main thread.
    ///
    /// ---
    /// see `record_callback` for callback-style usage
    #[napi(ts_return_type = "Promise<FfiSafeScript>")]
    pub fn record_async(&self) -> AsyncTask<AsyncRecord> {
        let shared_ptr = self.inner.clone();
        AsyncTask::new(AsyncRecord { worker: shared_ptr })
    }
}