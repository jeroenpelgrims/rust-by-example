fn main() {
  let mut count = 0u;

  //println!("");
  loop {
    count += 1;

    if count % 3 == 0 {
      print!("Fizz");
    }
    if count % 5 == 0 {
      print!("Buzz");
    }

    println!("");

    if count > 100 {
      break;
    }
  }
}