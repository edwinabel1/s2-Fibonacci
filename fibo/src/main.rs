use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let mut _display_all_number = false;
    let mut k: Option<u32> = None;
    let mut _n: Option<u32> = None;

    args.remove(0);
    for arg in &args {
        match arg.as_str() {
            "-a" => _display_all_number = true,
            _ => {
                if let None = k {
                    k = if let Ok(i) = arg.trim().parse() {
                        Some(i)
                    } else {
                        output_info();
                        None
                    }
                } else {
                    _n = if let Ok(i) = arg.trim().parse() {
                        Some(i)
                    } else {
                        output_info();
                        None
                    }
                }
            }
        }
    }

    if let None = k.and(_n) {
        output_info()
    } else {
        println!(
            "display_all_number:{}, k:{}, n:{}",
            &_display_all_number,
            &k.unwrap(),
            &_n.unwrap()
        );

        if _display_all_number {
            for index in 0..(_n.unwrap() + 1) {
                let fibo_param = FiboParam {
                    k: k.unwrap(),
                    n: index,
                };
                print!("{} ", &fn_fibo(fibo_param));
            }
        } else {
            let fibo_param = FiboParam {
                k: k.unwrap(),
                n: _n.unwrap(),
            };
            println!("value:{}", &fn_fibo(fibo_param));
        }
    }
}

fn output_info() {
    println!("fibo [-a] k n")
}

struct FiboParam {
    k: u32,
    n: u32,
}

fn fn_fibo(fibo: FiboParam) -> u32 {
    match fibo.n.cmp(&fibo.k) {
        std::cmp::Ordering::Less => 0,
        std::cmp::Ordering::Equal => 1,
        std::cmp::Ordering::Greater => {
            if fibo.n == fibo.k {
                1
            } else if fibo.n == fibo.k + 1 {
                1
            } else {
                let mut ret = 0;
                for index in (fibo.n - fibo.k)..fibo.n {
                    ret = ret
                        + fn_fibo(FiboParam {
                            k: fibo.k,
                            n: index,
                        })
                }
                ret
            }
        }
    }
}
