mod console {
  mod newline {
    fn log(s: &str) {
      println!("{}", s);
    }
  }

  mod sameline {
    fn log(s: &str) {
      print!("{}", s);
    }
  }
}

fn main() {
  console::newline::log("newline"); //error: fn is private
  console::sameline::log("sameline");  //error: fn is private
}