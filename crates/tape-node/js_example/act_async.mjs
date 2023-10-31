import {readFileSync} from "node:fs"
import {Actor} from "../index.js";

const script = JSON.parse(readFileSync('./sim.json', 'utf-8'))

const actor = new Actor(script, false, 'keyboard', 'esc')

actor
    .actAsync()
    .then(() => {
        console.log('[actCallback] done')
    })
    .catch(err => {
        console.log(err)
    })
