use std::time::Duration;
use std::{fs, thread};
use std::net::{TcpListener, TcpStream};
use std::io::{prelude::*, BufReader};
use webserver::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(4) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, file_path) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let content = fs::read_to_string(file_path).unwrap();
    let length = content.len();
    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, content);

    stream.write_all(response.as_bytes()).unwrap();
    
    // println!("Http request is {:#?}", http_result);
}
