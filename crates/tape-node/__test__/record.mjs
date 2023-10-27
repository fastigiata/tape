import {Recorder} from "../index.js"


// callback test
const callback_test = () => {
    const recorder = new Recorder('keyboard', 'esc')
    recorder.recordCallback((v) => {
        console.log('result of callback invoke', v)
    })

    console.log('this will not be blocked and will be printed immediately')

    // the recorder will be finished after 5 seconds
    setTimeout(() => {
        recorder.finish()
    }, 5_000)
}

// async test
const async_test = async () => {
    const recorder = new Recorder('keyboard', 'esc')
    recorder.record((v) => {
        console.log('result of callback invoke', v)
    })

    console.log('this will not be blocked and will be printed immediately')

    // the recorder will be finished after 5 seconds
    setTimeout(() => {
        recorder.finish()
    }, 5_000)
}