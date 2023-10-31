import {writeFileSync} from "node:fs"
import {Recorder} from "../index.js"

const recorder = new Recorder('keyboard', 'esc')

recorder.recordCallback((v) => {
    console.log('[recordCallback] done:', v)

    // output the script to json file
    writeFileSync('./sim.json', JSON.stringify(v), 'utf-8')
})

// here, you can do anything you want in the main thread and the recorder won't block it
console.log('this will not be blocked and will be printed immediately')

// The 'recording thread' will automatically join the main thread
// to prevent the child thread from being automatically killed when the main thread ends.

// or you can call `recorder.finish()` to manually stop the recorder whenever you want,
// here is an example, the recorder will be stopped after 5 seconds if you don't press the stop-signal
// setTimeout(() => {
//     recorder.finish()
// }, 5_000)