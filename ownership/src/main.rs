fn main() {
    let mut s = String::from("Hello");
    s.push_str(",world!");
    println!("{}",s);

  //  let s1 = String::from("hello");
   // let s2 = s1;
   let s = String::from("Hello world");
   let s1 = first_word(&s);
   let s2 = second_word(&s);

   println!("{}\n{}", s1,s2);
    
   let s2 = "hello world";
   let word = first_word(&s2[0..6]);
   println!("{}", word);
   let word = first_word(&s2[..]);
   println!("{}", word);
   let word = first_word(s2);
   println!("{}", word);
}

//slices - explaining them and how they work
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes(); /*
                                    convert string to bytes to see 
                                    where are white spaces
                                    */

    for(i, &item) in bytes.iter().enumerate(){ //iterating over each byte to see if there is white space
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &str) -> &str{
    let bytes = s.as_bytes();
    
    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[(i+1)..s.len()];
        }
    }
    &s[..]
}