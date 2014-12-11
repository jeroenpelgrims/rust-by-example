type NanoSecond = u64;
type Inch = u64;
#[allow(non_camel_case_types)]
type uint64_t = u64;

fn main() {
  let nanoseconds: NanoSecond = 5 as uint64_t;
  let inches: Inch = 6;

  println!("{}", nanoseconds * inches);
}