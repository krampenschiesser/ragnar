use ragnar_lib::{AppState, AppEvent};
use std::collections::HashMap;
use crate::error::Error;
use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, Read, BufWriter};
use anyhow::Result;
use crate::{StateExt, EventExt};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct SingleState<State, Msg>
    where State: AppState + Clone + serde::Serialize,
          Msg: AppEvent + Clone+ serde::Serialize
{
    original: State,
    msgs: Vec<Msg>,
    current: State,
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct StrippedSingleState<State>
    where State: AppState + Clone,
{
    original: State,
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
    pub fn load(name: &str, initial: State) -> Result<Self> {
        let default_value = Ok(SingleState {
            original: initial.clone(),
            msgs: Vec::new(),
            current: initial.clone(),
        });
        let dir = std::env::current_dir()?;
        let file = dir.join(format!("{}-state.json", name));
        if !file.exists() {
            info!("No state found in {:?} will use initial", file);
            return default_value;
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
        } else { state };
        if let Some(state) = state {
            Ok(state)
        } else {
            info!("No state found in {:?} will use initial", file);
            default_value
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
