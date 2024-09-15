# Dust

Interpreted scripting language

## Building
Make sure that Rust is installed (https://www.rust-lang.org), then inside the cloned repository use `cargo` to build it.

```
cargo build --release
```

You can then run the project with cargo.
```
cargo run --release -- input.txt
```

Or find the compiled executable directly in the project folder on the following path:
```
./target/release/dust
```

### Usage

#### Using variables

You can initialize variables with the `let` keyword, and optionally annotate their types. If there is no annotation the variable will receive the type of the value on the right hand side.

```
let x: number = 0

let x, y, z = 0
```

If the type on the right hand side contradicts the annotated type, then a runtime error will occur.

```
let x: int = true // Runtime error
```

Types assigned this way are static and the variable cannot be assigned a value of a different type later on. If you need a dynamic variable whose type can be changed, you must explicitly annotate it with the `dyn` keyword.

```
let x: dyn = 1

let x: int, y: dyn = 2
```

Once a variable is initialized, you can reassing its value with the assignment operator as long as the previously discussed type constraints are satisfied.

```
x = 42

y = x + 1
```

The following primitive types are available:

```
none
int
float
string
bool
```

#### Branches

You can use the `if` keyword with a control expression for conditional execution. If the expression evaluates to `false`, the block will not run.

The `else if` keyword allows you to test more conditions only in case the previous ones have evaluated to `false`.

You can also use the `else` keyword to specify a block to be executed only in case all of the control expressions evaluated to `false`.

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

You can use the `while` keyword with an expression for conditional loops. The loop will continue to run as long as the expression evaluates to `true`. Alternatively you can use the `break` keyword to exit the loop at an arbitrary point.

```
let x = 0
while x < 10 {
    x = x + 1
    if x == 7 {
        break
    }
}
```

You can also use a `for` loop to iterate over the elements of a collection. Currently the only supported type is the builtin `Vec` dynamically sized array.

```
let sum: int = 0;
for i in range(1, 10) {
    sum = sum + i;
}
println(sum);
```

#### Defining functions

You can define functions using the `fn` keyword. You must annotate the types of function parameters. If the function returns a value then the type of that return value must also be annotated.

```
fn power(base: int, exponent: int) -> int {
    if (exponent == 0) {
      return 1
    }
    return base * power(base, exponent - 1)
}
```

#### Classes

You can define classes that contain properties and methods. By default these are all private and you need to mark public ones with the `pub` keyword. All properties must be explicitly initialized with an expression.

```
class Person {
    pub name: string = "";
    pub age: int = 0;
    pub address: string = "";
}
```

Classes can only have a single constructor. The parameters are specified after the name of the class. Expressions used to initialize properties can access these arguments.

```
class Person(name: string, age: int, address: string) {
    name: string = name;
    age: int = age;
    address: string = address;
}
```

The constructor is an automatically generated associated function called `new`. Functions that use `self` as the first argument are member functions and functions defined within a class that do not use `self` are associated functions. Only member functions can opearte with the properties of a class instance. Member functions are called with the member access operator `.` after the concrete instance variable and associated functions are called with the scope resolution operator `::` after the name of the class.

```
class Person(name: string, age: int, address: string) {
    name: string = name;
    age: int = age;
    address: string = address;
    
    pub fn say_hello(self) {
        println("Hello, my name is " + self.name + ".");
    }
    
    pub fn print_species() {
        println("Human");
    }
}

let p = Person::new("Anonymous", 100, "Nowhere");
p.say_hello();
Person::print_species();
```

#### Error handling

The builtin type for error handling is the `Result` type. It consists of two properties, one `bool` type that represents the state and a dynamic tpye that contains data associated with the state. On success this could be the result of the operation and on failure it could contain extra information about what went wrong.

You can check the state of a `Result` with the `is_ok()` method. After you have done so, you can access the value with the `value()` method, but if you try to call `value()` on a `Result` instance that never had `is_ok()` called on it before you will get a runtime error. If you are sure that the result can only be a success you can use the `unwrap()` method that will return the contained value anyway, but you will get an error if the state of the `Result` instance is `false`.

```
let n = float::parse(input("> ").trim());

if n.is_ok() {
    println(n.value() * 2);
} else {
    println("Error parsing float: " + n.value());
}

let n = int::parse("42");
println(n.unwrap() * 2);
```

#### Miscellaneous

Supported operators: `+`, `-`, `/`, `*`, `^`, `%`, `<`, `>`, `==`, `!=`, `=`, `and`, `or`, `not`, `typeof`

See `Builtin.md` for a full list of builtin functions and classes.

In the command line interpreter:

You can use `\` at the end of a line for multiline input.  
Input `clear` to clear the screen.  
Input `reset` to delete all variables along with function and class definitions.
