fn matcher(num: i32) -> &'static str {
  match num {
    1 => "one",
    2 | 3 | 5 | 7 | 11 => "prime",
    13...19 => "teen",
    _ => "something else"
  }
}

fn main() {
  println!("{}", matcher(1));
  println!("{}", matcher(3));
  println!("{}", matcher(15));
  println!("{}", matcher(20));

  
}