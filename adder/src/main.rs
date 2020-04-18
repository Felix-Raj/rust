use add_one;
fn main() {
    let num =10;
    println!("num {} result {}", num, add_one::add_one(num));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(3, add_one::add_one(2));
    }
}