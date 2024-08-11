use std::{collections::HashMap, fs::{self, File}, path::{Path, PathBuf}};

use xml::reader::{XmlEvent, EventReader};
use xml::common::{Position, TextPosition};

use crate::lexer::Lexer;


pub type TermFreq = HashMap::<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;


fn parse_entire_xml_file(file_path: &Path) -> Result<String, ()> {
    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: Could not open file {file_path}: {err}", file_path = file_path.display());
    })?;

    let er = EventReader::new(file);
    let mut content = String::new();
    for event in er.into_iter() {
        let event = event.map_err(|err| {
            let TextPosition {row, column} = err.position();
            let msg = err.msg();
            eprintln!("{file_path}:{row}:{column}: ERROR: {msg}", file_path = file_path.display());
        })?;

        if let XmlEvent::Characters(text) = event {
            content.push_str(&text);
            content.push_str(" ");
        }
    }
    Ok(content)
}

pub fn save_tf_index(tf_index: &TermFreqIndex, index_path: &str) -> Result<(), ()> {
    println!("Saving {index_path}...");

    let index_file = File::create(index_path).map_err(|err| {
        eprint!("ERROR: Could not create index file {index_path}: {err}");
    })?;

    serde_json::to_writer(index_file, tf_index).map_err(|err| {
        eprintln!("ERROR: could not serialize index into file {index_path}: {err}");
    })?;

    Ok(())
}

pub fn tf_index_of_folder(dir_path: &Path, tf_index: &mut TermFreqIndex) -> Result<(), ()> {
    let dir = fs::read_dir(dir_path).map_err(|err| {
        eprintln!("ERROR: could not open directory {dir_path} for indexing: {err}",
                  dir_path = dir_path.display());
    })?;

    'next_file: for file in dir {
        let file = file.map_err(|err| {
            eprintln!("ERROR: could not read next file in directory {dir_path} during indexing: {err}",
                      dir_path = dir_path.display());
        })?;

        let file_path = file.path();

        let file_type = file.file_type().map_err(|err| {
            eprintln!("ERROR: could not determine type of file {file_path}: {err}",
                      file_path = file_path.display());
        })?;

        if file_type.is_dir() {
            tf_index_of_folder(&file_path, tf_index)?;
            continue 'next_file;
        }

        // TODO: how does this work with symlinks?

        println!("Indexing {:?}...", &file_path);

        let content = match parse_entire_xml_file(&file_path) {
            Ok(content) => content.chars().collect::<Vec<_>>(),
            Err(()) => continue 'next_file,
        };

        let mut tf = TermFreq::new();

        for term in Lexer::new(&content) {
            if let Some(freq) = tf.get_mut(&term) {
                *freq += 1;
            } else {
                tf.insert(term, 1);
            }
        }

        tf_index.insert(file_path, tf);
    }

    Ok(())
}


pub fn check_index(index_path: &str) -> Result<(),()> {
    println!("Reading {index_path} index file...");

    let index_file = File::open(index_path).map_err(|err| {
        eprintln!("ERROR: could not open index file {index_path}: {err}");
    })?;
    
    let tf_index: TermFreqIndex = serde_json::from_reader(index_file).map_err(|err| {
        eprint!("ERROR: could not parse index file {index_path}: {err}");
    })?;

    println!("{index_path} contains {count} files", count = tf_index.len());
    
    Ok(())
}

