use std::{
    collections::{HashMap, BTreeMap, HashSet},
    collections::hash_map::Entry as HashMapEntry,
    collections::btree_map::Entry as BTreeMapEntry,
    // mem::replace,
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

    pub fn expire_retain(&mut self) {
        let now = Instant::now();
        self.ttl.retain(|x, y| {
            if now.gt(x) {
                y.iter().for_each(|x|{self.map.remove(x);});
                false
            } else {
                true
            }
        });
    }

    pub fn expire(&mut self) {
        let now = Instant::now();
        let mut new_index : BTreeMap<Instant, HashSet<&'a str>> = Default::default();
        for (key, value) in self.ttl.clone().into_iter() {
            if key >= now {
                new_index.insert(key, value);
            } else {
                for &v in value.iter() {
                    self.map.remove(v);
                }
            }
        }
        self.ttl = new_index;
    }
}
