pub mod async_fs;
use ignore::{WalkBuilder, WalkState};
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

#[doc = "Currently only `md`, `txt` and `plain` files are supported"]
pub fn is_supported_file(path: &PathBuf) -> bool {
    let ext = path.extension().and_then(OsStr::to_str);
    path.is_file() && (ext.is_none() || ext == Some("md") || ext == Some("txt"))
}

pub fn try_read(path: &Path, create: bool) -> Result<String, Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(create)
        .open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn try_write(path: &Path, data: &[u8], append: bool) -> Result<(), Error> {
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .append(append)
        .create(true)
        .truncate(!append)
        .open(path)?;

    file.write_all(data)?;
    file.write("\n".as_bytes())?;
    file.flush()?;
    Ok(())
}

#[allow(dead_code)]
pub fn try_rename<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> Result<(), Error> {
    fs::rename(from, to)
}

#[allow(dead_code)]
pub fn try_unlink(path: &PathBuf) -> Result<(), Error> {
    fs::remove_file(path)
}

#[allow(dead_code)]
pub fn get_dir_files(dir: &Path) -> Result<Vec<PathBuf>, Error> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if is_supported_file(&path) {
                files.push(entry.path())
            }
        }
    }
    Ok(files)
}

#[allow(dead_code)]
pub async fn create_dir_all(path: &Path) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        fs::create_dir_all(dir)?
    }
    Ok(())
}

pub fn search_content(path: &Path, query: &str) -> Result<Vec<PathBuf>, ()> {
    let mut found: Vec<PathBuf> = vec![];
    if query.is_empty() {
        return Ok(found);
    }
    let (tx, rx) = mpsc::channel();
    let walker = WalkBuilder::new(path)
        .follow_links(false)
        .filter_entry(|f| f.metadata().unwrap().is_file() && 
        is_supported_file(&f.path().to_path_buf()))
        .same_file_system(true)
        .max_depth(Some(1))
        .build_parallel();

    walker.run(move || {
        let cloned_tx = tx.clone();
        Box::new(move |entry| {
            let Ok(entry) = entry else {
                return  WalkState::Skip;
            };
            let res = search_in_file(entry.path(), query).unwrap_or(vec![]);
            if cloned_tx.send(res).is_err() {
                return WalkState::Quit;
            }
            WalkState::Continue
        })
    });

    for mut received in rx {
        found.append(&mut received);
    };
    Ok(found)
}

#[doc = "Returns atmost 2 lines and filename where the query is found"]
fn search_in_file(path: &Path, query: &str) -> Result<Vec<PathBuf>, Error> {
    let mut found: Vec<PathBuf> = vec![];
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let max_lines_to_read = 2;
    for (_, line) in reader.lines().enumerate() {
        let line = line?;
        if let Some((_start, _end)) = match_in_line(&line, query) {
            // let snippet = &line[start..end];
            found.push(path.to_path_buf());
        }
        if found.len() >= max_lines_to_read {
            break;
        }
    };
    Ok(found)
}

fn match_in_line(line: &str, query: &str) -> Option<(usize, usize)> {
    // let snippet_len = 0;
    let start  = line.to_lowercase().find(&query.to_lowercase())?;
    // let start = if query_pos > snippet_len {
    //     query_pos - snippet_len
    // } else {
    //     0
    // };
    // let end = if query_pos + query.len() + snippet_len < line.len() {
    //     query_pos + query.len() + snippet_len
    // } else {
    //     line.len()
    // };

    Some((start, query.len()))
}