import {Recorder} from "../index.js"


const recorder = new Recorder('keyboard', 'esc')
recorder.record()

setTimeout(() => {
    console.log('task end!')
}, 5_000)
// 123