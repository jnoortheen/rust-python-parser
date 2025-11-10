extern crate divan;
extern crate python_parser;

use divan::Bencher;
// use pyo3::prelude::*;
use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

use python_parser::visitors::printer::format_module;
use python_parser::{file_input, make_strspan};

pub fn get_big_py_file() -> String {
    let dir = env::temp_dir();

    let path = dir.join("xonsh-rd-parser-test-file.py");

    if !path.exists() {
        let mut file = std::fs::File::create(&path).unwrap();
        for idx in 0..10000 {
            writeln!(file, r#"x_{idx} = {idx} + 1"#).unwrap();
            writeln!(file, r#"print(x_{idx})"#).unwrap();
            writeln!(file, r#"assert x_{idx} == {idx} + 1"#).unwrap();
        }
    }
    path.to_str().unwrap().to_string()
}

fn main() {
    // Run registered benchmarks.
    divan::main();
}

fn parse(file_name: &str) {
    let mut file = File::open(file_name).expect("Could not open file");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Could not read file");
    let (rest, ast) = file_input(make_strspan(&content)).unwrap();
    //println!("{:?}", ast);
    let output = format_module(&ast);
    if rest.fragment.0.len() > 0 {
        println!("\nUnparsed: {:?}\n\n", rest.fragment.0)
    }
    // println!("{}", output);
}

#[divan::bench()]
fn python_parser(bencher: Bencher) {
    let file_name = get_big_py_file();
    println!("Parsing file: {}", &file_name);

    // Python::initialize();
    // Python::attach(|py| {
    bencher.bench_local(move || {
        parse(file_name.as_str());
    });
    // })
}
