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
Fpwm is programmed in a primarily procedural paradigm. The reason for this is
that if the window manager is represented with a class, only one instance would
be constucted, which when you tack a step back and look at it, can be
represented with procedural programming. On top of that, the xlib interface was
written for C and as such is procedural in nature, making it naturally fit into
a procedural paradigm. Usually you could wrap the xlib interface into a class,
however, to detect whether or not another wm is already running, global state is
required as the signature of the error handler function is predetermined by
xlib.

Thus it was decided that fpwm would be primarily written in a procedural way.

This does not mean OOP is not used in fpwm. For example, the config loader is
written in an weak OOP paradigm. It might not strictly follow the rules of OOP,
instead opting to just wrap the grouped functionality and data in a class.

To try to make it clear what must be procedural and what can be another
paradigm here is the core rule: The window manager itself will be written in a
procedural format but any other parts of the code base can be in another
paradigm, preferably OOP. Examples of code that doesnt need to be procedural
would be the above mentioned config loader as it doesnt directly interact with
xlib.

### Columns
To keep things consistant, all code must fit within 80 character columns. This
includes comments as well.

### Name Scheme
Names should follow the standard guidelines set in place by Cargo. Of course you
cant control external crates so if the external crate goes against the
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