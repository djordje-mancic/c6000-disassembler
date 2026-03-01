# c6000-disassembler
Disassembler for the Texas Instruments TMS320 C6000 series of Digital Signal Processors. At the moment, this includes:
- TMS320C62x
- TMS320C64x
- TMS320C64x+

The disassembler mostly tries to follow the same syntax used in Texas Instruments' Code Composer Studio, with some exceptions where changes have been made for improved readability.

## Usage
You can use the disassembler either as a library or as a standalone program.
### Standalone Program
The command format is `c6000-disassembler [OPTIONS] <FILE>`

**Example:** ``c6000-disassembler CODE.bin``

All of the available options can be printed with ``c6000-disassembler --help``

### Library
You can add the disassembler as a dependency by running ``cargo add c6000-disassembler``

## Building
To build the disassembler, you will need to have Cargo (the Rust package manager) installed. 

Afterwards, you should be able to build by running ``cargo build`` in the main directory.