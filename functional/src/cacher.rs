use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;


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
    T: Debug + Copy,
    F: Fn(T) -> T
{
    cache_map: HashMap<String,T>,
    calculation: F,
}

impl<T,F> Cacher<T,F>
where
    T: Debug + Copy,
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

        let key: String = format!("{:?}", arg);

        if self.cache_map.contains_key(&key) {
            Status::Cached(self.cache_map[&key])
        } else {
            let v = (self.calculation)(arg);
            self.cache_map.insert(key, v);
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

        let mut  ar = -1;

        let v1 = c.value(ar);
        
        ar = -2;
        let v2 = c.value_with_status(ar);
        match v2 {
            Status::Cached(_) => panic!("Should not have happened! Was called the first time!"),
            Status::Computed(x) => assert_eq!(x, -20),
        }
        let v2 = c.value_with_status(ar);
        match v2 {
            Status::Cached(x) => assert_eq!(x, -20),
            Status::Computed(_) => panic!("Should not have happend! Proofs cache's memory is less than 1!"),
        }

        ar = -1;
        let v1 = c.value_with_status(ar);
        match v1 {
            Status::Cached(x) => assert_eq!(x, -10),
            Status::Computed(_) => panic!("TDD failing test for introducing cache memory larger than 1!"),
        }
    }

    #[test]
    fn call_with_floats() {
        let mut c = Cacher::new(|a| a);
        let v = c.value(1.1);
        assert_eq!(v, 1.1);
    }
}
