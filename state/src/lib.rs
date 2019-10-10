use serde::{Serialize, Deserialize, Serializer};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::any::{TypeId, Any};

#[derive(Debug)]
pub struct GlobalState<T: Any + Serialize + DeserializeOwned> {
    map: HashMap<TypeId,Box<T>>,
}

impl <T: Any + Serialize + DeserializeOwned> GlobalState<T> {
    pub fn register(&mut self, data: T) {
        self.map.insert(data.type_id(),Box::new(data));
    }
}

impl<T: Any + Serialize + DeserializeOwned> Default for GlobalState<T> {
    fn default() -> Self {
        GlobalState { map: HashMap::new() }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default, Deserialize, Serialize)]
    struct Module1State {}

    #[derive(Debug, Default, Deserialize, Serialize)]
    struct Module2State {}

    #[test]
    fn state_register() {
        let mut state: GlobalState<Any + Serialize + DeserializeOwned> = GlobalState::default();
        state.register(Module1State {});
        state.register(Module2State {});
    }
}
