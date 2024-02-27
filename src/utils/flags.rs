use core::cell::RefCell;
use regex::{Regex, Replacer};
use std::collections::HashMap;
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

    fn is_abbrev(token: &str) -> bool {
        // TODO: compile once
        let pattern = Regex::new(r"^-[a-zA-Z0-9]{1}").unwrap();
        pattern.is_match(token)
    }

    fn is_full(token: &str) -> bool {
        // TODO: compile once
        let pattern = Regex::new(r"^--[a-zA-Z0-9]+").expect("failed to compile pattern");
        pattern.is_match(token)
    }
}

#[test]
fn test_parser() {
    let parser = Parser::new(
        vec!["--k1", "v1", "--k2", "v2", "-h", "127.0.0.1", "-p", "8080"]
            .into_iter()
            .map(|s| String::from(s)),
    )
    .unwrap();

    assert_eq!("v1", parser.find("k1").unwrap().as_str());
    assert_eq!("127.0.0.1", parser.find_abbrev("h").unwrap().as_str());
}
