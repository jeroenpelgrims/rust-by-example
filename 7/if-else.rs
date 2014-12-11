fn main() {
  let x = 5i;

  let x_is =
    if x < 0 {
      "negative"
    } else if x > 0 {
      "positive"
    } else {
      "zero"
    };

  println!("{}", x_is);
}