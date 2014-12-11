fn main() {
  //let long_lived_var = 1i;
  let i = 1i;

  { //block start
    let i = 2i;

    println!("inner: {0}", i);
  } //block end

  println!("outer: {0}", i);
}