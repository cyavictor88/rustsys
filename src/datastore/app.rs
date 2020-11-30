
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc};



#[derive(Debug, Clone)]
pub struct App {
    shared: Arc<Mutex<HashMap<String, String>>>,
}
impl App {
    pub fn new() -> App {

        let mut map: HashMap<String, String> = HashMap::new();
        let shared = Arc::new( Mutex::new( map));
        App { shared: shared }
    }

    pub fn set(&self, key: String, value: String) {
                let mut state = self.shared.lock().unwrap();
                let prev = state.insert(
                    key,
                    value
                );
    }

    pub fn get(&self, key: &String) -> Option<String> {
        let state = self.shared.lock().unwrap();
        state.get(key).map(|sender| sender.clone())
    }

    pub fn list(&self)
    {
        let state = self.shared.lock().unwrap();
        for (key, value) in state.iter() {
        println!("{:?} / {:?}", key, value);
    }
}
  
}

fn do_it(map: &mut HashMap<String, String>) {

    map.clear();
}