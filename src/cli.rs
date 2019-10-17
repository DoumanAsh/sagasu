use std::path::PathBuf;

use arg::Args;

#[derive(Args, Debug)]
///Find files utility
pub struct Cli {
    #[arg(short, long)]
    ///Flag whether to print directories or not. By default is true, if file is not specified
    pub dir: bool,
    #[arg(short, long)]
    ///Flag whether to print executables or not. By default is true, if dir is not specified
    pub file: bool,
    #[arg(short, long)]
    ///Specifies that usage comes from another application. Disables colors.
    pub machine: bool,
    #[arg(short, long)]
    ///Follow symbolic links. By default they are not followed.
    pub sym: bool,
    #[arg(short, long)]
    ///Ignore errors during search.
    pub quiet: bool,

    #[arg(long = "minhop")]
    ///Minimum number of hops before starting to look. [default: 0]
    pub min_hop: usize,
    #[arg(long = "hop", default_value = "core::usize::MAX")]
    ///Specifies depth of recursion.
    pub max_hop: usize,
    #[arg(long = "sep", default_value = "'\\n'")]
    ///Specifies separator character between each entry. By default newline
    pub sep: char,


    #[arg(required)]
    ///Pattern to filter by. Allowed types: Regex
    pub pattern: regex::Regex,
    ///Specifies directory where to look. [default: .]
    pub path: Vec<PathBuf>,
}

impl Cli {
    pub fn new<'a, T: IntoIterator<Item = &'a str>>(args: T) -> Result<Self, i32> {
        let args = args.into_iter().skip(1);

        Cli::from_args(args).map_err(|err| match err.is_help() {
            true => {
                println!("{}", Cli::HELP);
                0
            },
            false => {
                eprintln!("{}", err);
                2
            },
        }).map(|mut args| {
            if args.path.len() == 0 {
                args.path.push(".".into());
            }

            if !args.file && !args.dir {
                args.file = true;
                args.dir = true;
            }

            args
        })
    }
}
