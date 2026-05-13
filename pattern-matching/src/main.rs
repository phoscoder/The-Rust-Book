fn main() {
    
    #[derive(Debug)]
    enum Language {
        English,
        Spanish,
        Russian,
        Japanese
    }

    let language = Language::English;

    match language {
        Language::English => println!("Hello, world!"),
        Language::Spanish => println!("¡Hola, mundo!"),
        Language::Russian => println!("Привет, мир!"),
        Language::Japanese => println!("こんにちは、世界！"),
        lang => println!("Unknown language! {:?}", lang),
    }  
}
