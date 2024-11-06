#![no_main]

#[no_mangle]

// searches a string for a substring and returns the match
pub fn main() {
    valida_rs::io::println("Please enter the substring to search for:");
    let query = valida_rs::io::read_line::<String>().unwrap();
    valida_rs::io::println("Please enter the contents to search in:");
    let contents = valida_rs::io::read_line::<String>().unwrap();
    
    valida_rs::io::println(
        &format!("Searching for '{}' in '{}'.", query, contents)
    );
    
    let results = search_case_insensitive(&query, contents.as_str());
    valida_rs::io::println(
        &format!("Words that contain '{}': {:?}", query, results)
    );
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        for word in line.split_whitespace() {
            if word.to_lowercase().contains(&query) {
                results.push(word);
            }
        }
    }

    results
}