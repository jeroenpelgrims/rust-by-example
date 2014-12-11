fn main() {
  // Suffixed literals. types known
  let x = 1u8;
  let y = 2u;
  let z = 3f32;

  // Unsuffixed literals. Type depends on usage
  let a = 1;
  let b = 1.0;

  println!("{}", x * a);
  println!("{}", z * b);

  println!("sixe of y: {}", std::mem::size_of_val(&y));
}