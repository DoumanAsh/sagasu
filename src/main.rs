mod cli;

use std::cell::Cell;
use std::path::{MAIN_SEPARATOR, PathBuf};
use std::io::Write;

use termcolor::WriteColor;
use walkdir::{self, WalkDir};

fn main() {
    let args = match cli::Args::new() {
        Ok(args) => args,
        Err(code) => std::process::exit(code),
    };

    let is_dir = args.dir;
    let is_file = args.file;
    let is_quiet = args.quiet;
    let path = args.path;
    let pattern = args.pattern;

    let mut found_count = 0;
    let result = Cell::new(1);

    let path_exist_filter = |path: &PathBuf| match path.is_dir() {
        true => true,
        false => {
            if !is_quiet {
                eprintln!("{}: No such directory", path.display());
                result.set(3)
            }
            false

        }
    };

    let filter_errors = |path: walkdir::Result<walkdir::DirEntry>| match path {
        Ok(entry) => {
            let entry_type = entry.file_type();
            match (entry_type.is_file() && is_file) || (entry_type.is_dir() && is_dir) {
                true => Some(entry),
                false => None,
            }
        },
        Err(error) => {
            if !is_quiet {
                eprintln!("ERROR: {}", error);
                result.set(3)
            }
            None
        }
    };

    {
        let color = match args.machine {
            true => termcolor::ColorChoice::Never,
            false => termcolor::ColorChoice::Auto,
        };
        let stdout = termcolor::StandardStream::stdout(color);
        let mut stdout = stdout.lock();

        for path in path.into_iter().filter(path_exist_filter) {
            let walker = WalkDir::new(path).min_depth(args.min_hop)
                                           .max_depth(args.max_hop)
                                           .follow_links(args.sym)
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

                        let _ = write!(&mut stdout, "{}{}", &file_name[mat.end()..], args.sep);
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
