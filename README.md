# Chip-8 Interpreter/Emulator
I ask that you please do not use this repo to train an AI.

This was my first rust project and first time building anything for the web. Don't dream of doing anything web related like this again, but rust was interesting to say the least. Not a big fan of the language, but it's got it's perks for sure. Definitely has inspired some upcoming projects.

## A (mostly) complete interpreter
This project implements most of the behavior of the Chip-8 close to accurate to the behavior on the COSMAC. SUPERCHIP and Octo instructions are currently on hold as I'm satisfied where I left this project off for now.
- Runs on desktop environments using SDL3.
- Runs on the web through WASM.

## Build Instructions
Desktop:
- Go into the `desktop_build` folder and run `cargo build`.

Web:
- Go into the `wasm_build` folder and run `wasm-pack build --target web`
- Transfer `chip8_wasm_bg.wasm` and `chip8_wasm.js` to the `web` folder.

## Running Instructions
Desktop:
- Go into the `desktop_build` folder and run `cargo run -- {ROM_PATH}`
    - You can also run `--help` to list all the arguments and their purposes.
    - Optional arguments: 
        - `-s`: The scale the window will be at. (Default = 10)
        - `-C`: Chip-8 Interpreter Mode (Default = Octo)
            - NOTE: not all Octo/SUPERCHIP instructions implemented

Web:
1. From local:
    - Go into the `web` folder.
    - Start a web server, easiest with python by spinning up an http server with python.
        - Different terminals/if you're doing this inside of python itself will have slightly different commands, check the API for `http.server`
2. From GitHub/Browser:
    - Go to the [github pages website](https://doing-it-sideways.github.io/chip8-emulator/)


### Input
Keypad is emulated, such that 4 rows of keys are used for button presses, shown: {Keyboard Mapping} | {Chip-8 Keypad Mapping})
1 | 1    2 | 2    3 | 3    4 | C
Q | 4    W | 5    E | 6    R | D
A | 7    S | 8    D | 9    F | E
Z | A    X | 0    C | B    V | F

## Special Thanks
@aquova for providing a great tutorial which I referenced at certain points as a sort of outline guide for the project, especially when starting the web development portion.
