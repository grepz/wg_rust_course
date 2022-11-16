use std::{
    collections::{HashMap, BTreeMap, HashSet},
    collections::hash_map::Entry as HashMapEntry,
    collections::btree_map::Entry as BTreeMapEntry,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Cache<'a> {
    map: HashMap<&'a str, (&'a usize, Instant)>,
    ttl: BTreeMap<Instant, HashSet<&'a str>>,
}

impl Default for Cache<'static> {
    fn default() -> Self {
        Self::new()
    }
}

impl Cache<'static> {
    pub fn new() -> Self {
        Self {
            ttl: BTreeMap::new(),
            map: HashMap::new()
        }
    }
}

impl<'a> Cache<'a> {
    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }
}

impl<'a> Cache<'a> {

    pub fn insert(&mut self, key: &'a str, bs: &'a usize, ttl: Duration) ->
        Option<(&'a usize, Instant)>
    {
        let tstamp = Instant::now() + ttl;
        let (old, new) =
            match self.map.entry(key) {
                HashMapEntry::Occupied(mut oe) => {
                    let (old_bs, old_tstamp) = oe.insert((bs, tstamp));
                    self.ttl.entry(old_tstamp)
                        .and_modify(|hs| {
                            hs.remove(key);
                        });
                    (Some((old_bs, old_tstamp)), (bs, tstamp))
                },
                HashMapEntry::Vacant(_ve) => {
                    self.map.insert(key, (bs, tstamp));
                    (None, (bs, tstamp))
                }
            };

        let _ttl_result =
            self.ttl
            .entry(tstamp)
            .and_modify(|hs| {
                hs.insert(key);
            })
            .or_insert_with(|| {
                let mut hs = HashSet::new();
                hs.insert(key);
                hs
            });
        match old {
            Some((_, old_tstamp)) => {
                match self.ttl.entry(old_tstamp) {
                    BTreeMapEntry::Occupied(
                        mut oe
                    ) => {
                        let w = oe.get_mut();
                        if w.is_empty() {
                            oe.remove_entry();
                        }
                    },
                    BTreeMapEntry::Vacant(_ve) => (),
                }
            },
            None => {}
        }
        Some(new)
    }


    pub fn remove(&mut self, key: &'a str) ->
        Option<(&'a usize, Instant)>
    {
        match self.map.remove(key) {
            Some((sz, inst)) => {
                self.ttl
                    .entry(inst)
                    .and_modify(|hs| {
                        hs.remove(key);
                    });
                match self.ttl.entry(inst) {
                    BTreeMapEntry::Occupied(mut oe) => {
                        let w = oe.get_mut();
                        if w.is_empty() {
                            oe.remove_entry();
                        }
                    },
                    BTreeMapEntry::Vacant(_ve) => (),
                }
                Some((sz, inst))
            },
            None => None
        }
    }

    pub fn get(&'a self, key: &'a str) ->
        Option<(&'a usize, Instant)>
    {
        let result = self.map
            .get(key);
        Some(*result.unwrap())
    }

    pub fn expire(&mut self) {
        let current_tstamp = Instant::now();
        let mut expired: BTreeMap<Instant, HashSet<&'a str>> = BTreeMap::new();
        let mut active: BTreeMap<Instant, HashSet<&'a str>> = BTreeMap::new();
        (expired, active) =
            self.ttl.iter_mut()
            .fold((expired, active),
                  |
                  (mut e, mut a),
                  (item_tstamp, hash_str)
                  | {
                      if *item_tstamp <= current_tstamp {
                          e.insert(*item_tstamp, hash_str.clone());
                      } else {
                          a.insert(*item_tstamp, hash_str.clone());
                      }
                      (e, a)
                  });

        expired
            .iter()
            .for_each(|(_, hs)| {hs.iter().for_each(|s| {self.remove(s);})});

        self.ttl = active;
    }
}

fn main() {
    let data = vec![
        (String::from("one"), Box::new(1), Duration::from_secs(5)),
        (String::from("two"), Box::new(2), Duration::from_millis(1000)),
        (String::from("two"), Box::new(2), Duration::from_millis(1000)),
        (String::from("two"), Box::new(2), Duration::from_millis(999)),
        (String::from("two"), Box::new(2), Duration::from_millis(18446744073709551615)),
        (String::from("two"), Box::new(999), Duration::from_millis(1)),
        (String::from("three"), Box::new(3), Duration::from_secs(5)),
        (String::from("four"), Box::new(4), Duration::from_secs(1)),
        (String::from("fifteen"), Box::new(15), Duration::from_secs(2)),
        (String::from("fifteen"), Box::new(15), Duration::from_secs(3)),
        (String::from("twenty"), Box::new(20), Duration::from_millis(1510))
    ];

    let mut cache = Cache::new();
    data.iter().for_each(|item| {
        cache.insert(&item.0, &item.1, item.2);
    });

    println!("GET method: {:#?}", cache.get("two"));
    println!("GET method: {:#?}", cache.get("four"));
    cache.remove("two");
    cache.remove("two");
    cache.remove("three");
    cache.remove("one");
    println!("Cache before expire: {:#?}", cache);

    std::thread::sleep(Duration::from_millis(1500));
    cache.expire();

    println!("Cache after expire: {:#?}", cache);

}
