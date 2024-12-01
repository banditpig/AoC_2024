use itertools::Itertools;
use load_file::load_str;
use std::collections::HashMap;
use std::fmt::Debug;
use std::panic::panic_any;
use std::process::Termination;

pub fn load_input(file: &str) -> Vec<&'static str> {
    let v = load_str!(file).split('\n').collect::<Vec<&str>>();
    v
}