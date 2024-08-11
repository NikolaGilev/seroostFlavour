use core::str;
use std::{fs::File, path::Path};
use tiny_http::{Header, Method, Request, Response, StatusCode};

use crate::{index::{TermFreq, TermFreqIndex}, lexer::Lexer};

fn serve_static_file(request: Request, file_path: &str, content_type: &str) -> Result<(), ()> {
    let content_type_header = Header::from_bytes("Content Type", content_type)
        .expect("That we didnt put any garbage in the headers");
    let file = File::open(file_path).map_err(|err| {
        eprintln!("ERROR: Could not open file {file_path}: {err}")
    })?;
    let response = Response::from_file(file).with_header(content_type_header);
    request.respond(response).map_err(|err| {
        eprintln!("ERROR: Could not serve static file {file_path}: {err}");
    })
}

fn serve_404(request: Request) -> Result<(), ()> {
    request.respond(Response::from_string("404").with_status_code(StatusCode(404)))
        .map_err(|err| {
            eprintln!("ERROR: Could not serve a request: {err}");
        })
}

fn tf(t: &str, d: &TermFreq) -> f32 {
    let a = d.get(t).cloned().unwrap_or(0) as f32;
    let b = d.values().map(| x| *x).sum::<usize>() as f32;
    // let b = d.iter().map(|(_, f)| *f).sum::<usize>() as f32;
    a/b
}

fn idf(t: &str, d: &TermFreqIndex) -> f32 {
    let n = d.len() as f32;
    let m = d.values().filter(|tf| tf.contains_key(t)).count().max(1) as f32;
    return (n/m).log10();
}

fn serve_api_search(tf_index: &TermFreqIndex, mut request: Request) -> Result<(), ()>{
    let mut buf = Vec::new();
    request.as_reader().read_to_end(&mut buf).map_err(|err| {
        eprintln!("ERROR: Could not read the body of the request: {err}");
    })?;

    let body = str::from_utf8(&buf).map_err(|err| {
        eprintln!("ERROR: Could not interpret body as UTF-* string: {err}");
    })?.chars().collect::<Vec<_>>();

    let mut result = Vec::<(&Path, f32)>::new();

    for (path, tf_table) in tf_index {
        let mut rank = 0f32;
        for token in Lexer::new(&body) {
            rank += tf(&token, &tf_table) * idf(&token, &tf_index);
        }
        result.push((path, rank));
    }
    result.sort_by(|(_, rank1), (_, rank2)| rank1.partial_cmp(rank2).unwrap());
    result.reverse();

    let json = serde_json::to_string(&result.iter().take(20).collect::<Vec<_>>())
        .map_err(|err| {
            eprintln!("ERROR: Could not convert search results to JSON: {err}");
        })?;
    
    let content_type_header = Header::from_bytes("Content-Type", "application/json")
        .expect("That we didn't put any garbage in the headers");

    let response = Response::from_string(&json).with_header(content_type_header);
    request.respond(response).map_err(|err| {
        eprintln!("ERROR: Could not serve a request: {err}")
    })
}



pub fn serve_request(tf_index: &TermFreqIndex, request: Request) -> Result<(), ()>{
    println!("INFO: received request! methid: {:?}, url: {:?}", request.method(), request.url());
    match (request.method(), request.url()) {
        (Method::Post, "/api/search") => {
            serve_api_search(tf_index, request)
        }
        (Method::Get, "/index.js") => {
            serve_static_file(request, "index.js", "text/javascript; charset=utf-8")
        }
        (Method::Get, "/") => {
            serve_static_file(request, "index.html", "text/html; charset=utf-8")
        }
        _ => {
            serve_404(request)
        }
    }
}