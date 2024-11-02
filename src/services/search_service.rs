pub fn search_in_file(file_content: &str, query: &str) -> Vec<String> {
    file_content
        .lines()
        .filter(|line| line.contains(query))
        .map(String::from)
        .collect()
}
