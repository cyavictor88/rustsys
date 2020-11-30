
use std::collections::{BTreeMap, HashMap};
use std::sync::{Arc, Mutex};
use tokio::sync::{mpsc};



#[derive(Debug, Clone)]
pub struct Neighbour {
    shared: Arc<Mutex<HashMap<String,  mpsc::Sender<String>>>>,
}
impl Neighbour {
    pub fn new() -> Neighbour {

        let mut map: HashMap<String,  mpsc::Sender<String>> = HashMap::new();
        let shared = Arc::new( Mutex::new( map));
        Neighbour { shared: shared }
    }

    pub fn set(&self, key: String, value:  mpsc::Sender<String>) {
                let mut state = self.shared.lock().unwrap();
                let prev = state.insert(
                    key,
                    value
                );
    }

    pub fn get(&self, key: &String) -> Option<mpsc::Sender<String>> {
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

    pub fn broadcast(&self, appname: String, host: String){
        let state = self.shared.lock().unwrap();
        for (key, value) in state.iter() {
            let info = format!("UPDATEAPPS {} {}",appname,host);
            value.send(info.to_string());
        }
    }
  
}

fn do_it(map: &mut HashMap<String, String>) {

    map.clear();
}