mod language_server;
mod tests;
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

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    match run_main(args).await {
        Ok(_) => {}
        Err(err) => eprintln!("{}", err),
    };
}

async fn run_main(args: Vec<String>) -> Result<(), String> {
    let entry_path = match args.iter().position(|e| *e == "-p") {
        Some(index) => match args.get(index + 1) {
            Some(v) => std::fs::canonicalize(PathBuf::from(v)).unwrap(),
            None => {
                return Err("Could not find path argument after -p flag".to_string());
            }
        },
        None => current_dir().unwrap(),
    };

    if args.contains(&"-ls".to_string()) {
        run_ls_mode(entry_path).await
    } else if args.contains(&"-w".to_string()) {
        run_watch_mode(entry_path).await
    } else {
        run_normal_mode(entry_path).await
    }
}

async fn run_ls_mode(entry_path: PathBuf) -> Result<(), String> {
    let (sender, reciever) = async_channel::unbounded::<Vec<ERPCError>>();
    let ls = tokio::spawn(language_server::run_language_server(reciever));
    let root_dirs = match get_root_dirs(entry_path) {
        Ok(val) => {
            if val.len() == 0 {
                sender.send(vec![ERPCError::ConfigurationError("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.".to_string())]).await.unwrap();
                return Err("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.".to_string());
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
            return Err(format!("Could not read root dirs: {}", err.to_string()));
        }
    };

    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                sender.send(vec![err]).await.unwrap();
                return Err("Configuration error occured".to_string());
            }
        };

        let generated = root_dir.join(".erpc").join("generated");
        if generated.exists() {
            match fs::remove_dir_all(&generated) {
                Ok(_) => {}
                Err(err) => {
                    sender.send(vec![err.into()]).await.unwrap();
                    return Err("IO error occured".to_string());
                }
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
                    return Err("Notify error occured".to_string());
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
                            return Err("Notify error occured".to_string());
                        }
                    },
                    Err(err) => {
                        sender.send(vec![ERPCError::RecvError(err)]).await.unwrap();
                        return Err(err.to_string());
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

    Ok(())
}

async fn run_watch_mode(entry_path: PathBuf) -> Result<(), String> {
    let root_dirs = get_root_dirs(entry_path)?;

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                return Err(err.to_string());
            }
        };
        let generated = root_dir.join(".erpc").join("generated");
        if generated.exists() {
            match fs::remove_dir_all(&generated) {
                Ok(_) => {}
                Err(err) => {
                    return Err(err.to_string());
                }
            };
        }

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

    Ok(())
}

async fn run_normal_mode(entry_path: PathBuf) -> Result<(), String> {
    let root_dirs = get_root_dirs(entry_path)?;

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        let generated = root_dir.join(".erpc").join("generated");
        if generated.exists() {
            match fs::remove_dir_all(&generated) {
                Ok(_) => {}
                Err(err) => {
                    return Err(err.to_string());
                }
            };
        }

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
                    let mut ret = String::new();
                    for e in res {
                        ret.push_str(&e.to_string());
                        ret.push_str("\n");
                    }
                    Err(ret)
                } else {
                    Ok(format!("Processed {}\n", root_dir.to_str().unwrap()))
                }
            }));
        }
    }
    let results = futures::future::join_all(handles).await;
    let mut ret = String::new();
    for res in results {
        match res {
            Ok(v) => match v {
                Ok(v) => println!("{}", v),
                Err(err) => ret.push_str(&format!("{}\n", err)),
            },
            Err(err) => ret.push_str(&format!("{}\n", err)),
        }
    }

    if ret == "" {
        return Ok(());
    } else {
        return Err(ret);
    }
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

fn get_root_dirs(start_dir: PathBuf) -> Result<Vec<PathBuf>, String> {
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
    match add_start_directories(&start_dir, &mut start_dirs, 100) {
        Ok(_) => {}
        Err(err) => {
            return Err(format!("Could not read root dirs: {err}"));
        }
    };

    if start_dirs.len() == 0 {
        return Err("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.".to_string());
    }

    Ok(start_dirs)
}
