
fn find_largest<T:PartialOrd + Copy>(list: &[T]) -> T {
  let mut largest = list[0];
  for &i in list.iter() {
    if i > largest {
      largest = i;
    }
  }
  largest
}

pub trait Summary {
  fn summarize(&self) -> String;
}

pub struct NewsArticle {
  author: String,
  title: String,
  content: String,
}

impl Summary for NewsArticle{
  fn summarize(&self) -> String {
    format!("{}, by {} ({})", self.title, self.author, self.content)
  }
}

#[derive(Debug)]
struct Point<T> {
  x: T,
  y: T,
}

impl<T> Point<T> {
  fn new( x:T, y:T) -> Self {
    Self {
      x: x,
      y: y,
    }
  }
}
fn main() {
  println!("Hello world!!");

  let v = vec![15, 15, 12323, 23, 21];
  let largest = find_largest(&v);
  println!("Largest value was {}", largest);  

  let p = Point { 
    x:3,
    y:5
  };

  let p2 = Point::new(5,5);

  println!("File {:?}", p);
  println!("File {:?}", p2);

  let art = NewsArticle {
    author: String::from("Joxx0r"),
    title: String::from("test_title"),
    content: String::from("test_content"),
  };

  println!("here is the text {} ", art.summarize());
}

