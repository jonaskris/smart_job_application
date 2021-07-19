use regex::Regex;
use std::io::{stdin};

/*
    Processes a string.
    Replaces tokens following the patterns:
        #token_name#
        #token_name[replacement_string]

    Processes the string in 2 main steps:
        First it iterates over all tokens based on regex pattern.
            For each token, it asks the user if it should replace the token with the replacement_string or simply remove the token.

            If no replacement_string is given in the token, it asks the user to write the replacement_string for the token.

            Each of these changes to the original string are stored in a vec of processing_changes, which contains the range of characters (index_from, index_to)
            to replace with the replacement string.

        Secondly it iterates over the processing_changes in reverse, as to keep the ranges for subsequent changes.

*/
pub fn process(original_string: &String) -> String {
    let mut processed_string = original_string.clone();

    let mut processing_changes = Vec::new(); // Changes to the original contents described as a tuple of a range and what string to replace the range with

    let token_pattern = Regex::new(r"#([a-zA-ZæøåÆØÅ_]*)(?:\[([\w\sæøåÆØÅ\-.,;:]*)\])?#").unwrap(); // Pattern used to search for tokens
    for regex_match in token_pattern.captures_iter(&processed_string) {
        let token = regex_match.get(0);
        let token_name = regex_match.get(1);
        let replacement_string = regex_match.get(2);

        match replacement_string {
            Some(replacement_string) => {
                println!("Should this token's: \n\t{}\n default replacement text: \n\t\"{}\"\n\tbe inserted?", token_name.unwrap().as_str(), replacement_string.as_str());
                let replace = get_yes_no_input();

                if replace {
                    processing_changes.push((token.unwrap().start(), token.unwrap().end(), String::from(replacement_string.as_str())));
                } else {
                    processing_changes.push((token.unwrap().start(), token.unwrap().end(), String::from("")));
                }
            },
            None => {
                println!("What should token {} be replaced with?", token_name.unwrap().as_str());
                let replacement_string = get_string_input();
                processing_changes.push((token.unwrap().start(), token.unwrap().end(), replacement_string));
            }
        }
    }

    // Apply processing changes from back to front so ranges remain valid
    for (start, end, replacement_string) in processing_changes.into_iter().rev() {
        processed_string.replace_range(start..end, replacement_string.as_str());
    }

    return processed_string;
}

fn get_yes_no_input() -> bool {
    println!("Y/N?");

    loop {
        let mut answer = String::new();
        let res = stdin().read_line(&mut answer);

        match res {
            Ok(_) => {
                if answer.trim() == "Y" || answer.trim() == "y" {
                    return true;
                } else if answer.trim() == "N" || answer.trim() == "n" {
                    return false;
                } else {
                    println!("Invalid input, try again!\nY/N?");
                }
            },
            Err(err) => {
                println!("Invalid input, error: {}", err);
            }
        };
    }
}

fn get_string_input() -> String {
    let mut string = String::new();
    loop {
        let result = stdin().read_line(&mut string);

        match result {
            Ok(_) => {
                string = String::from(string.trim());
                return string;
            },
            Err(err) => {
                println!("Not valid input! Err: {}\nTry again!", err);
            }
        }
    }
}