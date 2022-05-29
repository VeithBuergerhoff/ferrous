# ferrous
Ferrous is a programming language, meant to be simple to understand and use, with inspiration from both Rust and C#.
The main purpose of this project is to get a feel for rust and to satisfy my desire of writing my own compiler.
I learned the basics of Rust in three days and simply looked at other projects for how to structure my code.

The basic project structure and the structure of the lexer was heavily inspired by [the official rust repository](https://github.com/rust-lang/rust). 

Roadmap (not ordered):
- [x] Build a [Lexer](https://en.wikipedia.org/wiki/Lexical_analysis)
- [ ] Build a [Parser](https://en.wikipedia.org/wiki/Parsing) (WIP)
- [ ] Build a [Semantic Analyser](https://en.wikipedia.org/wiki/Semantic_analysis_(compilers))
- [ ] Build an [Optimizer](https://en.wikipedia.org/wiki/Optimizing_compiler)
- [ ] Emit [LLVM IL](https://llvm.org/docs/LangRef.html)
- [ ] Emit [Cranelift IL](https://github.com/bytecodealliance/wasmtime/tree/main/cranelift)
- [ ] Emit custom assembly code similar to [Java bytecode](https://en.wikipedia.org/wiki/Java_bytecode)
- [ ] Build an interpreter
- [ ] Build a debugger
- [ ] Build a language server implementing the [Language Server Protocol](https://en.wikipedia.org/wiki/Language_Server_Protocol) for intellisense
- [ ] Build a [VS Code](https://code.visualstudio.com/) extension as a bridge between VS Code and the language server
- [ ] Build a web-based editor that uses the language server for intellisense. Possibly using [Rocket](https://rocket.rs/) and/or webassembly
- [ ] Build something similar to [JavaKara](https://www.swisseduc.ch/informatik/karatojava/javakara) based on the editor for teaching programming, possibly using [WebGL](https://en.wikipedia.org/wiki/WebGL)

# Contributing
If you spot mistakes, bugs, bad practice, or simply want to help out, feel free to open an Issue. 
Pull requests are also welcome, but do try to keep them short.

# Additionally
I did think about streaming, but I'm quite shy and English is not my first language (although I am quite proficient.)
If I get a sufficient amount of requests to stream, I might reconsider.