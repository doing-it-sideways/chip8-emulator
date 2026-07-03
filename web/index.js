// Most code taken from @aquova's tutorial on github.
import init, * as wasm from "./wasm.js"

const WIDTH = 64
const HEIGHT = 32
const SCALE = 10
let anim_frame = 0

const canvas = document.getElementById("canvas");
canvas.width = WIDTH * SCALE
canvas.height = HEIGHT * SCALE

const ctx = canvas.getContext("2d")
ctx.fillStyle = "black"
ctx.fillRect(0, 0, WIDTH * SCALE, HEIGHT * SCALE)

const input = document.getElementById("fileinput")

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
    input.addEventListener("change", function(e) {
        if (anim_frame != 0) {
            window.cancelAnimationFrame(anim_frame)
        }

        let file = e.target.files[0]
        if (!file) {
            alert("Failed to read file")
            return
        }

        let fr = new FileReader()
        fr.onload = function(ev) {
            let buf = fr.result
            const rom = new Uint8Array(buf)
            chip8Binding = new wasm.Chip8Wasm()
        }

        fr.readAsArrayBuffer(file)

    }, false)

    chip8Binding.start(graphicsBinding, inputBinding)
}

run().catch(console.error)
