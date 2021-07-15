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



pub struct Cacher<F>
where
    F: Fn(u32) -> u32
{
    argument: Option<u32>,
    calculation: F,
    value: Option<u32>,
}

impl<F> Cacher<F>
where
    F: Fn(u32) -> u32
{
    pub fn new(calculation: F) -> Cacher<F> {
        Cacher {
            argument: None,
            calculation,
            value: None,
        }
    }

    pub fn value(&mut self, arg: u32) -> u32 {
        self.value_with_status(arg).unwrap()
    }

    pub fn value_with_status(&mut self, arg: u32) -> Status<u32> {

        match self.argument {
            Some(x) => {
                if x==arg {Status::Cached(self.value.unwrap())} 
                else {self.helper(arg)}
              },
            None => self.helper(arg),
        }
    }

    fn helper(&mut self, arg: u32) -> Status<u32> {
        self.argument = Some(arg);        
        let v = (self.calculation)(arg);
        self.value = Some(v);
        Status::Computed(v)
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

        let v1 = c.value(1);


        let v2 = c.value_with_status(2);
        match v2 {
            Status::Cached(_) => panic!("Should not have happened! Was called the first time!"),
            Status::Computed(x) => assert_eq!(x, 20),
        }

        let v2 = c.value_with_status(2);
        match v2 {
            Status::Cached(x) => assert_eq!(x, 20),
            Status::Computed(_) => panic!("Should not have happend! Proofs cache's memory is less than 1!"),
        }

        let v1 = c.value_with_status(1);
        match v1 {
            Status::Cached(x) => assert_eq!(x, 10),
            Status::Computed(_) => panic!("TDD failing test for introducing cache memory larger than 1!"),
        }
    }
}
