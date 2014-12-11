fn main() {
  for i in range(1u, 101) { //range end is exclusive
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
  }
}