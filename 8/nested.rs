fn main() {
  'outer: loop {
    println!("outer start");
    'inner: loop {
      println!("inner start");
      break 'outer;
      println!("inner end");
    }
    println!("outer end");
  }

  println!("exited");
}