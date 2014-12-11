use console::newline::log as log;

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
  log("newline");

  {
    use console::sameline::log as log;

    log("sameline");
  }
} 