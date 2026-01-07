# DASM Assembler

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**DASM Assembler** is the official assembler for the DCPU architecture, which you can find more about [here](https://github.com/DavidMik08/DCPU-emulator).
**Our goal** is for the user to be able to program using **words** instead of hex codes and to make it as user-friendly as possible while being as **low-level** as possible. 
   
## Vision

This project aims to:

* Design a **custom CPU** with a unique instruction set
* Provide a **two-pass assembler** to translate human-readable assembly code into machine code
* Develop a **custom operating system** that loads programs from an external memory and handles I/O
* Enable experimentation with **low-level programming, CPU design, and OS development**

This is intended for enthusiasts, learners, and anyone interested in exploring **hardware-software co-design**.

## Current Status

### DASM Assembler

* Two-pass assembler written in **Rust**
* Supports labels, instruction modifiers, immediate values, and data directives
* Outputs compact hexadecimal `.hex` files suitable for custom CPU execution

### CPU Design

* Instruction set architecture (ISA) defined
* Fixed 4-byte instruction format
* Basic ALU, branch, and control operations supported

**Instruction Format (4 bytes):**
| Command | Input1 | Input2 | Output |
|:-:|:-:|:-:|:-:|
| 8 bits | 8 bits | 8 bits | 8 bits |

## Features of the Assembler

* Label resolution and automatic address calculation
* Instruction modifiers for flags
* Immediate value detection and encoding
* Data directives for raw bytes and words
* Little-endian output format
* Compact hex output for loading on custom CPU

## Building the Assembler

Make sure you have **Rust** installed.

```bash
cargo build
```

The compiled binary will be located at:

```text
target/release/DASM_assembler
```

You can run the binary with: 

```bash
./DASM_assembler program.asm -o out.hex
```

## Usage

```bash
cargo run -- <input.asm> [-o output.hex]
```

## Examples

The examples are located in the ```/target/debug/examples```

### Example:
```asm
; this is an example of an for loop in DASM
; the c equivalent would be something like this:
; int i;
; for(i = 0; i<10; i = i+1) {}

add 0 0 r1

loop_start:
  ; compare r1 to 10
  sub r1 10 r0
  ; set the address
  add_igf loop_end.Low 0 r3
  add_igf loop_end.Mid 0 r4
  add_igf loop_end.High 0 r5
  ; branch to the end if r1 == 10
  biz 0 0 r0

  ; this is where you can put code that will run inside of the loop

  ; increment r1 (i)
  add r1 1 r1

  ; branch back to the start of the loop
  add 0 0 r0
  add_igf loop_start.Low 0 r3
  add_igf loop_start.Mid 0 r4
  add_igf loop_start.High 0 r5
  biz 0 0 r0
```

## License

This project is licensed under the **MIT License**. See [LICENSE](LICENSE) for details.
