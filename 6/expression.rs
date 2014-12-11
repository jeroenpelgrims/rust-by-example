fn main() {
  let y = {
    let x = 5i;
    let y = 6i;

    x * y
  };

  println!("{}", y);

  let unit = { 5i; };
  println!("{}", unit);
}