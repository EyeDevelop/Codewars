mod stop_spinning_my_words;

fn main() -> Result<(), std::io::Error> {
    println!("{}", stop_spinning_my_words::stop_spinning_my_words("Hey fellow warriors"));
    Ok(())
}