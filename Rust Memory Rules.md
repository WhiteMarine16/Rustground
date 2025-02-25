# Rust Memory Rules
1. Each **Value** can only have **one owner**. 
```rust
fn print_string(x: String){
    print!("{}", x);
}

fn main(){
    let name: String = String::from("Chris");

    print_string(name);
}
```
2. When **the owner** goes out of scope the **value** is **droped** from **memory**

3. While Values cannot have multiple owners
```rust
fn print_string(x: String){
    print!("{}", x);
}

fn main(){
    let name: String = String::from("Chris");

    print_string(name);
    print_string(name);
}
```
Functions and other calls can borrow references to values which belong to other owners
```rust
fn print_string(x: &String){
    print!("{}", x);
} 

fn main(){
    let name: String = String::from("Chris");

    print_string(&name);
}
```

# All Variables Are Immutable By Default In Rust 
To mututate a variable you have to explicity marked as **Mutable** with the **keyword: "mut"**   
 ```rust
fn print_string(x: &mut String){
    print!("{}", x);
} 

fn main(){
    let mut name: String = String::from("Chris");

    print_string(&mut name);
    print_string(&mut name);
}
```

 | JS | Rust |
| :-----| :-------| 
| const | let     | 
| let   | let mut |

# Has the **mut keyword**?
## yes
```rust
fn main() {

}
```
## no
```rust
fn main() {

}
```
