use super::*;
use std::env;

#[test]
fn read_poem() {
    let config = Config{
        query: String::from(""),
        filename: String::from("poem.txt"),
        case_sensitive: true,
    };
    match run(config) {
        Ok(()) => {},
        Err(e) => panic!("Failed reading file"),
    };
}

#[test]
fn no_such_file() {
    let config = Config{
        query: String::from(""),
        filename: String::from("poem.txt?"),
        case_sensitive: true,
    };
    match run(config) {
        Ok(()) => panic!("Should fail in reading file"),
        Err(e) => {},
    };
}

#[test]
#[should_panic]
fn zero_arguments_fails() {
    let args: Vec<String> = vec![String::from("program")];
    Config::new(&args).unwrap();
}

macro_rules! vec_of_strings {
    ($($x:expr),*) => (vec![$($x.to_string()),*]);
}

#[test]
#[should_panic]
fn one_arguments_fails() {
    let args = vec_of_strings!["program", "arg1"];
    Config::new(&args).unwrap();
}

#[test]
#[should_panic]
fn four_arguments_no_ci_fails() {
    let args = vec_of_strings!["program", "-x", "arg2", "arg3"];
    Config::new(&args).unwrap();
}

#[test]
fn four_arguments_ci() {
    let args = vec_of_strings!["program", "-i", "arg2", "arg3"];
    Config::new(&args).unwrap();
}

#[test]
#[should_panic]
fn five_arguments_fails() {
    let args = vec_of_strings!["program", "arg1", "-i", "arg2", "arg3", "arg4"];
    Config::new(&args).unwrap();
}

#[test]
fn read_three_args_env_off() {
    let args = vec_of_strings!["program", "arg1", "arg2"];
    env::remove_var("CASE_INSENSITIVE");
    let config = Config::new(&args).unwrap();
    assert_eq!("arg1", config.query);
    assert_eq!("arg2", config.filename);
    assert_eq!(true, config.case_sensitive);
}

#[test]
fn read_three_args_env_on() {
    let args = vec_of_strings!["program", "arg1", "arg2"];
    env::set_var("CASE_INSENSITIVE", "");
    let config = Config::new(&args).unwrap();
    assert_eq!("arg1", config.query);
    assert_eq!("arg2", config.filename);
    assert_eq!(false, config.case_sensitive);
}

#[test]
fn read_four_args() {
    let args = vec_of_strings!["program", "-i", "arg1", "arg2"];
    let config = Config::new(&args).unwrap();
    assert_eq!("arg1", config.query);
    assert_eq!("arg2", config.filename);
    assert_eq!(false, config.case_sensitive);
}

#[test]
fn case_sensitive() {
    let query = "duct";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

    assert_eq!(
        vec!["safe, fast, productive."],
        search(query, contents)
    );
}

#[test]
fn case_insensitive() {
    let query = "rUsT";
    let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

    assert_eq!(
        vec!["Rust:", "Trust me."],
        search_case_insensitive(query, contents)
    );
}
