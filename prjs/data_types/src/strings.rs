pub fn print_strings()
{
    // UTF-8
    let mut s1: &str = "Jefferson M Fujioka";
    // s = "new value"; // it doesn't work
    // let ch = s[0]; // it also doesn't work
    for c in s1.chars() {
        print!("{}", c);
    }
    println!("\nReverse...");
    for c in s1.chars().rev() {
        print!("{}", c);
    }
    println!("\n...");

    let idx = 1;
    if let Some(c) = s1.chars().nth(idx)
    {
        println!("Char at {} is {}", idx, c);
    }
    
    let mut string_s1 = String::from("string_s1");
    println!("string_s1: {}", string_s1);

    string_s1 = "string_s1".to_string();   // ditto above
    println!("string_s1: {}", string_s1);

    let mut alphabet_ch = 'a' as u8;
    let mut alphabet = String::new();

    while alphabet_ch < 'z' as u8
    {
        alphabet.push(alphabet_ch as char);
        alphabet.push(',');
        alphabet_ch += 1;
    }
    alphabet.pop();
    println!("The alphabet is: {}", alphabet);
    
    // String to &str
    s1 = &alphabet;
    println!("The alphabet (as &str) is: {}", s1);
    
    // concatenation
    let new_alpha = alphabet + "->ABC";
    println!("The new alphabet is: {}", new_alpha);
    
    let restore_alpha = new_alpha.replace("->ABC", "");
    println!("Restore alphabet: {}", restore_alpha);

}