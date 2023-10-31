use std::sync::{Arc, Mutex};
use napi::{
    bindgen_prelude::{AsyncTask, Result},
    Env, Error, JsFunction, JsUndefined, Status, Task,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use tape_core::act::Actor;
use crate::ffi_adapter::FfiSafeScript;

pub struct AsyncAct {
    worker: Arc<Mutex<Actor>>,
}

impl Task for AsyncAct {
    type Output = ();
    type JsValue = ();

    fn compute(&mut self) -> napi::Result<Self::Output> {
        self.worker.lock().unwrap().act_sync()
            .map_err(|_| Error::new(Status::WouldDeadlock, "no stop signal set before calling act_async"))
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
    /// - act_type: the type of the acter, can be "keyboard", "mouse" or "both"
    /// - stop_signal: the signal to stop the acter, can be one of valid 'CanonicalKey'
    #[napi(constructor)]
    pub fn new(
        script: FfiSafeScript,
        cyclic: bool,
        #[napi(ts_arg_type = "'keyboard' | 'mouse' | 'both'")]
        act_type: String,
        stop_signal: Option<String>,
    ) -> NodeActor {
        let worker = Actor::new(script.into(), cyclic, act_type.into(), stop_signal.map(Into::into));

        NodeActor { inner: Arc::new(Mutex::new(worker)) }
    }


    /// Set the script to be acted.
    ///
    /// This has no effect on the current acting. (The script is cloned once `act_callback` or `act_async` is called)
    #[napi]
    pub fn new_script(&self, script: FfiSafeScript) {
        self.inner.lock().unwrap().new_script(script.into());
    }

    /// Set whether the actor is acting cyclically.
    ///
    /// Can affect the current acting cause 'cyclic' will be checked before every loop.
    #[napi]
    pub fn set_cyclic(&self, cyclic: bool) {
        self.inner.lock().unwrap().set_cyclic(cyclic);
    }

    /// Set the type of the action to be acted.
    ///
    /// This has no effect on the current acting. (The script is cloned and filtered once `act_callback` or `act_async` is called)
    #[napi]
    pub fn set_act_type(
        &self,
        #[napi(ts_arg_type = "'keyboard' | 'mouse' | 'both'")]
        act_type: String,
    ) {
        self.inner.lock().unwrap().set_act_type(act_type.into());
    }

    /// Set the key that stops the acting
    ///
    /// This has no effect on the current acting. (The signal is copied once `act_callback` or `act_async` is called.)
    #[napi]
    pub fn set_stop_signal(&self, stop_signal: Option<String>) {
        self.inner.lock().unwrap().set_stop_signal(stop_signal.map(Into::into));
    }

    /// Start acting (The act will stop when the stop signal is received,
    /// you can also use the `finish` to interrupt the acting manually).
    ///
    /// This will run in a separate thread (created by `std::thread::spawn`), so it will not block the main thread.
    /// On the other hand, you may need to wait in the main thread for the acting to finish.
    ///
    /// ---
    /// see `act_async` for promise-like usage
    #[napi]
    pub fn act_callback(
        &self,
        #[napi(ts_arg_type = "() => void")]
        on_finish: JsFunction,
    ) -> Result<()> {
        let tsfn: ThreadsafeFunction<(), ErrorStrategy::Fatal> = on_finish
            .create_threadsafe_function(0, |_ctx| {
                let v: Vec<JsUndefined> = vec![];
                Ok(v)
            })?;

        self.inner.lock().unwrap().act(Some(Box::new(move || {
            tsfn.call((), ThreadsafeFunctionCallMode::NonBlocking);
        })));

        Ok(())
    }

    /// Interrupt the acting started by `act_callback`
    #[napi]
    pub fn finish(&self) -> Result<()> {
        self.inner.lock().unwrap().finish();

        Ok(())
    }

    /// Start acting (The act will not stop until the stop signal is received,
    /// that is, you have to set the stop signal before calling this function or it will throw an error directly).
    ///
    /// This will run in a separate thread (created by `libuv`), so it will not block the main thread.
    ///
    /// ---
    /// see `act_callback` for callback-style usage
    #[napi(ts_return_type = "Promise<void>")]
    pub fn act_async(&self) -> AsyncTask<AsyncAct> {
        let shared_ptr = self.inner.clone();
        AsyncTask::new(AsyncAct { worker: shared_ptr })
    }
}