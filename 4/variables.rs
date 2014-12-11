fn main() {
  let i = 1u;
  let b = true;
  let unit = ();
  let copied_i = i;

  println!("{}", copied_i);
  println!("{}", b);
  println!("{}", unit);

  let _unused = "abc"; //no warning because of leading underscore
}