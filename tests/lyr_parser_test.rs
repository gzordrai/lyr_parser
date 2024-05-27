#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use lyr_parser::parse_lyr_file;
    use std::io::ErrorKind;

    #[test]
    fn test_non_existent_file() {
        let result = parse_lyr_file("non_existent_file.lyr");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::NotFound);
    }

    #[test]
    fn test_wrong_extension() {
        let path: &str = "test.txt";
        let mut file: File = File::create(path).unwrap();
        writeln!(file, "Test").unwrap();

        let result = parse_lyr_file(path);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().kind(), ErrorKind::InvalidInput);

        fs::remove_file(path).unwrap();
    }

    #[test]
    fn test_correct_extension() {
        let path: &str = "test.lyr";
        let mut file: File = File::create(path).unwrap();
        writeln!(file, "<Object Id=\"123\"/>").unwrap();

        let result = parse_lyr_file(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec!["123"]);

        fs::remove_file(path).unwrap();
    }
}