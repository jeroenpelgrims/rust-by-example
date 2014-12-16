fn reverse(pair: (int, bool)) -> (bool, int) {
  let (i, b) = pair;
  
  (b, i)
}

fn main() {
  let long_tuple = (1u8, 2u16, 3u32, 4u64, "test");

  println!("{}", long_tuple);
  println!("{}", long_tuple.0);
  println!("{}", long_tuple.4);

  let tuple_of_tuples = ((1i, 2i), (3i, 4i, 5i));
  println!("{}", tuple_of_tuples);

  let pair = (1, true);
  println!("{}", reverse(pair));
}