// Copyright 2023-2024 Joshua D. Rose
// 
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
// 
//     `<http://www.apache.org/licenses/LICENSE-2.0>`
// 
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Quick and easy command tooling in Rust 🦀
//!
//! Provides an opionated set of commands for Linux
//! A library that focuses on:
//!
//! - Simplicity
//! - Speed
//! - DX
//!
//! The commands contained herein are developed with the goal
//! to make them as close to the real thing as possible, with a few
//! tweaks here and there for developer experience.

use std::os::linux::fs::MetadataExt;
use std::fs::{self};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::{env, io};
use std::process::Command;
use std::{str, vec};

/// Change the current working directory
///
/// # Example
///
/// ```rust
/// # use termease::{cd,ls};
/// let some_folder = "/tmp";
/// cd(some_folder);
/// ```
///
/// # Panics
///
/// The returned path will panic if the path is a directory,
/// invalid, or has some sort of other issue.
pub fn cd(directory: &str) -> Result<(), io::Error> {
    let path = Path::new(directory);

    if !path.exists() {
        panic!("directory does not exist");
    } else if !path.is_dir() {
        panic!("path is not a directory");
    };

    // chdir
    env::set_current_dir(path)
}

/// Get the current working directory
///
/// Returns an err if the current working directory is invalid
pub fn cwd() -> String {
    let path = std::env::current_dir().unwrap();
    format!("{}", path.display())
}

/// Make a directory in the current folder
///
/// # Examples
/// ```rust
/// # use std::fs;
/// # use std::path::Path;
/// # use termease::{ls,mkdir};
/// # // remove previous file if exists
/// # let dir = Path::new("/tmp/folder");
/// # if dir.exists() {
/// #   fs::remove_dir(dir).unwrap();
/// # }
/// #
/// #
/// mkdir("/tmp/some_random_dir").unwrap();
/// # fs::remove_dir("/tmp/some_random_dir").unwrap();
/// ```
///
/// However, attempting to remake an already existing folder will err
/// ```rust,should_panic
/// # use termease::mkdir;
/// # use std::fs;
/// mkdir("folder").unwrap();
/// mkdir("folder").unwrap();
/// # fs::remove_dir("folder").unwrap();
/// ```
///
/// # Panics
///
/// If the folder already exists, the object will panic, as well as if the
/// path prefix is invalid.
pub fn mkdir(directory: &str) -> Result<(), io::Error> {
    let path = Path::new(directory);

    if path.exists() {
        panic!("directory already exists");
    };

    fs::create_dir(directory)?;
    Ok(())
}


/// Iterate through all the files in a given directory
///
/// # Examples
///
/// Indexing user directories will work normally:
/// ```rust
/// # use termease::ls;
/// for item in ls("/tmp").unwrap().iter() {
///     println!("{}", item.display());
/// }
///
/// ```
/// The contents of privileged directories cannot be indexed:
/// ```rust,should_panic
/// # use termease::ls;
/// ls("/root").unwrap();
/// ```
///
/// # Panics
///
/// The returned path will panic if it is a directory or
/// if the path does not exist in the file system.
///
pub fn ls(directory: &str) -> Result<Vec<PathBuf>, &'static str> {
    let path: &Path = {
        let this = Path::new(directory);

        if !this.exists() {
            return Err("directory does not exist");
        } else if !this.is_dir() {
            return Err("path is not a directory");
        }

        this
    };

    let contents = {
        let mut this = Vec::new();
        let files = fs::read_dir(path).unwrap();

        for item in files {
            let file = item.unwrap();
            this.push(file.path())
        }
        this
    };

    Ok(contents)
}


/// Report a snapshot of the current process.
///
/// Emulates the linux 'ps' command.
pub fn ps() {
    todo!()
}

