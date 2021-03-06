# RUSTY CHIP8 

**Rusty CHIP-8** is a CHIP-8 emulator/interpreter written using the Rust programming language.

I have implemented this emulator based on the excellent blog post by Laurence Muller titled [How to write an emulator (CHIP-8 interpreter)](http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/).


## What is it?

CHIP-8 is a virtual machine (VM) developed in the 70'S by Josheph Weisbecker. Games written in the Chip-8 programming language could run on systems that had a Chip-8 interpreter.

As an emulator, Rusty CHIP-8 is a computer program that mimics the internal design and functionality of a computer system. Therefore, it can run software designed for the mimicked computer system.

As an interpretor, it understands the emulated system, and knows what to do when it encouters instructions from that system.

Games written for CHIP-8 can easily be found on the Internet by just searching using your favourite search engine.

## Documentation

### Opcode
CHIP-8 has 35 opcodes, which are all two bytes long and stored Big-ending. The table below list the opcodes in hexadecimal. The symbols in the Opcode column can be interpreted as follows:

* NNN: address
* NN: 8-bit constant
* N: 4-bit constant
* X and Y: 4-bit register identifier

Of these opcodes, 30/35 are currently implemented in Rusty CHIP-8 (Opcodes in bold are the ones that need to be implemented). 

| Opcode | Explanation |
-------- | -------------
| **0NNN** | **Calls RCA 1802 program at address NNN. Not necessary for most ROMs.** |
| 00E0 | Clears the screen. |
| 00EE | Returns from a subroutine. |
| 1NNN | Jumps to address NNN. |
| 2NNN | Calls subroutine at NNN. |
| 3XNN | Skips the next instruction if VX equals NN. |
| 4XNN | Skips the next instruction if VX doesn't equal NN. |
| 5XY0 | Skips the next instruction if VX equals VY. |
| 6XNN | Sets VX to NN. |
| 7XNN | Adds NN to VX. |
| 8XY0 | Sets VX to the value of VY. |
| 8XY1 | Sets VX to VX or VY. |
| 8XY2 | Sets VX to VX and VY. |
| 8XY3 | Sets VX to VX xor VY. |
| 8XY4 | Adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't. |
| **8XY5** | **VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't.** |
| 8XY6 | Shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift. |
| **8XY7** | **Sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't.** |
| 8XYE | Shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift.|
| 9XY0 | Skips the next instruction if VX doesn't equal VY. |
| ANNN | Sets I to the address NNN. |
| BNNN | Jumps to the address NNN plus V0. |
| CXNN | Sets VX to the result of a bitwise and operation on a random number and NN. |
| DXYN | Sprites stored in memory at location in index register (I), 8bits wide. Wraps around the screen. If when drawn, clears a pixel, register VF is set to 1 otherwise it is zero. All drawing is XOR drawing (i.e. it toggles the screen pixels). Sprites are drawn starting at position VX, VY. N is the number of 8bit rows that need to be drawn. If N is greater than 1, second line continues at position VX, VY+1, and so on. |
| EX9E | Skips the next instruction if the key stored in VX is pressed. |
| EXA1 | Skips the next instruction if the key stored in VX isn't pressed. |
| FX07 | Sets VX to the value of the delay timer. |
| **FX0A** | **A key press is awaited, and then stored in VX.** |
| FX15 | Sets the delay timer to VX. |
| FX18 | Sets the sound timer to VX. |
| FX1E | Adds VX to I.</sup> |
| **FX29** | **Sets I to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font.** |
| FX33 | Stores the Binary-coded decimal representation of VX, with the most significant of three digits at the address in I, the middle digit at I plus 1, and the least significant digit at I plus 2. (In other words, take the decimal representation of VX, place the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.) |
| FX55 | Stores V0 to VX in memory starting at address I.</sup> |
| FX65 | Fills V0 to VX with values from memory starting at address I.</sup> |

More information about CHIP-8 can be found on [Wikipedia](https://en.wikipedia.org/wiki/CHIP-8).

## Installation

Since the program is provided as source, you will need to compile it to your target platform. Therefore, you need to have a fully working installation of [Rust](http://www.rust-lang.org/). Once the program is compiled, you can run it from the command prompt without the need to install it. See the next section for usage instructions.

## Usage

Rusty CHIP-8 reads an input file containing the game to be run on the virtual machine. To launch a game, type the following instruction from the command prompt.

```
prompt> chip8 [PATH_TO_CHIP8_GAME_FILE]
```

## Licensing

Please see the file called (LICENSE)[chip8/LICENSE.md].

## Contacts
