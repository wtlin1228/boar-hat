# Compilers

[StanfordOnline SOE.YCSCS1 - Compilers](https://learning.edx.org/course/course-v1:StanfordOnline+SOE.YCSCS1+3T2020/home)

Design and build a compiler for Cool. Each assignment will cover one component of the compiler:

- lexical analysis
- parsing
- semantic analysis
- code generation

## PA2 - Lexical Analyzer

Grading: 63/63

- [cool.flex](PA2/cool.flex)

### Resource

- [Lexical Analysis With Flex, for Flex 2.6.2](https://westes.github.io/flex/manual/)

## PA3 - Parser

Grading: 70/70

- [cool.y](PA3/cool.y)

### Resource

- [Bison 2.4.1](https://www.cin.ufpe.br/~frsn/arquivos/GnuWin32/doc/bison/2.4.1/bison-2.4.1/bison.html#Location-Default-Action)
- [The Cool Reference Manual, Chapter 12, Figure 1: Cool syntax](https://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf)

## PA4 - Semantic Analyzer

Grading: 74/74

- [cool-tree.h](PA4/cool-tree.h)
- [semant.cc](PA4/semant.cc)
- [semant.h](PA4/semant.h)

### Resource

- [The Cool Reference Manual, Chapter 12, Type Rules](https://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf)

## PA5 - Code Generator

Grading: 63/63

- [cgen-context.h](PA5/cgen-context.h)
- [cgen.cc](PA5/cgen.cc)
- [cgen.h](PA5/cgen.h)
- [cool-tree.handcode.h](PA5/cool-tree.handcode.h)
- [emit.h](PA5/emit.h)

### Resource

- [The Cool Reference Manual, Chapter 13, Operational Semantics](https://theory.stanford.edu/~aiken/software/cool/cool-manual.pdf)
- [The Cool Runtime System](https://web.stanford.edu/class/cs143/materials/cool-runtime.pdf)
- [MIPS architecture](https://en.wikipedia.org/wiki/MIPS_architecture)
