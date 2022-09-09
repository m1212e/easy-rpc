mod language_server;
mod transpiler;
mod util;
use std::{
    env::{self, current_dir},
    fs::{self, DirEntry, File},
    io::{self},
    path::{Path, PathBuf},
};

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::{io::AsyncWriteExt, runtime::Handle};
use transpiler::{run, ERPCError};
use util::normalize_path::normalize_path;

//TODO erpcerror should be in separate file (errors in separate file generally?)
//TODO restructure for cleaner error handling
//TODO ensure all errors are collected before passing to language server for diagnostics (watch channel?)
//TODO increase rubustness

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-ls".to_string()) {
        run_ls_mode().await;
    } else if args.contains(&"-w".to_string()) {
        run_watch_mode().await;
    } else {
        run_normal_mode().await;
    }
}

async fn run_ls_mode() {
    let (sender, reciever) = async_channel::unbounded::<Vec<ERPCError>>();
    let ls = tokio::spawn(language_server::run_language_server(reciever));
    let root_dirs = match get_root_dirs() {
        Ok(val) => {
            if val.len() == 0 {
                sender.send(vec![ERPCError::ConfigurationError("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.".to_string())]).await.unwrap();
                return;
            }
            val
        }
        Err(err) => {
            sender
                .send(vec![ERPCError::ConfigurationError(format!(
                    "Could not read root dirs: {}",
                    err.to_string()
                ))])
                .await
                .unwrap();
            return;
        }
    };

    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                sender.send(vec![err]).await.unwrap();
                return;
            }
        };

        for source in config.sources {
            let root_dir = root_dir.clone();
            let role = config.role.clone();

            let (watch_sender, watch_reciever) = async_channel::bounded(1);

            let handle = Handle::current();
            let mut watcher = match RecommendedWatcher::new(
                move |res| {
                    let sdr = watch_sender.clone();
                    handle.spawn(async move {
                        sdr.send(res).await.unwrap();
                    });
                },
                notify::Config::default(),
            ) {
                Ok(val) => val,
                Err(err) => {
                    sender
                        .send(vec![ERPCError::NotifyError(err)])
                        .await
                        .unwrap();
                    return;
                }
            };

            let normalized_source_path = normalize_path(&root_dir.join(&source));

            watcher
                .watch(&normalized_source_path, RecursiveMode::Recursive)
                .unwrap();

            loop {
                let event = match watch_reciever.recv().await {
                    Ok(val) => match val {
                        Ok(val) => val,
                        Err(err) => {
                            sender
                                .send(vec![ERPCError::NotifyError(err)])
                                .await
                                .unwrap();
                            return;
                        }
                    },
                    Err(err) => {
                        sender.send(vec![ERPCError::RecvError(err)]).await.unwrap();
                        return;
                    }
                };

                match event.kind {
                    notify::EventKind::Create(_)
                    | notify::EventKind::Modify(_)
                    | notify::EventKind::Remove(_) => {
                        let res = run(
                            &normalized_source_path,
                            &root_dir.join(".erpc").join("generated"),
                            &role,
                        )
                        .await;
                        sender.send(res).await.unwrap();
                    }
                    _ => {}
                }
            }
        }
    }
    ls.await.unwrap();
}

async fn run_watch_mode() {
    let root_dirs = match get_root_dirs() {
        Ok(val) => {
            if val.len() == 0 {
                eprintln!("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.");
                return;
            }
            val
        }
        Err(err) => {
            eprintln!("Could not read root dirs: {}", err.to_string());
            return;
        }
    };

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                tokio::io::stderr()
                    .write(format!("{err}\n").as_bytes())
                    .await
                    .unwrap();
                return;
            }
        };

        for source in config.sources {
            let root_dir = root_dir.clone();
            let role = config.role.clone();

            handles.push(tokio::spawn(async move {
                let (sender, reciever) = async_channel::bounded(1);

                let handle = Handle::current();
                let mut watcher = match RecommendedWatcher::new(
                    move |res| {
                        let sender = sender.clone();
                        handle.spawn(async move {
                            sender.send(res).await.unwrap();
                        });
                    },
                    notify::Config::default(),
                ) {
                    Ok(val) => val,
                    Err(err) => {
                        tokio::io::stderr()
                            .write(format!("{err}\n").as_bytes())
                            .await
                            .unwrap();
                        return;
                    }
                };

                let normalized_source_path = normalize_path(&root_dir.join(&source));

                watcher
                    .watch(&normalized_source_path, RecursiveMode::Recursive)
                    .unwrap();

                loop {
                    let event = match reciever.recv().await {
                        Ok(val) => match val {
                            Ok(val) => val,
                            Err(err) => {
                                tokio::io::stderr()
                                    .write(format!("{err}\n").as_bytes())
                                    .await
                                    .unwrap();
                                return;
                            }
                        },
                        Err(err) => {
                            tokio::io::stderr()
                                .write(format!("{err}\n").as_bytes())
                                .await
                                .unwrap();
                            return;
                        }
                    };

                    match event.kind {
                        notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_) => {
                            let res = run(
                                &normalized_source_path,
                                &root_dir.join(".erpc").join("generated"),
                                &role,
                            )
                            .await;
                            if res.len() > 0 {
                                tokio::io::stderr()
                                    .write(format!("{:#?}\n", res).as_bytes())
                                    .await
                                    .unwrap();
                            } else {
                                tokio::io::stdout()
                                    .write(
                                        format!("Processed {}\n", root_dir.to_str().unwrap())
                                            .as_bytes(),
                                    )
                                    .await
                                    .unwrap();
                            }
                        }
                        _ => {}
                    }
                }
            }))
        }
    }
    futures::future::join_all(handles).await;
}

async fn run_normal_mode() {
    let root_dirs = match get_root_dirs() {
        Ok(val) => {
            if val.len() == 0 {
                eprintln!("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.");
                return;
            }
            val
        }
        Err(err) => {
            eprintln!("Could not read root dirs: {}", err.to_string());
            return;
        }
    };

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                tokio::io::stderr()
                    .write(format!("{err}\n").as_bytes())
                    .await
                    .unwrap();
                return;
            }
        };

        for source in config.sources {
            let root_dir = root_dir.clone();
            let role = config.role.clone();
            handles.push(tokio::task::spawn(async move {
                let res = run(
                    &normalize_path(&root_dir.join(source)),
                    &root_dir.join(".erpc").join("generated"),
                    &role,
                )
                .await;
                if res.len() > 0 {
                    tokio::io::stderr()
                        .write(format!("{:#?}\n", res).as_bytes())
                        .await
                        .unwrap();
                } else {
                    tokio::io::stdout()
                        .write(format!("Processed {}\n", root_dir.to_str().unwrap()).as_bytes())
                        .await
                        .unwrap();
                }
            }));
        }
    }
    futures::future::join_all(handles).await;
}

fn read_config(root_dir: &Path) -> Result<crate::transpiler::config::Config, ERPCError> {
    let path = root_dir.join("erpc.json");
    if !path.exists() {
        return Err(ERPCError::ConfigurationError(format!(
            "Could not find erpc.json at {path_str}",
            path_str = path
                .as_os_str()
                .to_str()
                .unwrap_or("<Unable to unwrap path>")
        )));
    }

    Ok(crate::transpiler::config::parse_config(File::open(path)?)?)
}

fn get_root_dirs() -> Result<Vec<PathBuf>, ERPCError> {
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

    let mut start_dirs: Vec<PathBuf> = Vec::new();
    add_start_directories(&current_dir().unwrap(), &mut start_dirs, 100)?;
    Ok(start_dirs)
}
