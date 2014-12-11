fn main() {
  let immutable = 1i;
  let mut mutable = 1i;

  println!("before: {0}", mutable);
  mutable += 1;
  println!("after: {0}", mutable);

  immutable += 1;
}