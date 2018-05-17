extern crate sound;
extern crate portaudio;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError<'a> {
    error: &'a str,
}

impl<'a> fmt::Display for MyError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MyError Here")
    }
}

impl<'a> Error for MyError<'a> {
    fn description(&self) -> &str {"MyError Here"}
}

fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), Box<Error>> {
    let portaudio = portaudio::PortAudio::new()?;
    Err(Box::new(MyError{ error: "error" }))
}


#[cfg(test)]
mod tests {
    use sound::foo::add;

    #[test]
    fn it_allows_us_to_import_a_function() {
        assert_eq!(add(2, 2), 4);
    }
}

