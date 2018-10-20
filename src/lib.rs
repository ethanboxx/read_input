use std::io;

pub trait ReadInput {
    fn read_input(msg: &str, err: &str) -> Self;
}

macro_rules! impl_read_inputn {
    ($($t:ty),*) => {$(
    impl ReadInput for $t {
        fn read_input(msg: &str, err: &str) -> Self {
            println!("{}", msg);
            let mut input = String::new();
            loop {
                io::stdin()
                    .read_line(&mut input)
                    .expect("Failed to read line");
                match input.trim().parse() {
                    Ok(num) => {
                        println!("");
                        break num;
                    }
                    Err(_) => {
                        println!("{}", err);
                        continue;
                    }
                }
            }
        }
    }
    )*}
}

impl_read_inputn! { i8, u8, i16, u16,f32, i32, u32,f64, i64, u64, i128, u128 }
