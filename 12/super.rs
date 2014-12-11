fn function() {
  println!("function()");
}

mod my {
  fn function() {
    println!("my::function()");
  }

  mod cool {
    pub fn function() {
      println!("my::cool::function()");
    }
  }

  pub fn do_stuff() {
    function();
    super::function();
    cool::function();
    self::cool::function();
    super::cool::function();

    {
      use super::cool::function as root_cool_function;
      root_cool_function();
    }
  }
}

mod cool {
  pub fn function() {
    println!("cool::function();");
  }
}

fn main() {
  my::do_stuff();
}