/// Emulate a linux application
///
/// Emulates the linux 'execute command'
///
/// # Example
///
/// ```rust,no_run
/// # use termease::execute;
/// // a simple command
/// execute("/usr/bin/test".to_string(), Some(vec!["1", "-a", "1"]))
/// ```
pub fn execute(path: String, args: Option<Vec<&str>>) {
    let location = Path::new(&path);
    let name = location.file_name().expect("No final component of file name").to_str().unwrap();

    if !location.exists() {
        // TODO: make custom error
        panic!("Location doesn't exist")
    }


    // Are there any args?
    if args.is_none() {
        // run the program
        Command::new(name)
            .spawn()
            .expect("command failed to start");
    } else {
        // new args that are also strings
        //
        // NOTE: there is probably a better way to do this
        let _args = args.unwrap();
        let mut params = Vec::new();
        for arg in _args {
            let s = String::from(arg);
            params.push(s);
        };

        // there are args. use them.
        Command::new(name)
            .args(params)
            .spawn()
            .expect("command failed to start");
    }

}

/// Return the current system time
///
/// Used in the commands:
/// * w
fn system_time() -> SystemTime {
    let now = SystemTime::now();
    now
}

/// Return the system uptime
///
/// Used in the commands:
/// * w
fn system_uptime() -> SystemTime {
    todo!()
}

/// Remove a directory in the current folder
///
/// # Examples
/// ```rust
/// # use std::fs;
/// # use std::path::Path;
/// # use termease::{ls,rmdir};
/// # if !Path::new("folder/").exists() {
/// #     fs::create_dir("folder").unwrap();
/// # }
/// // assuming the folder exists
/// rmdir("folder/").unwrap();
/// ```
///
/// # Panics
///
/// If the folder doesnt exist, the object will panic, as well as if the
/// path prefix is invalid.
pub fn rmdir(directory: &str) -> Result<(), &str> {
    let path = Path::new(directory);

    if path.exists() {
        fs::remove_dir(directory).unwrap();
        Ok(())
    } else {
        Err("directory does not exist")
    }
}

/// A stat table for the 'stat' command.
///
/// Holds the following values:
///
/// * block size/length
/// * block number
/// * block count
/// * permissions
/// * UID and GID
struct StatTable {
    size: u64,
    number: u64,
    count: u64,
    uid: u32,
    gid: u32
}

impl Default for StatTable {
    fn default() -> StatTable {
        StatTable {
            size: 0,
            number: 0,
            count: 0,
            uid: 0,
            gid: 0
        }
    }
}

/// Emulates the linux 'stat' command.
///
/// Stats the current directory by default, otherwise stat
/// the specified directory.
///
/// # Examples
///
/// ```rust
/// # use termease::stat;
/// stat(".");
/// // prints out directory information
/// ```
///
/// ```rust,compile_fail
/// stat("/non/existant/location");
/// ```
/// # Panics
///
/// The returned path will panic if you refer to an invalid path.
///
pub fn stat(folder: &'static str) {
    let dir = Path::new(folder);
    let meta = dir.metadata().expect("Could not get metadata");

    // TODO: implement stat table
    let mut stat_table = StatTable::default();

    stat_table.size = meta.st_blksize();
    stat_table.number = meta.st_blocks();
    stat_table.count = meta.st_size();
    stat_table.uid = meta.st_uid();
    stat_table.gid = meta.st_gid();
    // FIXME: this needs to be filled
    // stat_table.permissions = meta.permissions();

    println!("  File: {}", folder);
    println!(
        "  Size: {}\tBlocks: {}\tIO Block: {}   directory",
        stat_table.size,
        stat_table.number,
        0,
    );
    println!(
        "  Device: {}\tInode: {}\tLinks: 0",
        stat_table.count,
        stat_table.gid,

        // meta.ino(),  XXX: This is a nightly only thing
    );
    println!(
        "  Access: {:?}\tUid: {}\tGid: {}",
        meta.permissions(),
        stat_table.gid,
        stat_table.uid
    );
}

/// Emulates the linux 'w' command.
///
/// Shows who is logged on and what they are doing.
///
/// Shows in this order:
///  * localtime
///  * uptime
///  * active users
///  * load average for the past, 1, 5, and 15 minutes
pub fn w() {
    let _local_time = system_time();
    let _system_uptime = system_uptime();
    // TODO:
    let _active_users = 0;
    // TODO:
    let _load_average: f32 = 50.0;

    print!(" {:?} up", _system_uptime);
    print!(" {:?},\t", _local_time);
    print!("{:?} user,\t", _active_users);
    print!(
        "load_average:{} {} {},\t",
        _load_average / 60.0, // ~1 minutes
        _load_average / 12.0, // ~5 minutes
        _load_average / 4.0,  // ~15 minutes
    );
}

