use std::io;
use headless_chrome::Browser;
use std::error::Error;

const URL_LENGTH: u16 = 128;
//take url as an input from the user
//in general web browsers limit the maximum size of urls upto 2048 characters

fn main() {
    //print instructions
    print_instructions();

    let mut input_url = String::with_capacity(128);
    println!("");
    println!("Enter the url of the web page to surf !");
    io::stdin()
        .read_line(&mut input_url)
        .expect("Failed to read line");
    println!("");
    println!("You entered {} ", input_url);

    println!("******************************************************");
    println!("");
    open_browser(input_url);
    
    
}

fn open_browser(input_url: String) -> Result<(), Box<dyn Error>> {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;
    tab.navigate_to(&input_url)?;
    Ok(())
}


fn print_instructions() {
    println!("******************************************************");
    println!("");

    println!("Hello, welcome to rusty surfer !");
    println!("");
    println!("");

    println!("1. My aim is to present you with you details of all remote addresses that are used to load a webpage !");
    println!("");

    println!("2. To begin you have to enter a URL for me ");
    println!("");

    println!(
        "3. The maximum length of the URL should be {} characters",
        URL_LENGTH
    );
    println!("");

    println!("4. Lets do it !!!");
    println!("");

    println!("");
    println!("******************************************************");
}
