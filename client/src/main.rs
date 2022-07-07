mod hash_cash;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::process::Output;
use std::str::from_utf8;
use rand::Rng;
use shared::model::{Challenge, ChallengeAnswer, ChallengeResult, Message, Subscribe, Welcome};
use shared::hash_cash_model::MD5HashCashInput;
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
                    Message::RoundSummary(roundSummary) => {
                        println!("{:?}", roundSummary);
                    },
                    Message::EndOfGame(endOfGame) => {
                        println!("{:?}", endOfGame);
                    }
                    _ => {}
                    //HashCash::new(MD5HashCashInput { complexity: 9, message: "hello".parse().unwrap() }).run();
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
        let mut rng = rand::thread_rng();

        match challenge {
            Challenge::HashCashChallenge(input) => {
                let mut hash_cash = HashCash::new(input);
                let answer = hash_cash.run();
                challenge_answer = ChallengeAnswer::HashCashChallenge(answer);
            }
        }

        let mut index = rng.gen_range(0..game_state.players.len());
        while game_state.players[index].name == game_state.name {
            index = rng.gen_range(0..game_state.players.len());
        }

        let next_target = game_state.players[index].name.clone();

        let challenge_result = ChallengeResult {
            result: challenge_answer,
            next_target,
        };
        send_message(stream, &Message::ChallengeResult(challenge_result));
    }

    fn run_hash_cash(input: MD5HashCashInput) {
        let mut hash_cash = HashCash::new(input);
        hash_cash.run();
    }
}
