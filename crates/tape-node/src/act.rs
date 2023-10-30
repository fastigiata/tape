use std::sync::{Arc, Mutex};
use napi::{Env, Error, Status, Task};
use tape_core::act::Actor;
use crate::ffi_adapter::FFISafeScript;

pub struct AsyncActor {
    worker: Arc<Mutex<Actor>>,
}

impl Task for AsyncActor {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> napi::Result<Self::Output> {
        self.worker.lock().unwrap().act_sync()
            .map_err(|_| Error::new(Status::WouldDeadlock, "no stop signal set before calling record_async"))
    }

    fn resolve(&mut self, _env: Env, _output: Self::Output) -> napi::Result<Self::JsValue> {
        Ok(())
    }

    fn reject(&mut self, _env: Env, err: Error) -> napi::Result<Self::JsValue> {
        Err(err)
    }
}

#[napi(js_name = "Actor")]
pub struct NodeActor {
    inner: Arc<Mutex<Actor>>,
}

#[napi]
impl NodeActor {
    /// create a new actor
    /// - script: the script to act, can be changed later
    /// - cyclic: whether the actor is acting cyclically
    /// - record_type: the type of the recorder, can be "keyboard", "mouse" or "both"
    /// - stop_signal: the signal to stop the recorder, can be one of valid 'CanonicalKey'
    #[napi(constructor)]
    pub fn new(
        script: FFISafeScript,
        cyclic: bool,
        #[napi(ts_arg_type = "'keyboard' | 'mouse' | 'both'")]
        act_type: String,
        stop_signal: Option<String>,
    ) -> NodeActor {
        let worker = Actor::new(script.into(), cyclic, act_type.into(), stop_signal.map(Into::into));

        NodeActor { inner: Arc::new(Mutex::new(worker)) }
    }

    // TODO
}