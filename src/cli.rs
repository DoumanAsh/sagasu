use std::path::PathBuf;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "sagasu", raw(setting = "structopt::clap::AppSettings::ArgRequiredElseHelp"))]
pub struct Args {
    #[structopt(flatten)]
    pub flags: Flags,
    #[structopt(flatten)]
    pub options: Options,
    ///Pattern to filter by. Allowed types: Regex
    pub pattern: String,
    #[structopt(parse(from_os_str), default_value = ".")]
    ///Specifies directory where to look.
    pub path: Vec<PathBuf>,
}

impl Args {
    #[inline]
    pub fn new() -> Self {
        let mut args = Self::from_args();

        if !args.flags.dir && !args.flags.file {
            args.flags.dir = true;
            args.flags.file = true;
        }

        args
    }
}

#[derive(Debug, Copy, Clone, StructOpt)]
///CLI options
pub struct Options {
    #[structopt(long = "minhop", default_value = "0")]
    ///Minimum number of hops before starting to look.
    pub min_hop: usize,
    #[structopt(long = "hop")]
    ///Specifies depth of recursion.
    pub max_hop: Option<usize>
}

#[derive(Debug, Copy, Clone, StructOpt)]
pub struct Flags {
    #[structopt(short = "d", long = "dir")]
    ///Flag whether to print directories or not. By default is true, if file is not specified
    pub dir: bool,
    #[structopt(short = "f", long = "file")]
    ///Flag whether to print executables or not. By default is true, if dir is not specified
    pub file: bool,
    #[structopt(short = "s", long = "symlink")]
    ///Follow symbolic links. By default they are not followed.
    pub sym: bool,
    #[structopt(short = "q", long = "quiet")]
    ///Ignore errors during search.
    pub quiet: bool
}
