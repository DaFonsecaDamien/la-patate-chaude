use crate::hash_cash_model::{MD5HashCashInput, MD5HashCashOutput};

pub struct Hello {}

pub struct Welcome {
    version: u8
}

pub struct Subscribe {
    name: String
}

pub enum SubscribeError {
    AlreadyRegistered,
    InvalidName
}

pub enum SubscribeResult{
    Ok,
    Err(SubscribeError)
}

pub struct PublicPlayer {
    name: String,
    stream_id: String,
    score: i32,
    steps: u32,
    is_active: bool,
    total_used_time: f64
}

pub struct PublicLeaderBoard {
    player_list: Vec<PublicPlayer>
}

pub enum Challenge{
    HashCashChallenge(MD5HashCashInput)
}

pub struct ChallengeResult{
    result: ChallengeAnswer,
    next_target: String
}

pub enum ChallengeValue{
    Unreachable,
    Timeout,
    BadResult { used_time: f64, next_target: String },
    Ok { used_time: f64, next_target: String }
}

pub enum ChallengeAnswer{
    HashCashChallenge(MD5HashCashOutput)
}

pub struct ReportedChallengeResult{
    name: String,
    // value: JobValue
}

pub struct RoundSummary{
    challenge: String,
    chain: Vec<ReportedChallengeResult>
}

pub struct EndOfGame{
    leader_board: PublicLeaderBoard
}