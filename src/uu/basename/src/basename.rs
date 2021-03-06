// This file is part of the uutils coreutils package.
//
// (c) Jimmy Lu <jimmy.lu.2011@gmail.com>
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

// spell-checker:ignore (ToDO) fullname

#[macro_use]
extern crate uucore;

use std::path::{is_separator, PathBuf};

static NAME: &str = "basename";
static SYNTAX: &str = "NAME [SUFFIX]";
static SUMMARY: &str = "Print NAME with any leading directory components removed
 If specified, also remove a trailing SUFFIX";
static LONG_HELP: &str = "";

pub fn uumain(args: impl uucore::Args) -> i32 {
    let args = args.collect_str();

    //
    // Argument parsing
    //
    let matches = app!(SYNTAX, SUMMARY, LONG_HELP)
        .optflag(
            "a",
            "multiple",
            "Support more than one argument. Treat every argument as a name.",
        )
        .optopt(
            "s",
            "suffix",
            "Remove a trailing suffix. This option implies the -a option.",
            "SUFFIX",
        )
        .optflag(
            "z",
            "zero",
            "Output a zero byte (ASCII NUL) at the end of each line, rather than a newline.",
        )
        .parse(args);

    // too few arguments
    if matches.free.is_empty() {
        crash!(
            1,
            "{0}: {1}\nTry '{0} --help' for more information.",
            NAME,
            "missing operand"
        );
    }
    let opt_s = matches.opt_present("s");
    let opt_a = matches.opt_present("a");
    let opt_z = matches.opt_present("z");
    let multiple_paths = opt_s || opt_a;
    // too many arguments
    if !multiple_paths && matches.free.len() > 2 {
        crash!(
            1,
            "{0}: extra operand '{1}'\nTry '{0} --help' for more information.",
            NAME,
            matches.free[2]
        );
    }

    let suffix = if opt_s {
        matches.opt_str("s").unwrap()
    } else if !opt_a && matches.free.len() > 1 {
        matches.free[1].clone()
    } else {
        "".to_owned()
    };

    //
    // Main Program Processing
    //

    let paths = if multiple_paths {
        &matches.free[..]
    } else {
        &matches.free[0..1]
    };

    let line_ending = if opt_z { "\0" } else { "\n" };
    for path in paths {
        print!("{}{}", basename(&path, &suffix), line_ending);
    }

    0
}

fn basename(fullname: &str, suffix: &str) -> String {
    // Remove all platform-specific path separators from the end
    let mut path: String = fullname
        .chars()
        .rev()
        .skip_while(|&ch| is_separator(ch))
        .collect();

    // Undo reverse
    path = path.chars().rev().collect();

    // Convert to path buffer and get last path component
    let pb = PathBuf::from(path);
    match pb.components().last() {
        Some(c) => strip_suffix(c.as_os_str().to_str().unwrap(), suffix),
        None => "".to_owned(),
    }
}

#[allow(clippy::manual_strip)] // can be replaced with strip_suffix once the minimum rust version is 1.45
fn strip_suffix(name: &str, suffix: &str) -> String {
    if name == suffix {
        return name.to_owned();
    }

    if name.ends_with(suffix) {
        return name[..name.len() - suffix.len()].to_owned();
    }

    name.to_owned()
}
