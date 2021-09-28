fn main() {

    // Dereferencing a Raw Pointer
    
    {//19-3
        let mut num = 5;

        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    // Creating a Safe Abstraction over Unsafe Code 

    {//19-4
        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = r.split_at_mut(3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    {//19-5 --> 19-6
        use std::slice;

        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            assert!(mid <= len);

            // ... will not compile
            // (&mut slice[..mid], &mut slice[mid..])
            

            let ptr = slice.as_mut_ptr();
            
            unsafe {
                (
                    slice::from_raw_parts_mut(ptr, mid),
                    slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }   

        let mut v = vec![1, 2, 3, 4, 5, 6];

        let r = &mut v[..];

        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    // Using extern Functions to Call External Code
    
    {//19-8
        extern "C" {
            fn abs(input: i32) -> i32;
        }

        unsafe {
            println!("Absolute value of -3 according to C: {}", abs(-3));
        }
    }

}

