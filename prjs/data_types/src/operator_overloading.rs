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

pub fn print_complex()
{
    let a = Complex::new(2, 3);
    let b = Complex::new(3, 6);
    println!("({}) + ({}) = ", a.format(), b.format());
    println!("\t{:?}", a + b);
    
    let mut c = Complex::new(2, 3);
    let d = Complex::new(3, 6);
    println!("({}) += ({}) = ", c.format(), d.format());
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