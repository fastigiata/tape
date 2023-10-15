import {callbackTest} from "../index.js";

callbackTest(v => {
    console.log(`[callback]: ${v}`)
})

console.log('log right after callbackTest() -- non-blocking')

setTimeout(() => {
    console.log('log after 5 seconds -- test end')
}, 5_000)

// stdout should be:
// log right after callbackTest() -- non-blocking
// [callback]: hi -- i'm from another thread
// log after 5 seconds -- test end