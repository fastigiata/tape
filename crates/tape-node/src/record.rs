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

    #[napi]
    pub fn set_record_type(&mut self, record_type: String) {
        self.inner.set_record_type(record_type.into());
    }

    #[napi]
    pub fn set_stop_signal(&mut self, stop_signal: Option<String>) {
        self.inner.set_stop_signal(stop_signal.map(Into::into));
    }

    #[napi]
    pub fn record_callback(
        &mut self,
        #[napi(ts_arg_type = "(v: FFISafeAction) => void")]
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

    // #[napi]
    // pub fn record_callback() {
    // 
    // }

    // TODO: record_callback | see https://napi.rs/docs/concepts/threadsafe-function
    // TODO: record_async | see https://napi.rs/docs/concepts/async-task
}