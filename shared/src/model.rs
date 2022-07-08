use crate::hash_cash_model::{MD5HashCashInput, MD5HashCashOutput};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message{
    Hello,
    Welcome(Welcome),
    Subscribe(Subscribe),
    SubscribeResult(SubscribeResult),
    PublicLeaderBoard(Vec<PublicPlayer>),
    Challenge(Challenge),
    ChallengeResult(ChallengeResult),
    RoundSummary(RoundSummary),
    EndOfGame(EndOfGame),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {}

#[derive(Debug, Serialize, Deserialize)]
pub struct Welcome {
    pub version: u8
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Subscribe {
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SubscribeResult{
    Ok,
    Err(SubscribeError)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublicPlayer {
    pub name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Challenge{
    MD5HashCash(MD5HashCashInput),
    //MonstrousMazeChallenge(MonstrousMazeInput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChallengeResult{
    pub result: ChallengeAnswer,
    pub next_target: String
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ChallengeAnswer{
    MD5HashCash(MD5HashCashOutput)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReportedChallengeResult{
    name: String,
    // value: JobValue
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RoundSummary{
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndOfGame{
    leader_board: Vec<PublicPlayer>
}