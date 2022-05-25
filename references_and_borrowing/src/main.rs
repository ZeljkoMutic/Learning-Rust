fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}", s1, len);


    let mut s2 = String::from("hello");
    change(&mut s2);

    let mut s3 = String::from("hello");

    let r1 = &s3;
    println!("{} ",r1);
    let r2 = &mut s3;
    
    println!("{} ", r2);

    let _reference_to_nothing = dangle();
}

fn dangle() -> String{
    let s = String::from("hello");
    s
}

fn change(some_string: &mut String){
    some_string.push_str(", world");
}

fn calculate_length(s: &String) -> usize{
    s.len()
}
