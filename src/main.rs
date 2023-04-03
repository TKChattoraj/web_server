use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::ThreadPool;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4); 
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        println!("Stream: {:?}", stream);
        pool.execute(|| {handle_connection(stream)});
   }
   printl1n!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    //println!("Request: {:#?}", http_request);
    println!("Request: {:#?}", request_line);



    //let r: Vec<_> = http_request[0].split(' ').collect();


    let r: Vec<_> = request_line.split(' ').collect();
    println!("{:?}", r);
    let request_tuple=(r[0], r[1], r[2]);
    println!("Request: {:?}", request_tuple);
    if let ("GET", _, _) = request_tuple {
       send_response_from_get(stream, request_tuple)
    }
    else {
        send_404_response(stream)
    }

}

fn send_404_response(mut stream: TcpStream) {
    let status_line="HTTP/1.1 404 NOT FOUND";
    let contents = fs::read_to_string("404.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

fn send_response_from_get(mut stream: TcpStream, request_tuple: (&str, &str, &str)) {
    println!("in the send_response");
    println!("{:?}", request_tuple);

    match request_tuple {
        ("GET", "/titan", "HTTP/1.1") => send_titan_response(stream),
        ("GET", "/charlie", "HTTP/1.1")=>send_charlie_response(stream),
        ("GET", "/", "HTTP/1.1")=>send_generic_response(stream),
        _ =>send_404_response(stream),
    }
}
fn send_titan_response(mut stream: TcpStream) {
    let status_line="HTTP/1.1 200 OK";
    let contents = fs::read_to_string("titan.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn send_charlie_response(mut stream: TcpStream) {
    let status_line="HTTP/1.1 200 OK";
    let contents = fs::read_to_string("charlie.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
fn send_generic_response(mut stream: TcpStream) {
    let status_line="HTTP/1.1 200 OK";
    let contents = fs::read_to_string("index.html").unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    thread::sleep(Duration::from_secs(10));
    stream.write_all(response.as_bytes()).unwrap();
}