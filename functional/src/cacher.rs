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

pub struct Cacher<X,F,Y>
where
    X: Debug + Clone,
    F: Fn(X) -> Y,
    Y: Clone,
{
    cache_map: HashMap<String,(X,Y)>,
    calculation: F,
}

impl<X,F,Y> Cacher<X,F,Y>
where
    X: Debug + Clone,
    F: Fn(X) -> Y,
    Y: Clone,
{
    pub fn new(calculation: F) -> Cacher<X,F,Y> {
        Cacher {
            cache_map: HashMap::new(),
            calculation,
        }
    }

    pub fn value(&mut self, arg: &X) -> Y {
        self.value_with_status(arg).unwrap()
    }

    pub fn value_with_status(&mut self, arg: &X) -> Status<Y> {

        let key: String = format!("{:?}", *arg);

        if self.cache_map.contains_key(&key) {
            Status::Cached(self.cache_map[&key].1.clone())
        } else {
            let a = (*arg).clone();
            let v = (self.calculation)(a);
            self.cache_map.insert(key, ((*arg).clone(), v.clone()));
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

        let v1 = c.value(&1);
        let v2 = c.value(&2);

        assert_eq!(v2, 20);
    }

    #[test]
    fn call_with_different_values_and_status() {
        let mut c = Cacher::new(|a| a*10);

        let mut  ar = -1;

        let v1 = c.value(&ar);
        
        ar = -2;
        let v2 = c.value_with_status(&ar);
        match v2 {
            Status::Cached(_) => panic!("Should not have happened! Was called the first time!"),
            Status::Computed(x) => assert_eq!(x, -20),
        }
        let v2 = c.value_with_status(&ar);
        match v2 {
            Status::Cached(x) => assert_eq!(x, -20),
            Status::Computed(_) => panic!("Should not have happend! Proofs cache's memory is less than 1!"),
        }

        ar = -1;
        let v1 = c.value_with_status(&ar);
        match v1 {
            Status::Cached(x) => assert_eq!(x, -10),
            Status::Computed(_) => panic!("TDD failing test for introducing cache memory larger than 1!"),
        }
    }

    #[test]
    fn call_with_floats() {
        let mut c = Cacher::new(|a| a);
        let v = c.value(&1.1);
        assert_eq!(v, 1.1);
    }

    #[test]
    fn call_with_strings() {
        let mut c = Cacher::new(|a: String| a.clone());
        let ar = "Hallo".to_string();
        let v = c.value(&ar);
        assert_eq!(v, ar);
    }

    #[test]
    fn call_length_of_a_string_slice() {
        let mut c = Cacher::new(|a: &str| a.len());
        let ar = "hello";
        let v = c.value(&ar); 
        assert_eq!(v, 5); 
    }
}
