# FPWM Code Style Document
## Preface
This document details a lot of the nuances of the code of fpwm. If you want to
contribute please make a fork as this is more of a personal project for me that
I want to share. The style document is here for if I ever open this up to
contributions or if you want to keep code style consistent for your fork.

## Design Choices
This is a list of some decisions made while creating FPWM and the reasoning
behind them

### Paradigm
Fpwm is programmed in a loose OOP paradigm. The reason for this is that due to
how the Xlib library is written fpwm requires global state which does not fit
into a strictly OOP format.

### Columns
It is recommended to keep all code and comments within 80 columns, but it is
not required

### Name Scheme
Names should follow the standard guidelines set in place by Cargo. Of course,
you cant control external crates so if the external crate goes against the
guidelines, that is fine.

### Function Arguments and Parameters
Function arguments/parameters should be inline with the function name, unless
this causes you to break the 80 character column rule, in which case you should
put the starting parenthesis of the argument/parameter list on a newline with
each argument/parameter getting its own line. Each parameter should be indented
by  1 level. The ending parenthesis should also be put on a seperate line from
the last argument/parameter and at the same indentation level as the line with
the function name. There should not be a space between the function name and the
starting parenthesis.

### Starting Braces
Starting braces for structs and functions should be inline with the struct name
or the ending parenthesis of the function.

### Scope Spacing
In a new scope, you must have an empty newline at the begining and end of the
scope, this keeps code spaced out and easily readable.

These are examples for the last 3 sections:
```rust
fn my_func(arg1: type1, arg2: type2, arg3: type3) -> returnType {
    
    // Your code starts on this line

    // and ends on this line

}



fn my_func(
    arg_0: Type1,
    arg_1: Type2,
    arg_2: Type3
) -> ReturnType {
    
    // Your code starts on this line

    // and ends on this line

}
```

### Unsafe Blocks
If you have an unsafe block in a function and it wraps everything in the
function you can ignore the scope spacing rule for the unsafe block

An example would be:
```rust
fn my_func() {
    unsafe {

        // Your code starts on this line

        // and ends on this line

    }
}
```