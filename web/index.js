// Most code taken from @aquova's tutorial on github.
import init, * as wasm from "./chip8_wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 10
let anim_frame = 0

const canvas = document.getElementById("canvas")
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

const fileinput = document.getElementById("rominput")
const interpreterMode = document.getElementById("mode selector")

function loadRom(chip8, file) {
    if (anim_frame != 0) {
        window.cancelAnimationFrame(anim_frame)
    }

    if (!file) {
        alert("Failed to read file")
        return
    }

    let fr = new FileReader()
    fr.onload = function(ev) {
        let buf = fr.result
        const rom = new Uint8Array(buf)

        chip8.reload(interpreterMode.elements["mode"].value, rom)
    }

    fr.readAsArrayBuffer(file)
}

function loadDemo(chip8) {
    const req = new XMLHttpRequest()
    req.open("GET", "./demo/cavern.ch8")
    req.responseType = "arraybuffer"
    req.onload = function() {
        if (!(req.status === 200)) {
            throw "Couldn't retrieve file: " + req.status
        }
        else {
            console.log("demo file get: " + req.status)
        }
    }
    req.send()
    
    const demo = new Uint8Array(req.response)
    chip8.reload(interpreterMode.elements["mode"].value, demo)
}

async function run() {
    await init()
    let chip8Binding = new wasm.Chip8Wasm()
    let graphicsBinding = new wasm.JSGraphicsCtx(SCALE)
    let inputBinding = new wasm.JSInput()
    
    document.addEventListener("keydown", function(keyEvent) {
        inputBinding.keypressEvent(keyEvent, true)
    })

    document.addEventListener("keyup", function(keyEvent) {
        inputBinding.keypressEvent(keyEvent, false)
    })

    // change game rom
    fileinput.addEventListener("change", function(e) {
        let file = event.target.files[0]
        loadRom(chip8Binding, file)
    }, false)

    
    loadDemo(chip8Binding)
    //chip8Binding.start(graphicsBinding, inputBinding)
}

run().catch(console.error)
