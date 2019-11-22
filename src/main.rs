#[allow(unused_variables)]
// https://doc.rust-lang.org/book/ch09-00-error-handling.html
use std::fs::File;
use std::io::{self, ErrorKind, Read};

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
    
    let f = File::open("Hello.txt");
    // return type of File::open is a Result<T,E> on sucess it is Ok
    // and on error it is Err
    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("Hello.txt") {
                Ok(result) => result,
                Err(err) => panic!("Problem creating file {:?}", err),
            },
            other_error => panic!("Problem opening file {:?}", other_error),
        },
    };

    // unwrap
    // unwrap return the inner value is Result is Ok, else calls panic!
    let f = File::open("Hello.txt").unwrap();  // change file name and run
    // expect
    // expect does the same, but allows to specify the error message
    let f = File::open("Hello.txt").expect("File 404!");  // the error message
    // will start with the message tha we specified

    let uname = read_from_file().unwrap();
    println!("uname {}", uname);

    println!("uname {}",read_from_file_2().unwrap() );
}

fn read_from_file() -> Result<String, io::Error> {
    // propogating error
    let mut f = File::open("Hello.txt").unwrap();
    let mut s = String::new();
    // code that call this will get a Ok is success, else Err - as this are Result
    // the signature is fine
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(r) => Err(r),
    }
}

fn read_from_file_2() -> Result<String, io::Error> {
    // See the difference b/w using ? and match in the doc ( the same page )
    let mut f = File::open("Hello.txt").unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
    // to use `?`, the function that contains it usage ( here read_from_file_2 ) should
    // return Result<T, E>
}