mod my;
mod sub;

fn main() {
  my::function();
  sub::my2::function();

  {
    use sub::my2::function as foo;
    foo();
  }
}