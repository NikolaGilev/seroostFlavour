use std::{collections::HashMap, fs::File, path::{Path, PathBuf}};


pub type TermFreq = HashMap::<String, usize>;
pub type TermFreqIndex = HashMap<PathBuf, TermFreq>;


// fn parse_entire_xml_file(file_path: &Path) -> Result<String, ()> {
//     let file = File::open(file_path).map_err(|err| {
//         eprintln!("ERROR: Could not open file {file_path}: {err}", file_path = file_path.display());
//     })?;

//     let er = EventReader::new(file);
//     let mut content = String::new();
//     for event in er.into_iter() {
//         let event = event.map_err(|err| {
//             let TextPosition {row, column} = err.position();
//             let msg = err.msg();
//             eprintln!("{file_path}:{row}:{column}: ERROR: {msg}", file_path = file_path.display());
//         })?;

//         if let XmlEvent::Characters(text) = event {
//             content.push_str(&text);
//             content.push_str(" ");
//         }
//     }
//     Ok(content)
// }

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

pub fn tf_index_of_folder(dir_path: &Path, tf_index: &mut TermFreqIndex) -> Result<(), ()>{
    todo!()
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

