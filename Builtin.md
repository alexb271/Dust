# Builtin functions and classes

## Functions

#### println(dyn)
Print the value of the argument to the standard output followed by a new line

#### print(dyn)
Print the value of the argument to the standard output

#### input(string) -> string
Display a prompt on the standard output then read user input

#### rand(int, int) -> int
Generate a random number within a range (inclusive)

#### range(int, int) -> Vec
Create a Vec containing numbers within a range (exclusive)

#### panic(string)
Generate a runtime error with a given message

## int
Represents an integer

#### int::parse(string) -> Result
Parse an integer from a string

#### abs(self) -> int
Get the absolute value of an integer

#### to_float(self) -> float
Get the float representation of an integer

#### to_string(self) -> string
Get the string representation of an integer

## float
Represents a floating point number

#### float::parse(string) -> Result
Parse a float from a string

#### float::pi() -> float
Get an accurate float representation of pi

#### abs(self) -> int
Get the absolute value of a float

#### is_nan(self) -> bool
Returns true if the float has NaN value

#### to_int(self) -> int
Get the int representation of a float after rounding

#### to_string(self) -> string
Get the string representation of a float

## string
Represents text

#### join(self, string)
Append the argument to the string

#### starts_with(self, string) -> bool
Returns true if the string starts the same as the argument

#### ends_with(self, string) -> bool
Returns true if the string ends the same as the argument

#### contains(self, string) -> bool
Returns true if the string contains the argument

#### pop(self) -> dyn
Remove the final character and returns it, or none if empty

#### trim(self) -> string
Returns a new string based on itself that has trimmed whitespaces

#### to_lowercase(self) -> string
Returns a new string based on itself that is all lowercase

#### to_uppercase(self) -> string
Returns a new string based on itself that is all uppercase

#### len(self) -> int
Returns the number of characters in the string

#### chars(self) -> Vec
Returns a Vec containing a string for each character in the original

#### substring(self, int, int) -> string
Returns the substring in the given range (exclusive), or none

#### split(self, string) -> Vec
Split the string by a single character delimiter string and return the parts in a Vec of strings

## bool
Represents a logical boolean

#### bool::parse(string) -> Result
Parse a bool from a string

#### to_string(self) -> string
Get the string representation of a bool

## Vec
A dynamically sized array

#### Vec::new() -> Vec
Create a new Vec

#### push(self, dyn)
Push an item onto the end of the Vec

#### get(self, int) -> dyn
Get the item at the given index

#### set(self, int, dyn)
Set the item at the given index to the value provided

#### remove(self, int) -> dyn
Remove the item at the given index and return it

#### pop(self) -> dyn
Remove the last item and return it

#### clear(self)
Delete all items in the Vec

#### len(self) -> int
Returns the number of items in the Vec

## Result
Represents the outcome of some operation with associated additional data

#### Result::new(bool, dyn) -> Result
Create a new Result with the given state and value

#### is_ok(self) -> bool
Returns true if the Result is good, false otherwise

#### value(self) -> dyn
Returns the value stored in the Result if is_ok() has already been called on it, otherwise creates a runtime error

#### unwrap(self) -> dyn
Returns the value stored in the Result regardless whether is_ok() has already been called on it but creates a runtime error if the Result's state is 'false'

## File
Class with associated file related methods, cannot be instanced

#### File::read(string) -> Result
Read the contents of a file into a string

#### File::write(string, string) -> Result
Write to the file at the given path the contents passed as argument

#### File::append(string, string) -> Result
Append to the file at the given path the contents passed as argument

## Fs
Class with associated file system related methods, cannot be instanced

#### Fs::current_directory() -> Result
Get the current directory as a string on success

#### Fs::change_directory(string) -> Result
Change the current working directory

#### Fs::exists(string) -> bool
Check if the target of the path given as argument exists

#### Fs::list(string) -> Result
Get the contents of a directory as a Vec of strings on success

#### Fs::remove_file(string) -> Result
Delete a file at the given path

#### Fs::remove_directory(string) -> Result
Recursively delete a directory at the given path with all of its contents

#### Fs::create_directory(string) -> Result
Create a new directory at the given path

#### Fs::copy(string, string)
Copy the file at the provided source path to the destination path

#### Fs::move(string, string)
Move the file at the provided source path to the destination path

## Math
Class with associated math related methods, cannot be instanced

#### Math::sin(float) -> float
Sine function in radians

#### Math::sind(float) -> float
Sine function in degrees

#### Math::asin(float) -> float
Arcus sine function in radians

#### Math::asind(float) -> float
Arcus sine function in degrees

#### Math::cos(float) -> float
Cosine function in radians

#### Math::cosd(float) -> float
Cosine function in degrees

#### Math::acos(float) -> float
Arcus cosine function in radians

#### Math::acosd(float) -> float
Arcus cosine function in degrees

#### Math::tan(float) -> float
Tangent function in radians

#### Math::tand(float) -> float
Tangent function in degrees

#### Math::atan(float) -> float
Arcus tangent function in radians

#### Math::atand(float) -> float
Arcus tangent function in degrees

#### Math::ln(float) -> float
Natural logarithm

#### Math::log(float) -> float
Decimal logarithm
