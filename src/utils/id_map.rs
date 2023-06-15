use std::collections::HashMap;

struct IDProvider(usize);

impl IDProvider {
    fn new() -> Self {
        Self(0)
    }

    fn next(&mut self) -> usize {
        self.0 = self.0.wrapping_add(1);
        self.0
    }
}

pub struct IDMap<T> {
    map: HashMap<usize, T>,
    provider: IDProvider,
}

impl<T> IDMap<T> {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            provider: IDProvider::new(),
        }
    }

    pub fn insert(&mut self, t: T) -> usize {
        let mut id = self.provider.next();
        if self.map.contains_key(&id) || id == 0 {
            id = self.provider.next();
        }
        self.map.insert(id, t);
        id
    }

    pub fn remove(&mut self, id: usize) -> Option<T> {
        self.map.remove(&id)
    }

    pub fn get(&self, id: usize) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T> {
        self.map.get_mut(&id)
    }
}
