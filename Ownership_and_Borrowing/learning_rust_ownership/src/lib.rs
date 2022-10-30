pub fn inspect(s: &String) -> bool{
    if s.ends_with("s") {
        println!("It's a plural word");
        return true;
    }else{
        println!("The word is on singular!");
        return false;
    }
}


pub fn change(resp:bool, s: &mut String){
    if !resp{
        s.push_str("s");
    }else{
        println!("No, just not...");
    }
}

pub fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    }else{
        return false;
    }
}

pub fn bedazzle(s:&mut String){
    *s = String::from("sparkly");
}