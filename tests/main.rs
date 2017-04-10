// Conditionally compile `main` only when the test-suite is *not* being run.
#[cfg(not(test))]
fn main() {
    println!("If you see this, the tests were not compiled nor ran!");
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}