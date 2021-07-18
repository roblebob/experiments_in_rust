use std::collections::HashMap;
use std::hash::Hash;

pub enum Status<T> {
    Cached(T),
    Computed(T),
}

impl<T> Status<T>
{
    pub fn unwrap(self) -> T {
        match self {
            Status::Cached(x) => x,
            Status::Computed(x) => x,
        }
    }
}

pub struct Cacher<T,F>
where
    T: Eq + Hash + Copy,
    F: Fn(T) -> T
{
    cache_map: HashMap<T,T>,
    calculation: F,
}

impl<T,F> Cacher<T,F>
where
    T: Eq + Hash + Copy,
    F: Fn(T) -> T
{
    pub fn new(calculation: F) -> Cacher<T,F> {
        Cacher {
            cache_map: HashMap::new(),
            calculation,
        }
    }

    pub fn value(&mut self, arg: T) -> T {
        self.value_with_status(arg).unwrap()
    }

    pub fn value_with_status(&mut self, arg: T) -> Status<T> {

        if self.cache_map.contains_key(&arg) {
            Status::Cached(self.cache_map[&arg])
        } else {
            let v = (self.calculation)(arg);
            self.cache_map.insert(arg, v);
            Status::Computed(v)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a*10);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 20);
    }

    #[test]
    fn call_with_different_values_and_status() {
        let mut c = Cacher::new(|a| a*10);

        let v1 = c.value(-1);
        
        let v2 = c.value_with_status(-2);
        match v2 {
            Status::Cached(_) => panic!("Should not have happened! Was called the first time!"),
            Status::Computed(x) => assert_eq!(x, -20),
        }

        let v2 = c.value_with_status(-2);
        match v2 {
            Status::Cached(x) => assert_eq!(x, -20),
            Status::Computed(_) => panic!("Should not have happend! Proofs cache's memory is less than 1!"),
        }

        let v1 = c.value_with_status(-1);
        match v1 {
            Status::Cached(x) => assert_eq!(x, -10),
            Status::Computed(_) => panic!("TDD failing test for introducing cache memory larger than 1!"),
        }
    }
}
