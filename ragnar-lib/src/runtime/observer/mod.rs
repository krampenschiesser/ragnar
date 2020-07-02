use crate::runtime::observer::timingcategory::RuntimeTimingCategory;
use std::borrow::Cow;
use std::sync::Arc;
use std::time::{Duration, Instant};

pub mod timingcategory;

pub struct RuntimeObserver {
    // fn time(&self, name: &str, method: impl Fn());
// fn time_with_result<R, E>(&self, name: &str, method: impl Fn() -> Result<R, E>);
}

#[derive(Clone, Debug)]
pub struct Timer {
    start: Instant,
    name: Cow<'static, str>,
}

impl Timer {
    pub fn with_start<T: Into<Cow<'static, str>>>(start: Instant, name: T) -> Self {
        Self {
            start,
            name: name.into(),
        }
    }
    pub fn new<T: Into<Cow<'static, str>>>(name: T) -> Self {
        Self::with_start(Instant::now(), name)
    }

    pub fn stop(&self) -> Duration {
        Instant::now().duration_since(self.start)
    }
}

impl RuntimeObserver {
    pub fn observe_time(&self, category: RuntimeTimingCategory, timer: Timer) {
        let duration = timer.stop();
    }
}
