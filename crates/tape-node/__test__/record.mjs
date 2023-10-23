import {Recorder} from "../index.js"

const recorder = new Recorder('keyboard', 'esc')

// callback test
recorder.recordCallback((v) => {
    console.log(v)
})

// the recorder will be finished after 5 seconds
setTimeout(() => {
    recorder.finish()
}, 5_000)

// async test
// TODO

console.log('this will not be blocked and will be printed immediately')