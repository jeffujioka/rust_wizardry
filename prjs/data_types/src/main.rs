use std::mem;

const MY_GLOBAL_CONST_VAR:u32 = 0xdeadbeef;
static mut MY_GLOBAL_STATIC_VAR:u32 = MY_GLOBAL_CONST_VAR;

fn ex_loop()
{
    let mut i = 0;
    loop // == while true
    {
        println!("loop -> i = {}", i);
        i += 1;
        if i > 3 { break; }
    }
}

fn ex_or()
{
    for i in 0..3 // 0 -> 2
    {
        println!("for -> i = {}", i);
    }

    for (i, val) in (30..35).enumerate() // 30 -> 34
    {
        println!("for -> i:val = {}:{}", i, val);
    }
}

fn ex_match(country_code:u64)
{
    let country = match country_code {
        44 => "UK",
        49 => "Germany",
        7 => "Russia",
        55 => "Brazil",
        1..=1000 => "Unknown",
        _ => "Invalid"
    };
    println!("Country with code {} is {}", country_code, country);
}

fn main() {
    println!("Hello, data types!");

    let var_u8: u8 = 10;
    println!(
        "let var_u8:u8 = {} has {} bytes",
        var_u8,
        mem::size_of_val(&var_u8)
    );

    let v = 10;
    println!("let v = {} has {} bytes", v, mem::size_of_val(&v));
    let mut mut_v = 10;
    println!(
        "let mut mut_v = {} has {} bytes",
        mut_v,
        mem::size_of_val(&mut_v)
    );
    mut_v = 2552;
    println!(
        "let mut mut_v = {} has {} bytes",
        mut_v,
        mem::size_of_val(&mut_v)
    );

    let d = 10.0;
    println!("let d = {} has {} bytes", d, mem::size_of_val(&d));

    let f:f32 = 10.0;
    println!("let f = {} has {} bytes", f, mem::size_of_val(&f));

    let ptr: isize = 10;
    println!(
        "let ptr = {} has {} bytes, {}-bit OS ",
        ptr,
        mem::size_of_val(&ptr),
        mem::size_of_val(&ptr) * 8
    );

    let c = 'a';
    println!("let c = {} has {} bytes", c, mem::size_of_val(&c));
    
    let b: bool = false;
    println!("let b = {} has {} bytes", b, mem::size_of_val(&b));
    
    let b2 = 4 > 0;
    println!("let b2 = {} has {} bytes", b2, mem::size_of_val(&b2));

    println!("2.0^4 = {}", f64::powi(2.0, 4));
    println!("2.0^PI = {}", f64::powf(2.0, std::f64::consts::PI));

    println!("MY_GLOBAL_CONST_VAR = {}", MY_GLOBAL_CONST_VAR);
    unsafe
    {
        println!("MY_GLOBAL_STATIC_VAR = {}", MY_GLOBAL_STATIC_VAR);
    }

    let temp = 20;
    let day = if  temp > 20 {"sunny"} else {"cloudy"};
    println!("Today is {}", day);

    ex_loop();
    ex_or();
    ex_match(49);
    ex_match(55);
    ex_match(1000);
    ex_match(11000);
}
