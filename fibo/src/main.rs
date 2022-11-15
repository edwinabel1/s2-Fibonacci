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
    }
}

fn output_info() {
    println!("fibo [-a] k n")
}
