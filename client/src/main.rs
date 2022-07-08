mod hash_cash;
mod monstrous_maze;

use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use rand::Rng;
use shared::model::{Challenge, ChallengeAnswer, ChallengeResult, Message, PublicPlayer, Subscribe};
use crate::hash_cash::HashCash;
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    #[clap(short, long, value_parser)]
    pub player_name: String,
}

fn main() {
    let args: Args = Args::parse();
    println!("{:?}", args);
    let stream = TcpStream::connect("localhost:7878");
    let mut board: Vec<PublicPlayer> = vec![];
    let mut current_player = 0;
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
                        send_message(&mut stream, &Message::Subscribe(Subscribe {name: args.player_name.clone() }));
                    }
                    Message::SubscribeResult(subscribe) => {
                        println!("{:?}", subscribe);
                    },
                    Message::PublicLeaderBoard(leaderboard) => {
                        println!("{:?}", leaderboard);
                        board = leaderboard;
                    },
                    Message::Challenge(challenge) => {
                        message_challenge(&mut stream, challenge, &board, current_player);
                        current_player += 1;
                        if current_player > board.len() - 1 {
                            current_player = 0;
                        }
                    },
                    Message::Subscribe(subscribe) => {
                        println!("Suscribe: {:?}", subscribe);
                    }
                    Message::ChallengeResult(challenge_result) => {
                        println!("Challenge result received: {:?}", challenge_result);
                    },
                    Message::RoundSummary(round_summary) => {
                        println!("{:?}", round_summary);
                    },
                    Message::EndOfGame(end_of_game) => {
                        println!("{:?}", end_of_game);
                        break;
                    }
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
                println!("Received message length: {}", u32::from_be_bytes(buffer));
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

    fn message_challenge(stream: &mut TcpStream, challenge: Challenge, leaderboard: &Vec<PublicPlayer>, current_player: usize){
        let mut rng = rand::thread_rng();
        let mut index = rng.gen_range(0..leaderboard.len());
        let challenge_answer: ChallengeAnswer;

        match challenge {
            Challenge::MD5HashCash(input) => {
                let answer = HashCash::new(input).run();
                challenge_answer = ChallengeAnswer::MD5HashCash(answer);
            }
        }

        while current_player == index {
            index = rng.gen_range(0..leaderboard.len());
        }

        let challenge_result = ChallengeResult {
            result: challenge_answer,
            next_target: leaderboard[index].name.clone()
        };

        println!("Sending challenge result: {:?}", challenge_result);

        send_message(stream, &Message::ChallengeResult(challenge_result));
    }
}
