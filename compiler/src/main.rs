use clap::Parser;

use crate::parser::{Parsable, TokenReader};
mod tokenizer;
mod parser;
mod xml;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    path: String
}

fn main() {
    let args = Args::parse();
    // read file content from path
    let path = args.path;
    let tokens = tokenizer::tokenize(&path);
    println!("{:?}",tokens);
    let parsed_class = parser::structures::Class::try_parse(&TokenReader{tokens}, 0).unwrap();
    println!("{:?}",parsed_class);
    let node = parser::Node::Class(parsed_class.0);
    let output = xml::convert_node(node);
    println!("{}",output);
}
