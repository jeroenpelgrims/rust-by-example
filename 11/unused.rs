fn used_fn() {
}

fn unused_fn() {
}

#[allow(dead_code)]
fn ignored_unused_fn() {
}

fn main() {
  used_fn();
}