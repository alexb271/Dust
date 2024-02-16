# Dust

Prototype scripting language WIP

## Building

```
cargo build
```

### Usage

#### Using variables

You can initialize variables with the
`let` keyword, and optionally annotate
their types. If there is no annotation
the variable will receive the type of the
value on the right hand side.

```
let x: number = 0

let x, y, z = 0
```

If the type on the right hand side
contradicts the annotated type, then a
runtime error will occur.

```
let x: number = true // Runtime error
```

Types assigned this way are static and the
variable cannot be assigned a value of a
different type later on. If you need a
dynamic variable whose type can be changed,
you must explicitly annotate it with the
`dyn` keyword.

```
let x: dyn = 1

let x: number, y: dyn = 2
```

Once a variable is initialized, you can reassing
its value with the assignment operator as long as
the previously discussed type constraints are
satisfied.

```
x = 42

y = x + 1
```

The following built-in types are available:

```
none
number
string
bool
```

#### Defining functions

You can define functions using the `fn` keyword.
You must annotate the types of function parameters.
Optionally you can also annotate the type of the
function's return value.

```
fn power(base: number, exponent: number) -> number {
    if (exponent == 0) {
      return 1
    }
    return base * power(base, exponent - 1)
}
```

#### Built-in Functions:

The following built-in mathematical functions are available:
`sin`, `cos`, `tan`, `ln`, `log`, `abs`

For trigonometric functions prepend 'a' for arcus
and append 'd' for degree.

Use `pi` for an accurate value of the constant.
For example:

```
sin(pi)
asind(0.5)
```

Other built-in functions:

```
rand(min: number, max: number) -> number
parse_number(input: string) -> number
is_nan(input: number) -> bool
input(prompt: string) -> string
to_string(input: dyn) -> string
```

#### Branches

You can use the `if` keyword with a control expression
for conditional execution. If the expression evaluates
to `false`, the block will not run.

The `else if` keyword allows you to test more conditions
only in case the previous ones have evaluated to `false`.

You can also use the `else` keyword to specify a block
to be executed only in case all of the control expressions
evaluated to `false`.

```
if x % 2 == 0 {
    println(x)
} else if x < 0 {
    println(-x)
} else {
    println(x + 1)
}
```

#### Loops

You can use the `while` keyword with an expression for
conditional loops. The loop will continue to run as long
as the expression evaluates to `true`.
Alternatively you can use the `break` keyword to exit
the loop at an arbitrary point.

```
let x = 0
while x < 10 {
    x = x + 1
    if x == 7 {
        break
    }
}
```

#### Printing

You can use the `print` and `println` built-in functions
to print to the standard output.

```
let x = 0
while x < 5 {
    println(x * 2)
    x = x + 1
}
```

#### Miscellaneous

Supported operators: `+`, `-`, `/`, `*`, `^`, `%`, `<`, `>`, `==`, `!=`, `=`, `and`, `or`, `not`, `typeof`  
You can use `\\` at the end of a line for multiline input.  
Input `clear` to clear the screen.  
Input `reset` to delete all functions and variables.  
