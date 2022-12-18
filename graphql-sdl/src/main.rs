use rust_web_example::graphql::graphql_schema;
use std::fs;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let file_path = "./resource/schema.graphql";

    if Path::new(file_path).exists() {
        fs::remove_file(file_path).unwrap();
    }

    let mut sdl_file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

    let generated_schema = graphql_schema::generate_schema();

    write!(&mut sdl_file, "{}", &generated_schema.sdl()).unwrap();
    println!("{}", &generated_schema.sdl());
    println!("Graphql SDL created to {}", file_path);
}
