mod cli;

use std::cell::Cell;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::io::Write;

use regex::Regex;
use termcolor::WriteColor;
use walkdir::{self, WalkDir};

fn main() {
    let cli::Args {flags, options, pattern, path} = cli::Args::new();
    let mut found_count = 0;
    let result = Cell::new(1);

    let pattern = match Regex::new(&pattern) {
        Ok(pattern) => pattern,
        Err(error) => {
            eprintln!("Invalid pattern. Error: {}" , error);
            std::process::exit(2);
        }
    };

    let path_exist_filter = |path: &PathBuf| match path.is_dir() {
        true => true,
        false => {
            if !flags.quiet {
                eprintln!("{}: No such directory", path.display());
                result.set(2)
            }
            false

        }
    };

    let filter_errors = |path: walkdir::Result<walkdir::DirEntry>| match path {
        Ok(entry) => {
            let entry_type = entry.file_type();
            match (entry_type.is_file() && flags.file) || (entry_type.is_dir() && flags.dir) {
                true => Some(entry),
                false => None,
            }
        },
        Err(error) => {
            if !flags.quiet {
                eprintln!("ERROR: {}", error);
                result.set(2)
            }
            None
        }
    };

    {
        let stdout = termcolor::StandardStream::stdout(termcolor::ColorChoice::Auto);
        let mut stdout = stdout.lock();

        for path in path.into_iter().filter(path_exist_filter) {
            println!("Path: {}", path.display());

            let walker = WalkDir::new(path).min_depth(options.min_hop)
                                           .max_depth(options.max_hop.unwrap_or(std::usize::MAX))
                                           .follow_links(flags.sym)
                                           .into_iter()
                                           .filter_map(filter_errors);

            for entry in walker {
                match entry.file_name().to_str().and_then(|file_name| pattern.find(file_name).map(|res| (file_name, res))) {
                    Some((file_name, mat)) => {
                        found_count += 1;

                        match entry.path().parent() {
                            Some(dir) => {
                                let _ = write!(&mut stdout, "{}{}", dir.display(), MAIN_SEPARATOR);
                            },
                            None => (),
                        }

                        let _ = write!(&mut stdout, "{}", &file_name[..mat.start()]);

                        let _ = stdout.set_color(termcolor::ColorSpec::new().set_fg(Some(termcolor::Color::Red)));
                        let _ = write!(&mut stdout, "{}", mat.as_str());
                        let _ = stdout.reset();

                        let _ = writeln!(&mut stdout, "{}", &file_name[mat.end()..]);
                    },
                    None => ()
                }
            }
        }
    }

    if result.get() != 2  && found_count > 0 {
        result.set(0)
    }

    std::process::exit(result.get());
}
