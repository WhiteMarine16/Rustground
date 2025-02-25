fn main() {
  print!("My name is {0}, {1} {0}", "Bond", "James");
  println!("");
// This will not compile because `Structure` does not implement
// fmt::Display.
//println!("This struct `{}` won't print...", Structure(3));
   let pi = 3.141592;
   println!("Pi is roughly '{pi:.*}'", 3);
}
