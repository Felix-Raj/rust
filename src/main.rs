// https://doc.rust-lang.org/book/ch09-00-error-handling.html
use std::fs::File;

fn main() {
    /*
    recoverable and unrecoverable errors
    use Result<T, E> for recoverable and panic! for unrecoverable
    unwinding - rust walks back up and cleans the stack from each and
        every functions it encounters, but this is slow. Can opt for 
        immediate abort adding
        panic='abort'
        in appropriate [profile] section in Cargo.toml
        eg:-
        [profile.release]
        panic = 'abort'
    If not unwinding, cleaning for memory location is handled by OS
    */
    panic!("Crash!!!");
    let f = File::open("Hello.txt");
    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result
}