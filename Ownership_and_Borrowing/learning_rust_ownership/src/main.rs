#![allow(unused_mut, unused_variables)]
use learning_rust_ownership::inspect;
use learning_rust_ownership::change;
use learning_rust_ownership::eat;
use learning_rust_ownership::bedazzle;
fn main() {
    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });
    let result:bool = inspect(&arg);
    change(result, &mut arg);
    println!("I have many {}", arg);
    //
    if eat(arg) {
       println!("Might be bananas");
    } else {
        println!("Not bananas");
    }
    let mut material = "mud".to_string();
    println!("This material is just `{}`.", material);
    bedazzle(&mut material);
    println!("Wow! Now the material is `{}`!", material);
}
