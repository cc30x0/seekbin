use std::{env, thread, sync::{Mutex, Arc}, fs::File, io::Write};

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => panic!("UndefinedError occured"),
        1 | 2 => println!("InvalidArgslenError occured"),
        _ => {
            let arg1 = match args[1].parse() {
                Ok(arg) => arg,
                Err(e) => panic!("ParseError occured\ninfo: {}", e),
            };

            let arg2 = match args[2].parse() {
                Ok(arg) => arg,
                Err(e) => panic!("ParseError occured\ninfo: {}", e),
            };

            let (max, min) = {
                if arg1 > arg2 {
                    (arg1, arg2)
                } else {
                    (arg2, arg1)
                }
            };

            seek_binary(min, max);
        }
    }
}

fn seek_binary(min: usize, max: usize) {
    println!("range: min {}b ~ max {}b", min, max);

    let choose_size = |min, max| rand::thread_rng().gen_range(min..(max + 1));

    let mut handles = Vec::new();

    for i in 0..10 {
        handles.push(thread::spawn(move || {
            let size = choose_size(min, max) / 4;
            println!("{}, choose size: {}b", i, size * 4);

            let total = Arc::new(Mutex::new(Vec::new()));

            loop {
                 let total = Arc::clone(&total);

                 let mut total = total.lock().unwrap();

                if total.len() > size {
                    break;
                }

                append_usize(&mut (*total),  rand::thread_rng().gen());
            }

            let total = total.lock().unwrap();

            let mut bin_file = File::create(format!("{}.bin", i)).unwrap();

            bin_file.write(&(*total)).unwrap();
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    };
}

fn append_usize(total: &mut Vec<u8>, num: usize) {
    let mut real_flag = false;

    for oct in num.to_be_bytes() {
        if oct != 0 && !real_flag {
            real_flag = true;
        }

        if real_flag {
            total.push(oct);
        }
    }
}