/// Show who is logged on.
///
/// Prints out an emulated message based on the origina
/// 'who' command in linux.
///
/// # Example
/// ```rust,no_run
/// # use termease::who;
/// who();
/// // prints out message of who is online
/// ```
pub fn who() {
    todo!();
}

/// Shows the full path of shell commands
///
/// index_bin - should /bin be indexed? This is important because if this is true,
/// then read permissions need to be specified to the current executable so it can
/// read items in the path.
///
/// # Example
/// ```
/// // we don't need to use index_bin since I installed it as a user
/// # use termease::which;
/// let app: &str = "vim";
/// let vim_location = which(app, false);
/// ```
///
/// # Panics
///
/// If there is no path existant, then it will simply
/// raise an error, also if no perms are given since /bin is prilliged.
pub fn which(name: &str, index_bin: bool) -> Result<String, &str> {
    let mut paths = vec![Path::new("/usr/bin")];

    if index_bin {
        paths.push(Path::new("/bin"));
    }

    // collect all the files in paths
    for path in paths {
        for item in fs::read_dir(path).unwrap() {
            let item = item.expect("couldn't parse item");
            let is_dir: bool = {
                let _item = item.metadata().unwrap();
                let ft = _item.file_type();
                ft.is_dir()
            };

            if is_dir {
                continue
            };

            // NOTE: file name will always unwrap ok since is_dir is false
            let _name = item.file_name();

            if name == _name.to_str().unwrap() {
                // return the full path
                let _name_str = _name.to_str().unwrap();
                let _ = Ok::<String, &str>(String::from(_name.to_str().unwrap()));
            }
        }
    }
    Err("not found")
}

/// Print the effective user name
///
/// This command calls the local command (so it essentially acts as a bind).
/// With windows, it does the same thing as linux, would except with some minor variations.
///
/// ```no_run
/// # use termease::whoami;
/// assert_eq!(whoami(), "josh")  // NOTE: this fails in testing unless your user is //       called josh
/// ```
pub fn whoami() ->  String {
    let output = if cfg!(target_os = "windows") {
        Command::new("hostname")
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("whoami")
            .output()
            .expect("failed to execute process")
    };
    let string = String::from_utf8(output.stdout).unwrap();
    string
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_list_tmp_dir() {
        // clean up previous instances if they exist
        let _ = if Path::new("/tmp/test").exists() {
            fs::remove_dir("/tmp/test").unwrap();
        };
        let items: Vec<PathBuf> = ls("/tmp").unwrap();
        assert_ne!(items.len(), 0)
    }

    #[test]
    fn test_chdir_backwards() {
        let old: Vec<PathBuf> = ls(".").unwrap();
        let _ = cwd();
        let _ = cd("..");
        let new: Vec<PathBuf> = ls(".").unwrap();
        assert_ne!(new, old);
        // cd backwards once more
        let _ = cd(cwd().as_str());
    }

    #[test]
    fn test_chdir_forwards() {
        let _ = if Path::new("test").exists() {
            fs::remove_dir_all("test").unwrap();
        };
        let old: Vec<PathBuf> = ls(".").unwrap();
        let _ = if Path::new("test").exists() {
            let _ = cd("test");
        } else {
            let _ = mkdir("test");
            let _ = cd("test");
        };
        let new: Vec<PathBuf> = ls(".").unwrap();
        assert_ne!(new, old);
        // cd backwards
        let _ = cd("..");
        // remove newly created folder
        let _ = if Path::new("test").exists() {
            fs::remove_dir_all("test").unwrap();
        };
    }

    #[test]
    fn test_mkdir_locally() {
        let dir = "/tmp/test";
        let _ = if Path::new(dir).exists() {
            fs::remove_dir(dir).unwrap();
        };
        let _ = mkdir("/tmp/test").unwrap();
        assert!(Path::new(dir).exists());
        // clean up test
        fs::remove_dir(dir).unwrap();
    }

    #[ignore]
    #[test]
    fn test_stat_outputs_text() {
        stat(".");
        assert!(true);
    }
}
