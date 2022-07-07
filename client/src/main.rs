mod hash_cash;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use shared::model::{Challenge, ChallengeAnswer, ChallengeResult, Message, Subscribe};
use shared::hash_cash_model::{MD5HashCashInput, MD5HashCashOutput};
use crate::hash_cash::HashCash;

fn main() {
    let stream = TcpStream::connect("localhost:7878");
    match stream {
        Ok(mut stream) => {
            println!("Connected to server");
            send_message(&mut stream, &Message::Hello);
            loop {
                let record = receive_message(&mut stream);
                println!("{:?}", record);
                match record {
                    Message::Hello => {}
                    Message::Welcome(message) => {
                        println!("Welcome to the server, version {}", message.version);
                        send_message(&mut stream, &Message::Subscribe(Subscribe { name: "Henri".to_string() }));
                    }
                    Message::SubscribeResult(subscribe) => {
                        println!("{:?}", subscribe);
                    },
                    Message::PublicLeaderBoard(leaderboard) => {
                        println!("{:?}", leaderboard);
                    },
                    Message::Challenge(challenge) => {
                        message_challenge(&mut stream, challenge);
                    }
                    Message::RoundSummary(round_summary) => {
                        println!("{:?}", round_summary);
                    },
                    Message::EndOfGame(end_of_game) => {
                        println!("{:?}", end_of_game);
                    }
                    _ => {}
                }
            }
        }
        Err(err) => panic!("Cannot connect : {err}"),
    }


    fn send_message(stream: &mut TcpStream, message: &Message) {
        println!("Sending message: {:?}", message);
        let message = serde_json::to_string(&message).expect("Failed to serialize message");
        println!("Serialized message: {}", message);
        stream.write(&(message.len() as u32).to_be_bytes()).expect("Failed to write message length");
        stream.write(message.as_bytes()).expect("Failed to write message");
    }

    fn receive_message(stream: &mut TcpStream) -> Message {
        println!("Receiving message");
        let mut buffer = [0 as u8; 4];
        match stream.read(&mut buffer) {
            Ok(_) => {
                let buffer: u32 = u32::from_be_bytes(buffer);
                let mut new_buffer = vec![0; buffer as usize];
                match stream.read_exact(&mut new_buffer) {
                    Ok(_) => {
                        let message = from_utf8(&new_buffer).expect("Failed to read message");
                        println!("Received message: {}", message);
                        return serde_json::from_str(message).expect("Failed to deserialize message");
                    }
                    Err(err) => panic!("Failed to read message: {}", err),
                }
            },
            Err(e) => {
                println!("Failed to read message: {}", e);
            }
        }
        Message::Hello
    }

    fn message_challenge(stream: &mut TcpStream, challenge: Challenge){
        let challenge_answer: ChallengeAnswer;

        match challenge {
            Challenge::HashCashChallenge(input) => {
                let answer = run_hash_cash(input);
                challenge_answer = ChallengeAnswer::HashCashChallenge(answer);
            }
        }

        let challenge_result = ChallengeResult {
            result: challenge_answer,
            next_target: "".to_string(),
        };
        send_message(stream, &Message::ChallengeResult(challenge_result));
    }

    fn run_hash_cash(input: MD5HashCashInput) -> MD5HashCashOutput {
        let hash_cash = HashCash::new(input);
        hash_cash.run()
    }
}
