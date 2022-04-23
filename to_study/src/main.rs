// #![allow(unused)]
fn main() {
    use std::sync::RwLock;

    let lock = RwLock::new(1);
    let lock2 = RwLock::new(1);
    {
        let n = lock.read().unwrap();
        println!("n: {:?}", n);
        // let lock2 = RwLock::new(1);
        
        {
            let n2 = lock2.read().unwrap();
            println!("n2: {:?} {}", n2, n2);
            assert_eq!(*n2, 1);
        }
        {
            println!("m2: {:?}", lock2.try_write().unwrap());
            println!("m2: {:?}", lock2.try_write().unwrap());
            let mut m2 = lock2.try_write().unwrap();
            println!("m2: {:?}", m2);
            *m2 += 1;
            println!("m2: {:?}", m2);
        }
        {
            let n2 = lock2.read().unwrap();
            println!("n2: {:?}", n2);
            // *n += 1;
            // println!("n: {:?}", n);
            assert_eq!(*n2, 2);
        }
        assert_eq!(*n, 1);
    }
    {
        println!("m: {:?}", lock.try_write().unwrap());
        println!("m: {:?}", lock.try_write().unwrap());
        let mut m = lock.try_write().unwrap();
        {
            let n2 = lock2.read().unwrap();
            println!("n2: {:?}", n2);
            // assert_eq!(*n2, 1);
        }
        {
            println!("m2: {:?}", lock2.try_write().unwrap());
            println!("m2: {:?}", lock2.try_write().unwrap());
            let mut m2 = lock2.try_write().unwrap();
            println!("m2: {:?}", m2);
            *m2 += 1;
            println!("m2: {:?}", m2);
        }
        {
            let n2 = lock2.read().unwrap();
            println!("n2: {:?}", n2);
            // *n += 1;
            // println!("n: {:?}", n);
            // assert_eq!(*n2, 2);
        }
        println!("m: {:?}", m);
        *m += 1;
        println!("m: {:?}", m);
    }
    {
        let n = lock.read().unwrap();
        println!("n: {:?}", n);
        // *n += 1;
        // println!("n: {:?}", n);
        assert_eq!(*n, 2);
    }
    //assert!(lock.try_write().is_err());
}