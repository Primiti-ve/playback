#[derive(Copy, Clone, Debug)]
pub struct Config {
    pub verbose: u8,
}

impl Default for Config {
    fn default() -> Config {
        Config { verbose: 0 }
    }
}
