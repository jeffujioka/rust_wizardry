use std::ops::{Add, AddAssign, Neg};

#[derive(Debug)]
pub struct Complex<T>
{
    re: T,
    im: T
}

impl<T> Complex<T>
{
    fn new(re: T, im: T) -> Complex<T>
    {
        Complex::<T>{re: re, im: im}
    }
}

impl<T> Add for Complex<T>
    where T: Add<Output = T>
{
    type Output = Complex<T>;

    fn add(self, rhs: Self) -> Self::Output
    {
        Complex {
            re: self.re + rhs.re,
            im: self.im + rhs.im
        }
    }
}

impl<T> AddAssign for Complex<T>
where T: AddAssign<T>
{
    fn add_assign(&mut self, rhs: Self)
    {
        self.re += rhs.re;
        self.im += rhs.im;
    }
}

impl<T> Neg for Complex<T>
where T: Neg<Output = T>
{
    type Output = Complex<T>;

    fn neg(self) -> Self::Output
    {
        Complex {
            re: -self.re,
            im: -self.im
        }
    }
}

// this is another way.. without using --> where T: PartialEq
// or struct Complex<T> could simply inherite from PartialEq
// #[derive(Debug, PartialEq, Eq)]
// struct Complex<T>
impl<T: PartialEq> PartialEq for Complex<T>
{
    fn eq(&self, rhs: &Self) -> bool
    {
        self.re == rhs.re && self.im == rhs.im
    }
}

trait Printable
{
    fn format(&self) -> String;
}

impl<T: std::fmt::Display> Printable for Complex<T>
{
    fn format(&self) -> String
    {
        format!("{} + {}j", self.re, self.im)
    }
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

fn print_printable_staticaly<T: Printable>(val: &T)
{
    print!("({})", val.format());
}

fn print_printable_dinamically(val: &dyn Printable)
{
    print!("({})", val.format());
}

pub fn print_complex()
{
    let i = 100;
    println!("my i: {}", i.format());
    print_printable_staticaly(&i);
    println!("");
    print_printable_dinamically(&i);
    println!("");

    let a = Complex::new(2, 3);
    let b = Complex::new(3, 6);
    print_printable_staticaly(&a);
    print!(" + ");
    print_printable_dinamically(&a);
    print!(" = {:?}\n", a + b);
    
    let mut c = Complex::new(2, 3);
    let d = Complex::new(3, 6);
    print!("({}) += ({}) = ", c.format(), d.format());
    c += d;
    println!("\t{}", c.format());
    println!("\t{}", (-c).format());
    
    let e = Complex::new(2, 3);
    let f = Complex::new(3, 6);
    println!("({}) + ({}) = ", e.format(), f.format());
    println!("\t{}", e == e);
    println!("\t{}", e == f);
    println!("\t{}", f == e);
}