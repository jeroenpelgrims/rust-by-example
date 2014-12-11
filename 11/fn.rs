fn main() {
  fizzbuzz(100);
}

fn is_double_multiple(x: uint) -> bool {
  x % 3 == 0 && x % 5 == 0
}

fn fizzbuzz(n: uint) {
  for i in range(1, n + 1) {
    if is_double_multiple(i) {
      println!("FizzBuzz");
    } else if i % 3 == 0 {
      println!("Fizz");
    } else if i % 5 == 0 { 
      println!("Buzz");
    } else {
      println!("{}", i);
    }
  }
}