use std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

use crate::input::Opts;
use crate::request::Request;

mod client;
mod constants;
mod input;
mod request;
mod response;

#[tokio::main]
async fn main() {
    let opts = Opts::read();
    let request = Request::new(&opts.url);
    println!("{}", request.to_string());
    let mut stream = TcpStream::connect("120.232.145.185:80").unwrap();
    stream.write(request.to_string().as_bytes()).unwrap();
    let mut buffer = [0;10240]; 
    stream.read(&mut buffer).unwrap();
    println!("{}", str::from_utf8(&buffer).unwrap());
}
