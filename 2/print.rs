fn main() {
  print!("Apples and ");
  println!("oranges.");

  println!("Stringified: {}",3i);

  println!("Positioned: {0} {0} {0} your {1}, gently down the {2}.", "row", "boat", "stream");

  println!("Named arguments: {name} {age}", name="Bob", age=37i);
  println!("Argument types: LowerHex {0:x}, Binary {1:b}", 1234i, 5i);
}