# MFL

This is my first language. The idea is to start out with just integers, and implement a compiler using LLVM for something like a very small subset of rust.

## Syntax

We will start with only having functions, integers called `int` but actually being like rust's `i32` (hence represented in LLVM IR with i32's and signed operations), and let statements and mutation statements (same syntax as let statements without the "let"), if statements/expressions, simple while loops (for loops can't exist in the way I would want them without other types), and comments for convenience. Type annotations will be enforced for practice, but ignored (there's only `int`). The idea is that the lexer I build for this should need minimal adjustments for future compiler projects.

let statements:
```
let x: int = 2;  // standard representation
let   x   : int  // though all whitespace will 
 =2;             // be treated as equivalent
```

if statements/expressions:
```
// expression
if (x < y) {
    x
} else {
    y
}

// statement
if (x < y) {
    println(x);
} else {
    println(y);
}
```
There are no macros, so everything is just written as is. Note that since `bool` does not exist, we start out with if statements/expressions just checking whether the int not 0. which means `<` is a function `int -> int -> int`.

while loops:
```
while (x < y) {
    x = x + 1; // mutation statement — x already defined
}
```

functions:
```
fn less_than(x: int, y: int) -> int {
    x < y
}
```
