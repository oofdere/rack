# rack
(early stages) the nand2tetris toolchain, but it's rewritten in rust

## project status
right now I'm just trying to get through part 2 of nand2tetris, but if I'm going to rewrite all the tools anyway, might as well do it in a way where it cn act as a replacement of the official Java tools, which don't even do DPI scaling

 - [ ] hack assembler
   - [x] file input
   - [x] A-instruction parsing
   - [x] C-instruction parsing
   - [ ] Labels
   - [ ] Symbols
   - [ ] file output
   - [ ] debug symbols (far future)
 - [ ] hack disassembler
 - [ ] jack compiler
 - [ ] OS
 - [ ] ROM/program loader (like those knockoff 5 trillion in one famicom carts)
 - [ ] VM
 - [ ] hack debugger
 - [ ] GUI
 - [ ] WebAssembly
 - [ ] unit tests