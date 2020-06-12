pub trait Summable<T>
{
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut res:i32 = 0;
        for x in self { res += *x; }
        return res;
    }
}