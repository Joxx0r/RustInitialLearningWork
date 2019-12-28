
use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
fn here_is_a_test(value:i32 )
{
    println!("Value: {}", value);
}

fn here_is_a_test2(value: &mut String ) 
{
    println!("here is a value {}", value);
    value.push_str("_other_test_");
}
*/
fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 100);
   
    println!("Secret number {}", secret_number);
/*

    let mut secretName = String::from("here is a test");
    let secretNameTwo = secretName.clone();
    println!("#1 {} #2 {}", secretName, secretNameTwo);
    let number = 32;    
    here_is_a_test(number);
    here_is_a_test(number);
    here_is_a_test(number);
    here_is_a_test(number);

    here_is_a_test2(&mut secretName);
    here_is_a_test2(&mut secretName);
    
    here_is_a_test2(&mut secretName);
    here_is_a_test2(&mut secretName);
    here_is_a_test2(&mut secretName);
*/
    
   /* loop
    {
        println!("Please input your guess.!");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed parsing number");
        println!("Decided number {}",guess);
    
        let guess : u32 = match guess.trim().parse()
        {
            Ok(num) => num,
            Err(_) =>
            {
                println!("Non valid number. Try again");
                continue;
            }
        };

        let x  = 5;
        let y  = 5.0;

        println!("Here are x: {}, y: {}", x, y); 
    
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Number is greater"),
            Ordering::Less  => println!("Number is smaller"),
            Ordering::Equal => 
            {
                println!("Number is equal");
                break;
            }
        }

        
    }*/
}