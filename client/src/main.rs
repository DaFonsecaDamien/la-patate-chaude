use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use shared::model::Welcome;

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    println!("Success");
    match stream {
        Ok(mut stream) => {
            let message= "\"Hello\"".as_bytes();
            // let mes = Hello();
            stream.write_all(&(message.len() as u32).to_be_bytes()).unwrap();
            let _response = stream.write_all(&message);
            let mut buffer = [0 as u8; 29];
            match stream.read_exact(&mut buffer){
                Ok(_) => {
                    let text = from_utf8(&buffer).unwrap();
                    println!("{}", text);
                },
                Err(e)=>{
                    println!("{}", e);
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}