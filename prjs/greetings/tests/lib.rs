#[cfg(test)]

extern crate phrases;

mod tests
{
    use phrases::greetings::english;

    #[test]
    fn english_greeting_hello_correct()
    {
        assert_eq!("Hello!", english::hello());
    }

    #[test]
    #[should_panic]
    fn english_greeting_hello_incorrect()
    {
        assert_eq!("Hi!", english::hello());
    }

    #[test]
    #[ignore]
    fn english_greeting_hello_whatever()
    {
        assert_eq!("F#*!#", english::hello());
    }
}