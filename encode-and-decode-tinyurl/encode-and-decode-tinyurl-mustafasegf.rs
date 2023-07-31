struct Codec {
	store: HashMap<String, String>
}

use std::collections::HashMap;
use rand::{distributions::Alphanumeric, Rng};

/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
  fn new() -> Self {
    Self {
      store: HashMap::new(),
    }
  }

  // Encodes a URL to a shortened URL.
  fn encode(&mut self, longURL: String) -> String {
    loop {
      let key: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect();
        
      match self.store.contains_key(&key) {
        true => continue,
        false => {
          self.store.insert(key.clone(), longURL);
          break key;
        }
      }
    }
  }

  // Decodes a shortened URL to its original URL.
  fn decode(&self, shortURL: String) -> String {
    return self.store.get(&shortURL).unwrap().to_string();
  }
}

/**
 * Your Codec object will be instantiated and called as such:
 * let obj = Codec::new();
 * let s: String = obj.encode(strs);
 * let ans: VecVec<String> = obj.decode(s);
 */
