use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use shared::model::{Message, Welcome};

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    println!("Success");
    match stream {
        Ok(mut stream) => {
            let message= "\"Hello\"".as_bytes();
            stream.write_all(&(message.len() as u32).to_be_bytes()).unwrap();
            let _response = stream.write_all(&message);
            let mut buffer = [0 as u8; 4];
            match stream.read_exact(&mut buffer){
                Ok(_) => {
                    let buffer: u32 = u32::from_be_bytes(buffer);
                    let mut new_buffer = vec![0; buffer as usize];
                    match stream.read_exact(&mut new_buffer){
                        Ok(_) =>{
                            // let response = from_utf8(&new_buffer).unwrap();
                            let message = std::str::from_utf8(&new_buffer).expect("failed to parse message");
                            let record: Message = serde_json::from_str(&message).expect("failed to serialize message");
                            match record {
                                Message::Hello => {}
                                Message::Welcome(message) => {}
                                Message::Subscribe(..) => {}
                                Message::SubscribeResult(subscribe) => {}
                            }
                        },
                        Err(e)=>{
                            println!("{}", e);
                        }
                    }
                },
                Err(e)=>{
                    println!("{}", e);
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }
}

fn gameLoop(){

}