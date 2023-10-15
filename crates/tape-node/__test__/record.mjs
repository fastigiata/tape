import {callbackTest, Recorder} from "../index.js"


// const recorder = new Recorder('keyboard', 'esc')
// recorder.record()

callbackTest(v => {
    console.log('log fron callbackTest', v)
})
console.log('non blocking!')
setTimeout(() => {
    console.log('task end!')
}, 5_000)