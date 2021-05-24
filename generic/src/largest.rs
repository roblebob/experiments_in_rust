pub mod i32 {
    pub fn largest_( list: &[i32]) -> i32 {

        let mut largest = list[0];
        
        for &item in list {   if item > largest { largest = item; }   }
        
        largest
    }
}

pub mod char {
    pub fn largest( list: &[char]) -> char {

        let mut largest = list[0];
        
        for &item in list  {   if item > largest { largest = item; }   }
        
        largest
    }
}

pub mod copy {
    pub fn largest<T: PartialOrd + Copy >( list: &[T]) -> T {

        let mut largest = list[0];

        for &item in list {   if item > largest { largest = item; }   }

        largest
    }
}

pub mod clone {
    pub fn largest<T: PartialOrd + Clone >( list: &[T]) -> T {

        let mut largest = list[0].clone();

        for item in list {   if item > &largest { largest = item.clone() }   }

        largest
    }
}

pub mod reference {
    pub fn largest<T: PartialOrd> ( list: &[T]) -> &T {

        let mut largest = &list[0];

        for item in list {   if item > largest { largest = &item }   }

        largest
    }
}
