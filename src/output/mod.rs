pub trait Output {
    fn print(&self, string: &str);
}

pub struct Stdout {}

impl Output for Stdout {
    fn print(&self, string: &str) {
        println!("{}", string);
    }
}

pub fn stdout() -> Stdout {
    return Stdout {};
}
