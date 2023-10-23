use napi::{
    bindgen_prelude::{Result},
    JsFunction,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use tape_core::record::Recorder;
use crate::ffi_adapter::{FFISafeAction, FFISafeScript};

#[napi(js_name = "Recorder")]
pub struct NodeRecorder {
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

    /// Start recording
    /// (asynchronous, you can use `finish` to interrupt the recording).
    ///
    /// This will run in a separate thread, so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the recording to finish.
    /// ---
    /// If you want to use synchronous recording, please call `record_async`.
    #[napi]
    pub fn record_callback(
        &mut self,
        #[napi(ts_arg_type = "(v: FfiSafeAction) => void")]
        callback: JsFunction,
    ) -> Result<()> {
        let tsfn: ThreadsafeFunction<FFISafeScript, ErrorStrategy::Fatal> = callback
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

    // #[napi]
    pub fn record_async() {
        todo!()
    }
}