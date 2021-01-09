use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Auth {
    token: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Bomb {
    position: String,
    state: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Map {
    current_spectators: i32,
    mode: String,
    num_matches_to_win_series: i32,
    phase: String,
    round: i32,
    round_wins: HashMap<i8, String>,
    souvenirs_total: i32,
    team_ct: HashMap<String, i8>,
    team_t: HashMap<String, i8>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PhaseCountDowns {
    phase: String,
    phase_ends_in: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Player {
    activity: String,
    forward: String,
    match_stats: HashMap<String, i16>,
    name: String,
    observer_slot: i32,
    position: String,
    spectarget: String,
    state: HashMap<String, serde_json::Value>,
    steamid: String,
    team: String,
    weapons: HashMap<String, serde_json::Value>,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Provider {
    appid: i32,
    name: String,
    steamid: String,
    timestamp: i64,
    version: i32,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct Round {
    phase: String,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct PostBody {
    auth: Auth,
    bomb: Bomb,
    map: Map,
    phase_countdowns: PhaseCountDowns,
    pub player: Player,
    provider: Provider,
    round: Round,
}

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct GameState {
    pub player: Player,
}

#[cfg(test)]
mod test {
    use crate::types::PostBody;
    use anyhow::Result;
    use std::fs;

    #[test]
    fn test_parser() -> Result<()> {
        let sample_post = fs::read_to_string("sample_post")?;
        assert!(serde_json::from_str::<PostBody>(&sample_post).is_ok());
        Ok(())
    }
}
