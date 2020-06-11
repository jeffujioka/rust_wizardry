use std::mem;

const MY_GLOBAL_CONST_VAR:u32 = 0xdeadbeef;
static mut MY_GLOBAL_STATIC_VAR:u32 = MY_GLOBAL_CONST_VAR;

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
}
