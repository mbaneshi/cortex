pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(file_path)
}
