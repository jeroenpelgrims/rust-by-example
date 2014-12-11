fn main() {
  println!("1 + 2 = {}", 1u + 2);
  println!("1i - 2 = {}", 1i - 2);
  println!("1u - 2 = {}", 1u - 2);

  println!("{0}", !true);

  println!("{:b}", 0b0100u - 0b1000);
  println!("{:b}", 0b10i << 1);
}