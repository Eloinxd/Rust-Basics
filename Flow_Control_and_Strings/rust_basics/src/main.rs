// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_mut, unused_variables)]

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for arg in args {
        if arg == "sum" {
            sum();
        }else if arg == "double" {
            double();
        }else{
            count(arg);
        }
    }
}

fn sum() {
    let mut sum = 0;
    for num in 7..=23 {
        sum = sum + num;
    }
 
    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;
    while x <= 500 {
        count = count + 1;
        x  = x * 2;
    }

    println!("You can double x {} times until x is larger than 500", count);
}

fn count(arg: String) {
    let mut count = 0;
    loop {
        if count < 8 {
            println!("{}", arg); 
        }else{
            break;
        }
        count = count + 1;
    }

}