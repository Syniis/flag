struct FlagStore {
    flags: Vec<usize>,
    current_flag: usize,
}

impl FlagStore {
    pub fn new(size: usize) -> Self {
        Self {
            flags: vec![0; size],
            current_flag: 1,
        }
    }

    pub fn insert(&mut self, key: usize) {
        self.flags[key] = self.current_flag;
    }

    pub fn insert_check(&mut self, key: usize) -> bool {
        let res = !self.contains(key);
        self.insert(key);
        res
    }

    pub fn contains(&self, key: usize) -> bool {
        self.flags[key] == self.current_flag
    }

    pub fn clear(&mut self) {
        if self.current_flag == usize::MAX {
            self.flags.iter_mut().for_each(|idx| *idx = 0);
            self.current_flag = 0;
        }
        self.current_flag += 1;
    }
}

#[test]
fn insert() {
    let mut flags = FlagStore::new(10);
    let ok = flags.insert_check(0);
    assert!(ok);
    let ok = flags.insert_check(0);
    assert!(!ok);
}

#[test]
fn clear() {
    let size = 10;
    let mut flags = FlagStore::new(size);
    flags.insert(0);
    flags.insert(1);
    flags.clear();
    assert!(!flags.contains(0));
}
