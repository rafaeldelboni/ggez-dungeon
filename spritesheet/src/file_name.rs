use regex::{Regex};

pub struct FileName {
    pub name: String,
    pub ext: String,
}

pub fn extract(file_name: &str) -> FileName {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?P<name>[^:\\/]*?)(?:\.(?P<ext>[^ :\\/.]*))?$"
        ).unwrap();
    }
    let file = RE.captures(&file_name).unwrap();

    FileName {
        name: String::from(&file["name"]),
        ext: String::from(&file["ext"]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_extract_file_name() {
        let file = "filename.ext";
        assert_eq!(extract(&file).name, "filename");
    }

    #[test]
    fn should_extract_file_extension() {
        let file = "something.ext";
        assert_eq!(extract(&file).ext, "ext");
    }
}
