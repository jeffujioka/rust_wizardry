pub trait Animal
{
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn speak(&self)
    {
        println!("{} cannot speak.", self.name())
    }
}

pub struct Human
{
    pub name: &'static str
}

pub struct Cat
{
    pub name: &'static str
}

impl Animal for Human
{
    fn create(name: &'static str) -> Human
    {
        return Human{name: name};
    }

    fn name(&self) -> &'static str
    {
        self.name
    }

    fn speak(&self)
    {
        println!("{} says hello!", self.name)
    }
}

impl Animal for Cat
{
    fn create(name: &'static str) -> Cat
    {
        Cat{name: name}
    }
    fn name(&self) -> &'static str
    {
        self.name
    }
}