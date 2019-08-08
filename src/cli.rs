use std::path::PathBuf;

const USAGE: &str = "Find files utility

USAGE:
    sagasu.exe [OPTIONS] <pattern> [path]...

OPTIONS:
    -d, --dir                 Flag whether to print directories or not. By default is true, if file is not specified
    -f, --file                Flag whether to print executables or not. By default is true, if dir is not specified
    -h, --help                Prints help information
    -m, --machine             Specifies that usage comes from another application. Disables colors.
    -q, --quiet               Ignore errors during search.
    -s, --symlink             Follow symbolic links. By default they are not followed.
    -V, --version             Prints version information
        --hop <max_hop>       Specifies depth of recursion.
        --minhop <min_hop>    Minimum number of hops before starting to look. [default: 0]
        --sep <sep>           Specifies separator character between each entry. By default newline

ARGS:
    <pattern>    Pattern to filter by. Allowed types: Regex
    <path>...    Specifies directory where to look. [default: .]
";

#[derive(Debug)]
pub struct Args {
    pub min_hop: usize,
    pub max_hop: usize,
    pub sep: char,

    pub dir: bool,
    pub file: bool,
    pub sym: bool,
    pub quiet: bool,
    pub machine: bool,

    ///Pattern to filter by. Allowed types: Regex
    pub pattern: regex::Regex,
    ///Specifies directory where to look.
    pub path: Vec<PathBuf>,
}

impl Args {
    #[inline]
    pub fn new() -> Result<Self, i32> {
        let mut min_hop = 0;
        let mut max_hop = std::usize::MAX;
        let mut sep = '\n';

        let mut dir = false;
        let mut file = false;
        let mut sym = false;
        let mut quiet = false;
        let mut machine = false;

        let mut pattern = None;
        let mut path = Vec::new();

        let mut args = std::env::args().skip(1);

        while let Some(arg) = args.next() {
            if arg.starts_with('-') {
                match &arg[1..] {
                    "d" | "-dir" => dir = true,
                    "f" | "-file" => file = true,
                    "h" | "-help" => {
                        println!("{}", USAGE);
                        return Err(0);
                    },
                    "m" | "-machine" => machine = true,
                    "q" | "-quiet" => quiet = true,
                    "s" | "-symlink" => sym = true,
                    "-hop" => match args.next() {
                        Some(new_hop) => match new_hop.parse() {
                            Ok(new_hop) => max_hop = new_hop,
                            Err(_) => {
                                eprintln!("Flag {} is specified with invalid value '{}'. Should be integer", arg, new_hop);
                                return Err(2);
                            }
                        },
                        None => {
                            eprintln!("Flag {} is specified, but argument is missing", arg);
                            return Err(2);
                        }
                    },
                    "-minhop" => match args.next() {
                        Some(new_hop) => match new_hop.parse() {
                            Ok(new_hop) => min_hop = new_hop,
                            Err(_) => {
                                eprintln!("Flag {} is specified with invalid value '{}'. Should be integer", arg, new_hop);
                                return Err(2);
                            }
                        },
                        None => {
                            eprintln!("Flag {} is specified, but argument is missing", arg);
                            return Err(2);
                        }
                    },
                    "-sep" => match args.next() {
                        Some(new_sep) => match new_sep.parse() {
                            Ok(new_sep) => sep = new_sep,
                            Err(_) => {
                                eprintln!("Flag {} is specified with invalid value '{}'. Should be single character", arg, new_sep);
                                return Err(2);
                            }
                        },
                        None => {
                            eprintln!("Flag {} is specified, but argument is missing", arg);
                            return Err(2);
                        }
                    },
                    _ => {
                        eprintln!("Invalid flag '{}' is specified", arg);
                        println!("{}", USAGE);
                        return Err(2);
                    }
                }
            } else if pattern.is_none() {
                match regex::Regex::new(&arg) {
                    Ok(new) => pattern = Some(new),
                    Err(error) => {
                        eprintln!("Invalid pattern. Error: {}" , error);
                        return Err(2);
                    }
                }
            } else {
                path.push(arg.into());
            }
        }

        let pattern = match pattern {
            Some(pattern) => pattern,
            None => {
                eprintln!("Missing pattern.");
                println!("{}", USAGE);
                return Err(2);
            }
        };

        if path.len() == 0 {
            path.push(".".into());
        }

        if !file && !dir {
            file = true;
            dir = true;
        }

        Ok(Self {
            min_hop,
            max_hop,
            sep,
            dir,
            file,
            sym,
            quiet,
            machine,
            pattern,
            path,

        })
    }
}
