use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use chapter_20_webserver::ThreadPool;

fn main() {
    let ip = "127.0.0.1:7878";
    let listener = TcpListener::bind(ip).unwrap();

    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    println!("Connection established {}", stream.peer_addr().unwrap());
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = match &request_line[..] {
        "GET / HTTP/1.1" => build_ok_response(),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            build_ok_response()
        }
        _ => build_not_found_response(),
    };

    stream.write_all(response.as_bytes()).unwrap();
}

// fn print_request(buf_reader: &BufReader<&mut TcpStream>) {
//     let http_request: Vec<_> = buf_reader
//         .lines()
//         .map(|result| result.unwrap())
//         .take_while(|line| !line.is_empty())
//         .collect();
//     println!("Request {:#?}", http_request);
// }

fn build_ok_response() -> String {
    const HTML_PAGE: &str = "hello.html";
    const CODE: i32 = 200;
    const MESSAGE: &str = "OK";
    build_http_response(HTML_PAGE, CODE, MESSAGE)
}

fn build_not_found_response() -> String {
    const HTML_PAGE: &str = "404.html";
    const CODE: i32 = 404;
    const MESSAGE: &str = "NOT FOUND";
    build_http_response(HTML_PAGE, CODE, MESSAGE)
}

fn build_http_response(html_page: &str, code: i32, status: &str) -> String {
    let status_line = "HTTP/1.1 ".to_string() + code.to_string().as_str() + status;
    let content = fs::read_to_string(html_page).unwrap();
    let length = content.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    response
}
