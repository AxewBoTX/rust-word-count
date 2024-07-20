#[cfg(test)]
mod tests;

use std::{env, fs, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub struct ArgumentList {
    pub options: Vec<String>,
    pub paths: Vec<String>,
}
impl ArgumentList {
    // filter and sort arguments into `ArgumentList` object
    pub fn new(args: Vec<String>) -> ArgumentList {
        let mut options: Vec<String> = vec![];
        let mut paths = vec![];

        let default_options = vec!["--bytes", "--words", "--lines"];

        for arg in &args[1..] {
            match (
                arg.starts_with("--"),
                default_options.contains(&arg.as_str()),
            ) {
                (true, false) => {
                    // invalid option argument
                    println!("Invalid option argument: {:#?}", arg)
                }
                (true, true) => {
                    // proper option argument
                    options.push(arg.to_string());
                }
                (false, false) => match get_file_path(&arg.to_string()) {
                    // maybe a file
                    Ok(_) => paths.push(arg.to_string()),
                    Err(e) => {
                        eprintln!("{}", e.to_string());
                    }
                },
                (_, _) => {}
            }
        }

        return ArgumentList { options, paths };
    }
    // check provided argument list options return boolean tuple
    pub fn check_options(&self) -> (bool, bool, bool) {
        return (
            self.options.contains(&String::from("--bytes")),
            self.options.contains(&String::from("--words")),
            self.options.contains(&String::from("--lines")),
        );
    }
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let arg_list = ArgumentList::new(args);

    let options = arg_list.check_options();
    for file_path in &arg_list.paths {
        match get_file_info(file_path.to_string(), options) {
            Ok(contents) => {
                println!("");
                print!("{:#?}: ", file_path.to_string());
                if options.0 == true {
                    print!("bytes-{} ", contents.0);
                }
                if options.1 == true {
                    print!("words-{} ", contents.1)
                }
                if options.2 == true {
                    print!("lines-{} ", contents.2)
                }
                if options == (false, false, false) {
                    print!(
                        "bytes-{} words-{} lines-{}",
                        contents.0, contents.1, contents.2
                    );
                }
                println!("");
            }
            Err(e) => {
                eprintln!("{}", e.to_string());
            }
        }
    }
}

// get `PathBuf` of provided filename or return an error
pub fn get_file_path(filename: &String) -> Result<PathBuf, String> {
    match fs::canonicalize(&PathBuf::from(&filename)) {
        Ok(filepath) => return Ok(filepath),
        Err(e) => {
            return Err(
                format!("Filename: {:#?} Error: {:#?}", filename, e.to_string()).to_string(),
            );
        }
    };
}

pub fn get_file_info(
    filename: String,
    options: (bool, bool, bool),
) -> Result<(usize, usize, usize), String> {
    let filepath = match get_file_path(&filename) {
        Ok(filepath) => filepath,
        Err(e) => {
            return Err(format!("{}", e.to_string()).to_string());
        }
    };
    let contents = match fs::read_to_string(&filepath) {
        Ok(contents) => contents,
        Err(e) => {
            return Err(format!(
                "Failed to parse file contents to a string, Error: {}",
                e.to_string()
            ));
        }
    };
    if options == (false, false, false) {
        return Ok((
            contents.as_bytes().len(),
            contents
                .strip_suffix("\n")
                .unwrap()
                .split("\n")
                .collect::<Vec<_>>()
                .join(" ")
                .split(" ")
                .filter(|word| !word.is_empty())
                .collect::<Vec<_>>()
                .len(),
            contents
                .strip_suffix("\n")
                .unwrap()
                .split("\n")
                .collect::<Vec<_>>()
                .len(),
        ));
    } else {
        return Ok((
            {
                // bytes
                if options.0 == true {
                    contents.as_bytes().len()
                } else {
                    0
                }
            },
            {
                // words
                if options.1 == true {
                    contents.split_whitespace().count()
                } else {
                    0
                }
            },
            {
                // lines
                if options.2 == true {
                    contents
                        .strip_suffix("\n")
                        .unwrap()
                        .split("\n")
                        .collect::<Vec<_>>()
                        .len()
                } else {
                    0
                }
            },
        ));
    }
}
