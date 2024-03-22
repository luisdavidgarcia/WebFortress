fn main() {
    println!("Hello, world!");
    let num: u8 = 5;
    println!("The number is: {}", num);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
