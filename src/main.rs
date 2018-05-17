extern crate sound;

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use sound::foo::add;

    #[test]
    fn it_allows_us_to_import_a_function() {
        assert_eq!(add(2, 2), 4);
    }
}

