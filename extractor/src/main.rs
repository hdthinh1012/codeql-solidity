mod extractor;
mod trap;

#[macro_use]
extern crate lazy_static;
extern crate num_cpus;

use clap::arg;
use encoding::{self};
use rayon::prelude::*;
use std::borrow::Cow;
use std::fs;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use tree_sitter::{Language, Parser, Range};

/**
 * Gets the number of threads the extractor should use, by reading the
 * CODEQL_THREADS environment variable and using it as described in the
 * extractor spec:
 *
 * "If the number is positive, it indicates the number of threads that should
 * be used. If the number is negative or zero, it should be added to the number
 * of cores available on the machine to determine how many threads to use
 * (minimum of 1). If unspecified, should be considered as set to -1."
 */
fn num_codeql_threads() -> usize {
    let threads_str = std::env::var("CODEQL_THREADS").unwrap_or_else(|_| "-1".to_owned());
    match threads_str.parse::<i32>() {
        Ok(num) if num <= 0 => {
            let reduction = -num as usize;
            std::cmp::max(1, num_cpus::get() - reduction)
        }
        Ok(num) => num as usize,

        Err(_) => {
            tracing::error!(
                "Unable to parse CODEQL_THREADS value '{}'; defaulting to 1 thread.",
                &threads_str
            );
            1
        }
    }
}

lazy_static! {
    static ref CP_NUMBER: regex::Regex = regex::Regex::new("cp([0-9]+)").unwrap();
}

fn encoding_from_name(encoding_name: &str) -> Option<&(dyn encoding::Encoding + Send + Sync)> {
    match encoding::label::encoding_from_whatwg_label(encoding_name) {
        s @ Some(_) => s,
        None => CP_NUMBER.captures(encoding_name).and_then(|cap| {
            encoding::label::encoding_from_windows_code_page(
                str::parse(cap.get(1).unwrap().as_str()).unwrap(),
            )
        }),
    }
}

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_target(false)
        .without_time()
        .with_level(true)
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ruby_extractor=warn")),
        )
        .init();
    let num_threads = num_codeql_threads();
    tracing::info!(
        "Using {} {}",
        num_threads,
        if num_threads == 1 {
            "thread"
        } else {
            "threads"
        }
    );
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_threads)
        .build_global()
        .unwrap();

    let matches = clap::App::new("Solidity extractor")
        .version("1.0")
        .author("GitHub")
        .about("CodeQL Solidity extractor")
        .arg(arg!(--"source-archive-dir" <DIR> "Sets a custom source archive folder"))
        .arg(arg!(--"output-dir" <DIR>         "Sets a custom trap folder"))
        .arg(arg!(--"file-list" <FILE_LIST>    "A text file containing the paths of the files to extract"))
        .get_matches();
    let src_archive_dir = matches
        .value_of("source-archive-dir")
        .expect("missing --source-archive-dir");
    let src_archive_dir = PathBuf::from(src_archive_dir);

    let trap_dir = matches
        .value_of("output-dir")
        .expect("missing --output-dir");
    let trap_dir = PathBuf::from(trap_dir);
    let trap_compression = trap::Compression::from_env("CODEQL_SOLIDITY_TRAP_COMPRESSION");

    let file_list = matches.value_of("file-list").expect("missing --file-list");
    let file_list = fs::File::open(file_list)?;

    let language = tree_sitter_solidity::language();
    let schema = node_types::read_node_types_str("solidity", tree_sitter_solidity::NODE_TYPES)?;
    let lines: std::io::Result<Vec<String>> = std::io::BufReader::new(file_list).lines().collect();
    let lines = lines?;
    lines
        .par_iter()
        .try_for_each(|line| {
            let path = PathBuf::from(line).canonicalize()?;
            let src_archive_file = path_for(&src_archive_dir, &path, "");
            let mut source = std::fs::read(&path)?;
            let mut needs_conversion = false;

            let mut trap_writer = trap::Writer::new();
            extractor::extract(
                language,
                "solidity",
                &schema,
                &mut trap_writer,
                &path,
                &source,
                &[],
            )?;
            std::fs::create_dir_all(&src_archive_file.parent().unwrap())?;
            if needs_conversion {
                std::fs::write(&src_archive_file, &source)?;
            } else {
                std::fs::copy(&path, &src_archive_file)?;
            }
            write_trap(&trap_dir, path, &trap_writer, trap_compression)
        })
        .expect("failed to extract files");

    let path = PathBuf::from("extras");
    let mut trap_writer = trap::Writer::new();
    extractor::populate_empty_location(&mut trap_writer);
    write_trap(&trap_dir, path, &trap_writer, trap_compression)
}

fn write_trap(
    trap_dir: &Path,
    path: PathBuf,
    trap_writer: &trap::Writer,
    trap_compression: trap::Compression,
) -> std::io::Result<()> {
    let trap_file = path_for(trap_dir, &path, trap_compression.extension());
    std::fs::create_dir_all(&trap_file.parent().unwrap())?;
    trap_writer.write_to_file(&trap_file, trap_compression)
}

fn path_for(dir: &Path, path: &Path, ext: &str) -> PathBuf {
    let mut result = PathBuf::from(dir);
    for component in path.components() {
        match component {
            std::path::Component::Prefix(prefix) => match prefix.kind() {
                std::path::Prefix::Disk(letter) | std::path::Prefix::VerbatimDisk(letter) => {
                    result.push(format!("{}_", letter as char))
                }
                std::path::Prefix::Verbatim(x) | std::path::Prefix::DeviceNS(x) => {
                    result.push(x);
                }
                std::path::Prefix::UNC(server, share)
                | std::path::Prefix::VerbatimUNC(server, share) => {
                    result.push("unc");
                    result.push(server);
                    result.push(share);
                }
            },
            std::path::Component::RootDir => {
                // skip
            }
            std::path::Component::Normal(_) => {
                result.push(component);
            }
            std::path::Component::CurDir => {
                // skip
            }
            std::path::Component::ParentDir => {
                result.pop();
            }
        }
    }
    if !ext.is_empty() {
        match result.extension() {
            Some(x) => {
                let mut new_ext = x.to_os_string();
                new_ext.push(".");
                new_ext.push(ext);
                result.set_extension(new_ext);
            }
            None => {
                result.set_extension(ext);
            }
        }
    }
    result
}

fn skip_space(content: &[u8], index: usize) -> usize {
    let mut index = index;
    while index < content.len() {
        let c = content[index] as char;
        // white space except \n
        let is_space = c == ' ' || ('\t'..='\r').contains(&c) && c != '\n';
        if !is_space {
            break;
        }
        index += 1;
    }
    index
}
