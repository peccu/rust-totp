use std::env;
use std::time::SystemTime;
use otp::make_totp;

fn main() {
    let args: Vec<String> = env::args().collect();
    let secret = &args[1];
    print!("{}", make_totp(secret, 30, 0).unwrap());
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => eprintln!("valid in {} seconds.", 30 - n.as_secs() % 30),
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}
