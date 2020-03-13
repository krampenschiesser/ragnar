use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::Arc;
use threadpool::ThreadPool;

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq)]
struct State {
    data: String,
}

struct NonConeableArc<T>(Arc<T>);

impl<T> Deref for NonConeableArc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> NonConeableArc<T> {
    pub fn new(arc: Arc<T>) -> Self {
        NonConeableArc(arc)
    }
}

fn run_it() {
    let pool = ThreadPool::new(8);

    let mut state = State::default();
    state.data = "hello".into();

    let state = Arc::new(state);

    let max = 4;
    for i in 0..max {
        let local = NonConeableArc::new(state.clone());
        pool.execute(move || {
            println!("{:?}", local.data);
        });
    }
    pool.join();

    let mut state: State = match Arc::try_unwrap(state) {
        Ok(state) => state,
        Err(arc_state) => {
            error!("We still have {} strong references to arc although it should be 0", Arc::strong_count(&arc_state));
            let reference: &State = &arc_state;
            reference.clone()
        }
    };
    state.data = "world".into();

    let state = Arc::new(state);
    for i in 0..max {
        let local = NonConeableArc::new(state.clone());
        pool.execute(move || {
            println!("{:?}", local.data);
        });
    }
    pool.join();
}


#[cfg(test)]
mod tests {
    use log::LevelFilter;

    use super::*;

    #[test]
    fn run() {
        println!("1");
        let _ = env_logger::builder().is_test(true).default_format().filter(None, LevelFilter::Debug).try_init().unwrap();
        println!("2");
        info!("starting");
        run_it();
    }
}
