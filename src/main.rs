/*
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len()
  {
    x
  }
  else
  {
    y
  }
}
*/

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  if x.len() > y.len()
  {
    x
  }
  else
  {
    y
  }
}
fn main() {
  println!("hello world");
  let a = "test232";
  let b = "test_1";
  
  let f = longest(a, b);
  println!("Longest string was {}", f);
}

