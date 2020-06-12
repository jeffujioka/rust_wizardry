pub struct Person
{
    pub name: String
}

impl Person
{
    // pub fn new(name: &str) -> Person
    // {
    //     Person{name: name.to_string()}
    // }

    // pub fn new<S: Into<String>>(name: S) -> Person
    pub fn new<S>(name: S) -> Person
        where S: Into<String>
    {
        Person{name: name.into()}
    }
}