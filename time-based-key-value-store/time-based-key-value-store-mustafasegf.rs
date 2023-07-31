use std::collections::HashMap;
use std::cmp::Ordering;

struct TimeMap {
  map: HashMap<String, Vec<(i32, String)>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl TimeMap {
  fn new() -> Self {
    Self {
      map: HashMap::new()
    }
  }
  
  fn set(&mut self, key: String, value: String, timestamp: i32) {
    self
      .map
      .entry(key)
      .or_default()
      .push((timestamp, value));

    
    // println!("map: {:#?}", self.map);
  }
  
  fn get(&self, key: String, timestamp: i32) -> String {
    let nums =  match self.map.get(&key) {
      Some(nums) => nums,
      _ => return "".to_string(),
    };

    if nums[0].0 > timestamp {
      return "".to_string();
    }

    let mut lo = 0i32;
    let mut hi = nums.len() as i32 - 1;

    while lo <= hi {
      let mid = lo + (hi - lo) / 2 as i32;

      match nums[mid as usize].0.cmp(&timestamp) {
        Ordering::Equal => return nums[mid as usize].1.clone(),
        Ordering::Less => lo = mid + 1,
        Ordering::Greater => hi = mid - 1,
      }
    }

    return nums[lo as usize - 1].1.clone();
  }
}

/**
 * Your TimeMap object will be instantiated and called as such:
 * let obj = TimeMap::new();
 * obj.set(key, value, timestamp);
 * let ret_2: String = obj.get(key, timestamp);
 */
