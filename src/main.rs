use std::fs;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;

fn main() {
   let listener =
        TcpListener::bind("127.0.0.1:6463").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
//test
    }

}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    let get = b"HTTP/1.1 200 OK\r\n";

    let (status_line, filename) = 
        if buffer.starts_with(get){
            ("HTTP/1.1 200 OK", "index.html")

        } else {
            ("HTTP/1.1 404 NOT FOUND!", "404.html")
       
        };

  
    let contents = fs::read_to_string(filename).unwrap();


    let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
    
    
    
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

  



}
  
