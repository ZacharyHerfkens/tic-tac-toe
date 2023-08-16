use std::io::Write;

pub fn prompt(msg: &str) -> String {
    print!("{msg}");
    std::io::stdout().flush().unwrap();
    let mut resp = String::new();
    std::io::stdin().read_line(&mut resp).unwrap();
    resp.trim().to_string()
}

pub fn prompt_map<T, E: std::fmt::Display>(msg: &str, f: impl Fn(&str) -> Result<T, E>) -> T {
    loop {
        let resp = prompt(msg);
        match f(&resp) {
            Ok(t) => return t,
            Err(e) => println!("{}", e),
        }
    }
}