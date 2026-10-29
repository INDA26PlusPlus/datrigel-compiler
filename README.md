# datrigel-compiler

## Quick guide:
SMN or Simple math notation, is my programming language and now compiler.

The compiler compiles the contants of "test_src_code.txt", by editing the file, other programs can be written. It is run buy running the executable inside the src folder, thereby it will fin "test_src_code.txt".

Moreover the grammer is specified in gramar.md, aswell as a outline of how i intend to implement fibonacci calculations.

## Quick guide to compiling to qbe.
Running the file (Descirbed in the previous guide) will compile the "fibonacci_src_code.txt" into QBE code. That code will be printed in the terminal.

``` bash
cargo run > temp && qbe -o out.s temp && cc out.s && rm temp && rm out.s
````
The terminal command can be used to compile from SMN directly into a executable.

The particular fibonacci number is chosen by changing the "ntimes" variable in the SMN source code.