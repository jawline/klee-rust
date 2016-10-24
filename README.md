# klee-rust
A interface for using KLEE and Rust together

Note: This is just a klee C binding. Rust uses its own LLVM, and in order for this to work a version of KLEE must be compiled with the same LLVM. I haven't supported/looked at this work in a while and I'm unsure how involved this process is now.

#Dependancies
- KLEE built with Rust LLVM
- Rust
