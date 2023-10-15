import {asyncTaskTest} from "../index.js";

const start = Date.now()
asyncTaskTest()
    .then(v => {
        console.log(`[resolve in ${Date.now() - start}ms] ${v}`)
    })
    .catch(err => {
        console.log(`[error in ${Date.now() - start}ms] ${err}`)
    })
console.log(`log right after asyncTaskTest() -- non-blocking`)

setTimeout(() => {
    console.log(`log after 5 seconds -- test end`)
}, 5_000)

// stdout should be:
// log right after asyncTaskTest() -- non-blocking
// [resolve in 3000ms] hi -- i'm from async task
// log after 5 seconds -- test end