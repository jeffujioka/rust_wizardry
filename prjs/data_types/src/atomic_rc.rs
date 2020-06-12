use std::sync::{Arc, Mutex};

#[derive(Debug)]
struct Person
{
    name: Arc<String>,
    state: Arc<Mutex<String>>
}

impl Person
{
    fn new(name: Arc<String>, state: Arc<Mutex<String>>) -> Person
    {
        Person {name: name, state: state}
    }

    fn greet(&self)
    {
        let mut l_st = self.state.lock().unwrap();
        l_st.clear();
        l_st.push_str("EXCITED!");
        println!("Hi, my name is {} and I'm {}", self.name, l_st.as_str());
    }
}

pub fn print_arc_strong_count()
{
    let jeff = Arc::new("Jeff".to_string());
    let state = Arc::new(Mutex::new("bored".to_string()));
    let person = Person::new(jeff.clone(), state.clone());

    let t = std::thread::spawn(move || {
        person.greet();
    });

    t.join().unwrap();
    println!("Name = {}, state = {}", jeff, state.lock().unwrap());
}