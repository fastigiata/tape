use napi::{
    bindgen_prelude::{AsyncTask, Result},
    Env, Error, JsFunction, Status, Task,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use tape_core::canonicalize::Script;
use tape_core::record::Recorder;
use crate::ffi_adapter::{FFISafeScript};

struct AsyncRecord {
    // TODO: fix error here
    task: Box<dyn Fn() -> std::result::Result<Script, ()> + Send>,
}

impl Task for AsyncRecord {
    type Output = Script;
    type JsValue = FFISafeScript;

    fn compute(&mut self) -> Result<Self::Output> {
        (self.task)().map_err(|_| Error::new(Status::WouldDeadlock, "no stop signal set before calling record_async"))
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
    // TODO: consider using arc for async usage
    inner: Recorder,
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
        NodeRecorder {
            inner: Recorder::new(record_type.into(), stop_signal.map(Into::into))
        }
    }

    /// Set the type of the action to be recorded
    ///
    /// This has no effect on the current recording. (The listener is set once `record_callback` or `record_async` is called.)
    #[napi]
    pub fn set_record_type(&mut self, record_type: String) {
        self.inner.set_record_type(record_type.into());
    }

    /// Set the key that stops the recording
    ///
    /// This has no effect on the current recording. (The signal is copied once `record_callback` or `record_async` is called.)
    #[napi]
    pub fn set_stop_signal(&mut self, stop_signal: Option<String>) {
        self.inner.set_stop_signal(stop_signal.map(Into::into));
    }

    /// Start recording (The record will stop when the stop signal is received,
    /// you can also use the `finish` to interrupt the recording manually).
    ///
    /// This will run in a separate thread (created by `std::thread::spawn`), so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    /// ---
    /// see `record_async` for promise-like usage
    #[napi]
    pub fn record_callback(
        &mut self,
        #[napi(ts_arg_type = "(v: FfiSafeAction) => void")]
        on_finish: JsFunction,
    ) -> Result<()> {
        let tsfn: ThreadsafeFunction<FFISafeScript, ErrorStrategy::Fatal> = on_finish
            .create_threadsafe_function(0, |ctx| {
                Ok(vec![ctx.value])
            })?;

        self.inner.record(Some(Box::new(move |script| {
            tsfn.call(script.into(), ThreadsafeFunctionCallMode::NonBlocking);
        })));

        Ok(())
    }

    /// Interrupt the recording started by `record_callback`
    #[napi]
    pub fn finish(&self) -> Result<()> {
        self.inner.finish();

        Ok(())
    }

    /// Start recording (The record will not stop until the stop signal is received,
    /// that is, you have to set the stop signal before calling this function or it will throw an error directly).
    ///
    /// This will run in a separate thread (created by `xxx`), so it will not block the main thread.
    /// ---
    /// see `record_callback` for callback-style usage
    #[napi]
    pub fn record_async(&self) -> AsyncTask<AsyncRecord> {
        // TODO: implement async usage
        AsyncTask::new(AsyncRecord { task: Box::new(|| self.inner.record_sync()) })
    }
}