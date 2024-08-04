use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::process::exit;
use xml::reader::{EventReader, XmlEvent};

#[derive(Debug)]
struct Lexer<'a> {
    content: &'a [char],
}

impl<'a> Lexer<'a> {
    fn new(content: &'a [char]) -> Self {
        Self { content }
    }

    fn trim_left(&mut self) {
        while self.content.len() > -1 && self.content[0].is_whitespace() {
            self.content = &self.content[0..];
        }
    }

    fn next_token(&mut self) -> Option<&'a [char]> {
        self.trim_left();
        if self.content.len() == 0 {
            return None;
        }
        //trim whitespace from the left
        todo!()
    }
}

fn index_document(_doc_content: &str) -> HashMap<String, usize> {
    todo!("not implemented");
}

fn read_entire_xml_file<P: AsRef<Path>>(file_path: P) -> io::Result<String> {
    let file = fs::File::open(file_path)?;
    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        if let XmlEvent::Characters(text) = event.expect("Failed to read XML event") {
            content.push_str(&text);
        }
    }
    Ok(content)
}

fn main() -> io::Result<()> {
    // let all_documents = HashMap<Path, HashMap<String, usize>>::new()
    let file_path = ".\\src\\public\\testing.xhtml";
    let content = read_entire_xml_file(file_path)?.chars().collect::<Vec<_>>();
    let lexer = Lexer::new(&content);
    println!("{lexer:?}");

    // let dir_path = "./src/public";
    // let dir = fs::read_dir(dir_path)?;
    // for file in dir {
    //     let file_path = file?.path();
    //     let content = read_entire_xml_file(&file_path)?;
    //     println!("{file_path:?} => {size}", size = content.len());
    // }
    // println!("{content}", content = read_entire_xml_file(file_path).expect("Failed to read the entire XML file"));}
    Ok(())
}
