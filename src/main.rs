

use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
  //base test
  //panic!("test");

  //out of bounds crash
  //let v = vec![1,2,3];
 // v[99];


 //file reading example
 /*let path = "bin/hello.txt";
  let f = File::open(path);
  let f = match f {
    Ok(file) => 
    {
      println!("Found file {}", path);
      file
    }
    Err(file_open_error) => match file_open_error.kind() {
      ErrorKind::NotFound => match File::create(path)
      {
        Ok(fc) => fc,
        Err(file_creation_error) => panic!("failed creating file {:?}", file_creation_error),
      }
      other_error => panic!("Problem opening file {:?}", file_open_error),
    }
  };*/

 /* let filePath = "bin/hello.txt";
  let f = match File::open(filePath) 
  {
    Ok(file) => {
      println!("Successfully open filed {}", filePath);
      file
    }
    Err(file_opening_error) => match file_opening_error.kind() {
      ErrorKind::NotFound => match File::create(filePath) {
          Ok(file) => file,
          Err(error) => panic!("Failed creating file {:?}", error),
      }
      other_error => panic!("problem opening file {:?}", file_opening_error),
    }
  };*/

 /* let filePath = "bin/hello_2.txt";
  let f = match File::open(filePath)
  {
    Ok(file) => file,
    Err(error) => match error.kind() {
      ErrorKind::NotFound => match File::create(filePath) {
        Ok(file) => file,
        Err(file_creating_error) => panic!("failed creating file {:?}", file_creating_error),
      }
      other_error => panic!("failed opening filepath {}", filePath),
    }
  };
  */
/*
  let filePath = "bin/hello_3.txt";
  let f = File::open(filePath).unwrap_or_else(|error|
  {
    if error.kind() == ErrorKind::NotFound {
      File::create(filePath).unwrap_or_else(|error|{
        panic!("Failed creating file {:?}", error);
      })
    }
    else
    {
      panic!("other error {:?}", error.kind());
    }
  });*/

  /*let filepath = "bin/hello_4.txt";
  let f = File::open(filepath).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
      File::create(filepath).unwrap_or_else(|error|{
        panic!("failed creating file {:?}", error.kind());
      })
    }
    else
    {
      panic!("Failed creating file{}", filepath);
    }
  });*/

  /*let filepath = "bin/hello_5.txt";
  let f = File::open(filepath).unwrap_or_else(|error|{
    if error.kind() == ErrorKind::NotFound {
      File::create(filepath).unwrap_or_else(|error|{
        panic!("Failed creating file {:?}", error.kind());
      })
    }
    else
    {
      panic!("Failed creating file {:?}", error.kind());
    }
  });*/
/*
  let filePath = "hello_expect.txt";
  let f = File::open(filePath);
  let mut f = match f {
    Ok(f) => f,
    Err(e) => println()
  };*/

  let f= match read_username_from_file("bin/username.txt") {
    Ok(f) => f,
    Err(_) => panic!("failed opening file!!"), 
  };

  println!("username: {}", f);
  
}

/*
fn read_username_from_file(filePath:&str) -> Result<String, io::Error> {
  let f = File::open(filePath);
  let mut f = match f {
      Ok(file) => file,
      Err(e) => return Err(e),
  };

  let mut s = String::new();

  match f.read_to_string(&mut s) {
      Ok(_) => Ok(s),
      Err(e) => Err(e),
  }
}*/

/*
fn read_username_from_file(filePath:&str) -> Result<String, io::Error> {
  let f = File::open(filePath);
  let mut f = match f {
    Ok(f) => f,
    Err(e) => return Err(e),
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  }
}*/
/*
fn read_username_from_file(filePath:&str) -> Result<String, io::Error> {
  let f = File::open(filePath);
  let mut f = match f {
    Ok(f) => f,
    Err(e) => return Err(e),
  };

  let mut s = String::new();
  match f.read_to_string(&mut s) {
    Ok(_) => Ok(s),
    Err(e) => Err(e),
  };
}*//*

fn read_username_from_file(filePath:&str) -> Result<String, io::Error> {
  let mut f = File::open(filePath)?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}*/

fn read_username_from_file(filePath:&str) -> Result<String, io::Error> {
  let mut f = File::open(filePath)?;
  let mut s = String::new();
  f.read_to_string(&mut s)?;
  Ok(s)
}