use std::fmt::Debug;

pub trait Shape
{
    fn name(&self) -> &'static str;
    fn area(&self) -> f64;
}

#[derive(Debug)]
pub struct Square
{
    pub side: f64
}

#[derive(Debug)]
pub struct Circle
{
    pub radius: f64
}

impl Shape for Square
{
    fn name(&self) -> &'static str
    {
        "Square"
    }
    fn area(&self) -> f64
    {
        self.side * self.side
    }
}

impl Shape for Circle
{
    fn name(&self) -> &'static str 
    {
        "Circle"
    }
    fn area(&self) -> f64 
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}

pub fn print_shape_1(shape1: impl Shape + Debug, 
                     shape2: impl Shape + Debug)
{
    println!("1.1 -> the area of {:?} is {}", shape1, shape1.area());
    println!("1.2 -> the area of {:?} is {}", shape2, shape2.area());
}

pub fn print_shape_2<T1: Shape + Debug, 
                     T2: Shape + Debug>(shape1: T1, shape2: T2)
{
    println!("2.1 -> the area of {:?} is {}", shape1, shape1.area());
    println!("2.2 -> the area of {:?} is {}", shape2, shape2.area());
}

pub fn print_shape_3<T1, T2>(shape1: T1, shape2: T2)
    where T1: Shape + Debug,
          T2: Shape + Debug
{
    println!("3.1 -> the area of {:?} is {}", shape1, shape1.area());
    println!("3.2 -> the area of {:?} is {}", shape2, shape2.area());
}
