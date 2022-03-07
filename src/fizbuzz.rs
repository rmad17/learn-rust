fn main() {
    for num in 1..101 { // Range notation!
        match (num%3, num%5) { // Pattern Matching FTW!
            (0, 0) => println!("fizzbuzz"),
            (0, _) => println!("fizz"),
            (_, 0) => println!("buzz"),
                 _ => println!("{}", num) 
        }
    }
}
