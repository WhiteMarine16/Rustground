Printing is handled by a series of macros defined in std::fmt some of which are:

- format!: write formatted text to String
- print!: same as format! but the text is printed to the console (io::stdout).
- println!: same as print! but a newline is appended.
- eprint!: same as print! but the text is printed to the standard error (io::stderr).
- eprintln!: same as eprint! but a newline is appended.
All parse text in the same fashion. As a plus, Rust checks formatting correctness at compile time.

In general, the `{}` will be automatically replaced with any arguments. These will be stringified.
```rust
fn main() {
println!("{} days", 31);
}
```
Positional arguments can be used. Specifying an integer inside `{}`determines which additional argument will be replaced. Arguments start at 0 immediately after the format string.
```rust
fn main() {
println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
}
```
As can named arguments.
```rust
fn main() {
println!("{subject} {verb} {object}",
  object="the lazy dog",
  subject="the quick brown fox",
  verb="jumps over");
}
```
Different formatting can be invoked by specifying the format character after a `:`.
```rust
fn main() {
println!("Base 10:               {}",   69420); // 69420
println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
println!("Base 8 (octal):        {:o}", 69420); // 207454
println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
}
```
You can right-justify text with a specified width. 
This will output "    1". (Four white spaces and a "1", for a total width of 5.)
```rust
fn main() {
println!("{number:>5}", number=1);
}
```
You can pad numbers with extra zeroes
```rust
fn main() {
println!("{number:0>5}", number=1); // 00001
}
```
And left-adjust by flipping the sign. This will output "10000".
```rust
fn main() {
println!("{number:0<5}", number=1); // 10000 
}
```
Only types that implement fmt::Display can be formatted with `{}`. User-defined types do not implement fmt::Display by default.
```rust
fn main() {
#[allow(dead_code)] // disable `dead_code` which warn against unused module
struct Structure(i32);
}
```
For Rust 1.58 and above, you can directly capture the argument from a surrounding variable. 
```rust
fn main() {
//Just like the above, this will output "    1", 4 white spaces and a "1".
let number: f64 = 1.0;
let width: usize = 5;
println!("{number:>width$}");
}
```
std::fmt contains many traits which govern the display of text. The base form of two important ones are listed below:

- fmt::Debug: Uses the {:?} marker. Format text for debugging purposes.
- fmt::Display: Uses the {} marker. Format text in a more elegant, user friendly fashion.

Here, we used fmt::Display because the std library provides implementations for these types. To print text for custom types, more steps are required.

Implementing the fmt::Display trait automatically implements the ToString trait which allows us to convert the type to String.

In line 43, #[allow(dead_code)] is an attribute which only applies to the module after it.
