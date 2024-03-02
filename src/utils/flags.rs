use core::cell::RefCell;
use regex::{Regex, Replacer};
use rocket::time::format_description::parse;
use std::collections::HashMap;
use std::str::FromStr;
use std::vec;
use std::{
    env::{self, Args},
    rc::Rc,
};

use crate::core::errs::SCSPErr;

/// Parser provides low-level apis to parse command line arguments
pub struct Parser {
    mappings: Rc<RefCell<HashMap<String, String>>>,
    abbrev_mappings: Rc<RefCell<HashMap<String, String>>>,
}

impl Parser {
    pub fn new(args: impl Iterator<Item = String>) -> Result<Self, SCSPErr> {
        let mut res = Parser {
            mappings: Rc::new(RefCell::new(HashMap::new())),
            abbrev_mappings: Rc::new(RefCell::new(HashMap::new())),
        };
        let mut is_key = true;
        let mut key: String = String::from("");
        let mut is_abbrev = false;
        for token in args {
            if is_key {
                key = String::from(token.as_str());

                is_key = false;
                if Self::is_abbrev(&token) {
                    is_abbrev = true;
                } else if Self::is_full(&token) {
                    is_abbrev = false;
                } else {
                    return Err(SCSPErr::new("expecting key"));
                }
            } else {
                is_key = true;
                if is_abbrev {
                    let name = key.replacen("-", "", 1);
                    res.abbrev_mappings
                        .borrow_mut()
                        .insert(name, token.to_owned());
                } else {
                    let name = key.replacen("--", "", 1);
                    res.mappings.borrow_mut().insert(name, token.to_owned());
                }
            }
        }
        if !is_key {
            // there should be even number of args
            // to form strict key-val pair
            return Err(SCSPErr::new("invalid number of args"));
        }
        Ok(res)
    }

    /// find value by full key
    /// don't include dash lines in the key
    pub fn find(&self, key: &str) -> Option<String> {
        let m: std::cell::Ref<'_, HashMap<String, String>> = self.mappings.borrow();
        let v = m.get(key);
        v.map(|s| s.clone())
    }

    /// find value by abbrevations
    fn find_abbrev(&self, key: &str) -> Option<String> {
        self.abbrev_mappings.borrow().get(key).map(String::clone)
    }

    pub fn find_i64(&self, key: &str) -> Option<i64> {
        self.find_num::<i64>(key)
    }

    pub fn find_num<T: FromStr>(&self, key: &str) -> Option<T> {
        let m = self.find(key);
        m.map(Self::parse_num)
        .flatten()
    }

    /// find value matched by either full key or abbrev key
    /// key_full has higher priority, i.e., if key_full matches, key_abbrev is 
    /// ignored even if there's a value associated with it.
    pub fn find_either(&self, key_full: &str, key_abbrev: &str) -> Option<String> {
        self.find(key_full).or(self.find_abbrev(key_abbrev))
    }

    /// same as find_either
    pub fn find_either_num<T: FromStr>(&self, key_full: &str, key_abbrev: &str) -> Option<T> {
        self.find_either(key_full, key_abbrev).map(Self::parse_num).flatten()
    }

    fn parse_num<T: FromStr>(s: String) -> Option<T> {
        let parsed = s.parse::<T>();
        if parsed.is_err() {
            None
        } else {
            match parsed {
                Ok(val) => Some(val),
                _ => None,
            }
        }
    }

    fn is_abbrev(token: &str) -> bool {
        // TODO: compile once
        let pattern = Regex::new(r"^-[a-zA-Z0-9]{1}$").unwrap();
        pattern.is_match(token)
    }

    fn is_full(token: &str) -> bool {
        // TODO: compile once
        let pattern = Regex::new(r"^--[\-a-zA-Z0-9]+").expect("failed to compile pattern");
        pattern.is_match(token)
    }
}

#[test]
fn test_parser() {
    let parser = Parser::new(
        vec![
            "--k1",
            "v1",
            "--k2",
            "v2",
            "-h",
            "127.0.0.1",
            "-p",
            "8080",
            "--num1",
            "1",
            "--long-arg",
            "long-val",
            "-l", "3"
        ]
        .into_iter()
        .map(|s| String::from(s)),
    )
    .unwrap();

    assert_eq!("v1", parser.find("k1").unwrap().as_str());
    assert_eq!("127.0.0.1", parser.find_abbrev("h").unwrap().as_str());
    assert_eq!(1, parser.find_either_num::<u16>("num1", "n").unwrap());
    assert_eq!("long-val", parser.find("long-arg").unwrap());
    assert_eq!(None, parser.find("non-exist"));

    assert_eq!(3, parser.find_either_num::<u16>("level", "l").unwrap())

}

#[test]
fn test_illegal_abbrev() {
    let illegal_parser = Parser::new(
        vec!["-illegal-arg", ""]
            .into_iter()
            .map(|s| String::from(s)),
    );
    assert!(illegal_parser.is_err());
}

#[test]
fn test_unbalance_arg() {
    let illegal_parser2 = Parser::new(vec!["--illegal-arg"].into_iter().map(|s| String::from(s)));
    assert!(illegal_parser2.is_err());
}
