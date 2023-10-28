import {Recorder} from "../index";
// import {Recorder} from "../index.js";

const recorder = new Recorder('keyboard')

// this call will rejected directly cause no stop-signal is specified
recorder.recordAsync()
    .then((v) => {
        console.log('[recordAsync 1] done:', v)
    })
    .catch(err => {
        console.log('[recordAsync 1] error:', err)
    })

// then we specify the stop-signal and call again
recorder.setStopSignal('esc')
// now it works, and the async call won't be finished until the specified key is pressed
recorder.recordAsync()
    .then((v) => {
        console.log('[recordAsync 2] done:', v)
    })
    .catch(err => {
        console.log('[recordAsync 2] error:', err)
    })

// here, you can do anything you want in the main thread and the recorder won't block it
console.log('this will not be blocked and will be printed immediately')

// The 'recording thread' will automatically join the main thread
// to prevent the child thread from being automatically killed when the main thread ends.