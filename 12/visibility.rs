mod console {
  pub mod newline {
    fn private() {
    }

    pub fn log(s: &str) {
      private();
      println!("{}", s);
    }
  }

  pub mod sameline {
    pub fn log(s: &str) {
      print!("{}", s);
    }
  }
}

fn main() {
  console::sameline::log("sameline");
  console::newline::log("newline");
} 