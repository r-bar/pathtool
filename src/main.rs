use std::collections::HashSet;
use std::env;
use std::path::Path;

use clap::{self, Clap};

type IsSbin = bool;
type Depth = isize;
type PathCmp = (Depth, IsSbin, String);

fn path_cmp(path: &String) -> PathCmp {
    let p = Path::new(path);
    let depth = (p.iter().count() as isize) * -1;
    let is_sbin = path.ends_with("/sbin") || path.ends_with("/sbin/");
    (depth, is_sbin, path.to_owned())
}

#[derive(Clap, Debug)]
struct Args {
    #[clap(short, long)]
    append: Vec<String>,

    #[clap(short, long)]
    prepend: Vec<String>,

    #[clap(short = 'A', long)]
    append_all: Vec<String>,

    #[clap(short = 'P', long)]
    prepend_all: Vec<String>,

    #[clap(short = 'S', long)]
    no_sort: bool,

    #[clap(short = 'D', long)]
    no_dedup: bool,
}

impl Args {
    fn sort(&self) -> bool {
        !self.no_sort
    }
}

fn main() {
    let mut args = Args::parse();
    let path_var = env::var("PATH").unwrap_or_default();
    let mut sorted_paths = Vec::new();

    sorted_paths.append(&mut args.prepend);
    sorted_paths.extend(path_var.split(":").map(String::from));
    sorted_paths.append(&mut args.append);

    if args.sort() {
        sorted_paths.sort_by_key(path_cmp);
    }

    let mut seen_paths = HashSet::new();
    let no_dedup = args.no_dedup;
    let paths: Vec<String> = args
        .prepend_all
        .into_iter()
        .chain(sorted_paths.into_iter())
        .chain(args.append_all.into_iter())
        .filter(|p| no_dedup || seen_paths.insert(p.clone()))
        .collect();

    println!("{}", &paths.join(":"));
}
