extern crate sound;
extern crate portaudio;

fn main() {
    match run() {
        Ok(_) => {},
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), portaudio::Error> {
    let portaudio = try!(portaudio::PortAudio::new());
    Err(portaudio::Error::NoError)
}


#[cfg(test)]
mod tests {
    use sound::foo::add;

    #[test]
    fn it_allows_us_to_import_a_function() {
        assert_eq!(add(2, 2), 4);
    }
}

