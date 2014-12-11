fn main() {
  let mut i = 1u;

  while i < 101 {
    if i % 3 == 0 {
      print!("Fizz");
    }
    if i % 5 == 0 {
      print!("Buzz");
    }
    if i % 3 > 0 && i % 5 > 0 {
      print!("{}", i);
    }

    println!("");
    i += 1;
  }
}