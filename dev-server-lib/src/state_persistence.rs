use crate::{EventExt, StateExt};
use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SingleState<State, Msg>
where
    State: StateExt,
    Msg: EventExt,
{
    #[serde(deserialize_with = "State::deserialize")]
    original: State,
    #[serde(bound = "")]
    msgs: Vec<Msg>,
    #[serde(deserialize_with = "State::deserialize")]
    pub current: State,
}

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct StrippedSingleState<State>
where
    State: StateExt,
{
    #[serde(deserialize_with = "State::deserialize")]
    original: State,
    #[serde(deserialize_with = "State::deserialize")]
    current: State,
}

impl<State: StateExt, Msg: EventExt> Into<SingleState<State, Msg>> for StrippedSingleState<State> {
    fn into(self) -> SingleState<State, Msg> {
        SingleState {
            current: self.current,
            original: self.original,
            msgs: Vec::with_capacity(0),
        }
    }
}

impl<State: StateExt, Msg: EventExt> SingleState<State, Msg> {
    pub fn load(name: &str) -> Option<Self> {
        match SingleState::load_internal(name) {
            Ok(s) => s,
            Err(e) => {
                error!(
                    "Could not load session state for session '{}'. Exception: {}\n{}",
                    name,
                    e,
                    e.backtrace()
                );
                None
            }
        }
    }

    fn load_internal(name: &str) -> Result<Option<Self>> {
        let dir = std::env::current_dir()?;
        let file = dir.join(format!("{}-state.json", name));
        if !file.exists() {
            let msg = format!("No state found in {:?} will use initial", file);
            return Err(anyhow::Error::msg(msg));
        }
        let state = match Self::load_full_from_file(&file) {
            Ok(s) => Some(s),
            Err(e) => {
                error!("Could not load full state from file {:?}: {}", file, e);
                None
            }
        };
        let state = if state.is_none() {
            match Self::load_state_only_from_file(&file) {
                Ok(s) => Some(s),
                Err(e) => {
                    error!("Could not load partial state from file {:?}: {}", file, e);
                    None
                }
            }
        } else {
            state
        };
        if let Some(state) = state {
            Ok(Some(state))
        } else {
            info!("No state found in {:?} will use initial", file);
            Ok(None)
        }
    }

    /// Loads state and events from file
    fn load_full_from_file(file: &PathBuf) -> Result<SingleState<State, Msg>> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);
        let state: SingleState<State, Msg> = serde_json::from_reader(reader)?;
        Ok(state)
    }

    /// Loads just state without events from file
    fn load_state_only_from_file(file: &PathBuf) -> Result<SingleState<State, Msg>> {
        let file = File::open(file)?;
        let reader = BufReader::new(file);

        let state: StrippedSingleState<State> = serde_json::from_reader(reader)?;
        Ok(state.into())
    }

    pub fn save(&self, name: &str) -> Result<()> {
        let dir = std::env::current_dir()?;
        let file = dir.join(format!("{}-state.json", name));

        let mut file = File::open(file)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
}
