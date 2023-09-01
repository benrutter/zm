use std::fs;

fn main() {
    if let Some(matched_path) = match_current("target") {
        println!("{}", matched_path);
    } else {
        println!("No matching file found.");
    }
}

fn match_current(to_match: &str) -> Option<String> {

    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        let path_str = path.unwrap().file_name().to_string_lossy().to_string();
        if path_str.ends_with(to_match) {
            return Some(path_str);
        }
    }
    return None
}


// logic is: check current directory
//           otherwise do a breadth first search of everything from home