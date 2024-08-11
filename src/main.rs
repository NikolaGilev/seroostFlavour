use std::env;
use std::fs::File;
use std::path::Path;
use std::process::ExitCode;
use tiny_http::Server;
mod serve;
mod index;
mod lexer;

fn usage(program: &str) {
    eprintln!("Usage: {program} [SUBCOMMAND] [OPTIONS]");
    eprintln!("Subcommands:");
    eprintln!("    index <folder>                  index the <folder> and save the index to index.json file");
    eprintln!("    search <index-file>             check how many documents are indexed in the file (searching is not implemented yet)");
    eprintln!("    serve <index-file> [address]    start local HTTP server with Web Interface");
}


fn entry() -> Result<(), ()> {
    let mut args = env::args();
    let program = args.next().expect("path to program is provided");

    let subcommand = args.next().ok_or_else(|| {
        usage(&program);
        eprintln!("ERROR: No subcommand is provided");
    })?;

    match subcommand.as_str() {
        "index" => {
            let dir_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: No directory is provided for {subcommand} subcommand");
            })?;

            let mut tf_index = index::TermFreqIndex::new();
            index::tf_index_of_folder(Path::new(&dir_path), &mut tf_index)?;
            index::save_tf_index(&tf_index, "index.json")?;
        },
        "search" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: No path to index is provided for {subcommand} subcommand");
            })?; 
            index::check_index(&index_path)?;
        },
        "serve" => {
            let index_path = args.next().ok_or_else(|| {
                usage(&program);
                eprintln!("ERROR: No path to index is provided for {subcommand} subcommand");
            })?; 
            let index_file = File::open(&index_path).map_err(|err| {
                eprintln!("ERROR: Could not open index file {index_path}: {err}");
            })?;

            let tf_index: index::TermFreqIndex = serde_json::from_reader(index_file).map_err(|err| {
                eprintln!("ERROR: Could not parse index file {index_path}: {err}")
            })?;

            let address = args.next().unwrap_or("127.0.0.1:6969".to_string());

            let server = Server::http(&address).map_err(|err| {
                eprintln!("ERROR: Could not start HTTP server at {address}: {err}");
            })?;

            println!("INFO: Listening at http://{address}/");

            for request in server.incoming_requests() {
                // TODO: serve custom 500 in case of an error
                serve::serve_request(&tf_index, request).ok();
            }
        },
        _ => {
            usage(&program);
            eprintln!("ERROR: unknown subcommand {subcommand}");
            return Err(());
        }
    }

    Ok(())

}

fn main() -> ExitCode {
    match entry() {
        Ok(()) => ExitCode::SUCCESS,
        Err(()) => ExitCode::FAILURE,
    }
}