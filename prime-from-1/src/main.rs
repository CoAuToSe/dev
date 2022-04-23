fn main() {
    println!("Hello, world!");
    let mut numbers = vec![2];
    let mut index = 2 as usize;
    loop {
        // std::thread::sleep(std::time::Duration::new(0, 1_000_000));
        index += 1;
        println!("{:?} \n {}", &numbers, index);
        if prime(&numbers, index) {
            numbers.push(index)
        }
    }
}

fn prime(known: &Vec<usize>, num: usize) -> bool {
    for e in known {
        let some = *e;
        if some * some > num {
            break;
        }
        if num % some == 0 {
            if num != some {
                return false;
            }
        }
    }
    return true;
}

fn prime_factorisation<'a>(number: usize) -> &'a Vec<(usize, usize)> {
    let temp = vec![];
    return &temp;
}
