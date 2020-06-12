#[derive(Debug)]
struct Person
{
    name: String
}

#[derive(Debug)]
struct Company<'a>
{
    name: String,
    ceo: &'a Person
    // ceo: &Person -> error[E0106]: missing lifetime specifier
}

impl Person
{
    // lifetime ellision -> compiler expands to:
    // fn get_ref_name<'a>(&'a self) -> &'a String
    fn get_ref_name(&self) -> &String
    {
        &self.name
    }
}

// impl Company -> error[E0726]: implicit elided lifetime not allowed here
impl<'a> Company<'a>
{
    fn print(&self)
    {
        println!("CEO of {} is {}", self.name, self.ceo.name);
    }
}

pub fn print_company_person()
{
    let boss = Person { name: String::from("Elon Musk") };
    let tesla = Company { name: String::from("Tesla"), ceo: &boss };

    println!("{:?}", tesla);
    tesla.print();

    let z: &String;
    {
        let john = Person { name: String::from("John") };
        z = john.get_ref_name();
        println!("{}", z);
    }
    // println!("{}", z); -> error[E0597]: `john` does not live long enough
}