use std::rc::Rc;

#[derive(Debug)]
struct Person
{
    name: Rc<String>
}

impl Person
{
    fn new(name: Rc<String>) -> Person
    {
        Person {name: name}
    }
}

pub fn print_rc_strong_count()
{
    let jeff = Rc::new("Jeff".to_string());
    println!("{} has {} strong pointer(s) reference(s)", jeff, Rc::strong_count(&jeff));
    {
        let other_jeff = Person::new(jeff.clone());
        println!("{} has {} strong pointer(s) reference(s)", jeff, Rc::strong_count(&jeff));
    }
    println!("{} has {} strong pointer(s) reference(s)", jeff, Rc::strong_count(&jeff));
}