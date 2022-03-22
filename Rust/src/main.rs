mod highest_lowest;
use highest_lowest::highest_lowest;

fn main() -> Result<(), std::io::Error> {
    println!("{}", highest_lowest("1 2 3 4 5"));
    Ok(())
}