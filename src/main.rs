mod language_server;
mod transpiler;
mod util;
use std::{
    env::{self, current_dir},
    fs::{self, DirEntry},
    io::{self},
    path::{Path, PathBuf},
};

use transpiler::{run, ERPCError};

//TODO restructure for cleaner error handling
//TODO ensure all errors are collected before passing to language server for diagnostics (watch channel?)

#[tokio::main]
async fn main() {
    let mut start_dirs: Vec<PathBuf> = Vec::new();
    match add_start_directories(&current_dir().unwrap(), &mut start_dirs, 100) {
        Ok(_) => {}
        Err(_) => {
            //TODO: handle this error ls compatible
            eprintln!("Could not scan for start dirs");
        }
    }

    let args: Vec<String> = env::args().collect();

    if args.contains(&"-ls".to_string()) {
        let (sender, reciever) = async_channel::bounded::<Option<ERPCError>>(1);
        if start_dirs.len() == 0 {
            sender.send(Some(ERPCError::ConfigurationError("Could not detect any easy-rpc project. Make sure the project contains an erpc.json at its root.".to_string()))).await.unwrap();
        } else {
            for dir in start_dirs {
                let sender = sender.clone();
                tokio::spawn(async move {
                    loop {
                        match run(&dir, true).await {
                            Ok(_) => {}
                            Err(err) => {
                                sender.send(Some(err)).await.unwrap();
                            }
                        };
                    }
                });
            }
        }
        language_server::start_language_server(reciever).await;
    } else {
        if start_dirs.len() == 0 {
            eprintln!("Could not detect any easy-rpc project. Make sure the project contains an erpc.json at its root.");
        }
        if args.contains(&"-w".to_string()) {
            let mut handles = vec![];
            for dir in start_dirs {
                println!("Listening for {}", dir.to_str().unwrap());
                handles.push(tokio::task::spawn(async move {
                    loop {
                        match run(&dir, true).await {
                            Ok(_) => {}
                            Err(err) => {
                                eprintln!("{}", err.to_string());
                            }
                        };
                    }
                }));
            }
            futures::future::join_all(handles).await;
        } else {
            for dir in start_dirs {
                match run(&dir, false).await {
                    Ok(_) => {
                        println!("Transpiled {}", dir.to_str().unwrap())
                    }
                    Err(err) => {
                        eprintln!("{}", err.to_string())
                    }
                }
            }
        }
    }
}

fn add_start_directories(
    path: &Path,
    list: &mut Vec<PathBuf>,
    depth: usize,
) -> Result<(), io::Error> {
    if depth == 0 {
        return Ok(());
    }

    let paths = fs::read_dir(path)?.collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
    for entry in &paths {
        if entry.file_type()?.is_file() && entry.file_name() == "erpc.json" {
            list.push(path.to_path_buf());
            return Ok(());
        }
    }

    for entry in &paths {
        if entry.file_type()?.is_dir() {
            add_start_directories(&entry.path(), list, depth - 1)?;
        }
    }

    Ok(())
}
