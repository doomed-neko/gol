use std::{
    env::{self, Args},
    error::Error,
    process::exit,
    str::FromStr,
};

macro_rules! parse_arg {
    ($field:expr,$short_cmd:literal, $long_cmd:literal, $arg_var:expr,$args_var:expr, $pn_var:expr) => {
        if $arg_var == $short_cmd || $arg_var == $long_cmd {
            $field = Self::parse_next_arg(&mut $args_var, $pn_var);
        }
    };
}
pub struct GameArgs {
    pub no_gui: usize,
    pub window_width: u32,
    pub window_height: u32,
    pub tile_size: u32,
    pub fill_chance: f64,
}
impl GameArgs {
    pub fn cell_count(&self) -> u32 {
        (self.window_height * self.window_width) / self.tile_size
    }
    pub fn rows(&self) -> usize {
        (self.window_width / self.tile_size) as usize
    }
    pub fn cols(&self) -> usize {
        (self.window_height / self.tile_size) as usize
    }
}

impl Default for GameArgs {
    fn default() -> Self {
        Self {
            no_gui: 0,
            window_width: 1440,
            window_height: 960,
            tile_size: 10,
            fill_chance: 0.3,
        }
    }
}

impl GameArgs {
    pub fn print_help(program_name: &str) {
        println!("Usage: {program_name} [OPTION]");
        println!("Simulate Conway's Game Of Life.");
        println!();
        println!("  -?\t--help\t\t\tPrint helo");
        println!("  -n\t--nogui\t\t\tRun N generations without gui and exit");
        println!("  -f\t--fill-chance\t\tSet the random fill chance [0,1]");
        println!("  -w\t--window-width\t\tSet the window width");
        println!("  -h\t--window-height\t\tSet the window width");
        println!("  -s\t--tile-size\t\tSet the tile size");
    }

    fn print_error_and_exit(message: &str, err: impl Error, program_name: &str) -> ! {
        eprintln!("ERROR: {message}: {err}");
        Self::print_help(program_name);
        exit(1);
    }

    pub fn new() -> Self {
        let mut default = Self {
            ..Default::default()
        };
        let mut args = env::args();
        let program_name = args.next().unwrap_or("gol".to_string());
        while let Some(arg) = args.next() {
            if arg == "-?" || arg == "--help" {
                Self::print_help(&program_name);
                exit(0);
            }
            parse_arg!(default.no_gui, "-n", "--nogui", arg, args, &program_name);
            parse_arg!(
                default.fill_chance,
                "-f",
                "--fill-chance",
                arg,
                args,
                &program_name
            );
            parse_arg!(
                default.window_width,
                "-w",
                "--window-width",
                arg,
                args,
                &program_name
            );
            parse_arg!(
                default.window_height,
                "-h",
                "--window-height",
                arg,
                args,
                &program_name
            );
            parse_arg!(
                default.tile_size,
                "-s",
                "--tile-size",
                arg,
                args,
                &program_name
            );
        }
        default
    }
    fn parse_next_arg<T>(args: &mut Args, program_name: &str) -> T
    where
        T: FromStr,
        <T as std::str::FromStr>::Err: std::error::Error,
    {
        let Some(next_value) = args.next().map(|x| {
            x.parse::<T>().unwrap_or_else(|err| {
                Self::print_error_and_exit("faield to parse value", err, program_name)
            })
        }) else {
            eprintln!("Error: Bad usage");
            Self::print_help(program_name);
            exit(1);
        };
        next_value
    }
}
