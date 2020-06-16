pub mod greetings
{
    pub mod english
    {
        pub fn hello() -> String { "Hello!".to_string() }
        pub fn goodbye() -> String { "Goodbye".to_string() }
    }
    pub mod portuguese
    {
        pub fn hello() -> String { "Fala meu querido!".to_string() }
        pub fn goodbye() -> String { "Falô".to_string() }
    }
    pub mod german
    {
        pub fn hello() -> String { "Hallo!".to_string() }
        pub fn goodbye() -> String { "Tchuss".to_string() }
    }
}