mod error;
mod language_server;
mod tests;
mod transpiler;
mod util;
use std::{
    env::{self, current_dir},
    fs::{self, DirEntry, File},
    io::{self},
    path::{Path, PathBuf},
    time::Duration,
};

use error::DisplayableError;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use tokio::runtime::Handle;
use transpiler::run;
use util::normalize_path::normalize_path;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    run_main(args).await;
}

async fn run_main(args: Vec<String>) {
    let entry_path = match args.iter().position(|e| *e == "-p") {
        Some(index) => match args.get(index + 1) {
            Some(v) => normalize_path(&PathBuf::from(v)),
            None => {
                eprintln!("Could not find path argument after -p flag");
                return;
            }
        },
        None => current_dir().unwrap(),
    };

    if args.contains(&"-ls".to_string()) {
        let (sender, reciever) = async_channel::unbounded::<Vec<DisplayableError>>();
        tokio::spawn(language_server::run_language_server(reciever));
        run_watch(entry_path, sender, false).await;
    } else if args.contains(&"-w".to_string()) {
        let (sender, reciever) = async_channel::unbounded::<Vec<DisplayableError>>();
        // just log the incoming results to console
        tokio::spawn(async move {
            loop {
                println!("{:#?}", reciever.recv().await);
            }
        });
        run_watch(entry_path, sender, true).await;
    } else {
        println!("{}", run_once(entry_path).await);
    }
}

async fn run_watch(
    entry_path: PathBuf,
    error_reporter: async_channel::Sender<Vec<DisplayableError>>,
    report_success: bool,
) {
    let root_dirs = loop {
        //TODO optimize
        let entry_path = entry_path.clone();
        match get_root_dirs(entry_path) {
            Ok(v) => break v,
            Err(err) => {
                error_reporter.send(vec![err]).await.unwrap();
            }
        };

        //give the user some time to adjust and dont spam them
        tokio::time::sleep(Duration::from_millis(10000)).await;
    };

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = loop {
            match read_config(&root_dir) {
                Ok(v) => break v,
                Err(err) => {
                    error_reporter.send(vec![err]).await.unwrap();
                }
            };

            //give the user some time to adjust and dont spam them
            tokio::time::sleep(Duration::from_millis(10000)).await;
        };
        let generated = root_dir.join(".erpc").join("generated");
        if generated.exists() {
            match fs::remove_dir_all(&generated) {
                Ok(_) => {}
                Err(err) => {
                    error_reporter
                        .send(vec![format!(
                            "Could not remove directory at (2) {}: {err}",
                            generated.to_str().unwrap_or("<could not unwrap path>")
                        )
                        .into()])
                        .await
                        .unwrap();
                }
            };
        }

        for source in config.sources {
            let root_dir = root_dir.clone();
            let role = config.role.clone();

            let error_reporter = error_reporter.clone();
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
                        error_reporter.send(vec![err.into()]).await.unwrap();
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
                                error_reporter.send(vec![err.into()]).await.unwrap();
                                return;
                            }
                        },
                        Err(err) => {
                            error_reporter.send(vec![err.into()]).await.unwrap();
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
                                error_reporter.send(res).await.unwrap();
                            } else {
                                if report_success {
                                    error_reporter
                                        .send(vec![format!(
                                            "Processed {}\n",
                                            root_dir.to_str().unwrap()
                                        )
                                        .into()])
                                        .await
                                        .unwrap();
                                } else {
                                    error_reporter.send(vec![]).await.unwrap();
                                }
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

async fn run_once(entry_path: PathBuf) -> String {
    let root_dirs = match get_root_dirs(entry_path) {
        Ok(v) => v,
        Err(err) => return err.message(),
    };

    let mut handles = vec![];
    for root_dir in root_dirs {
        let config = match read_config(&root_dir) {
            Ok(val) => val,
            Err(err) => {
                return err.message();
            }
        };

        let generated = root_dir.join(".erpc").join("generated");
        if generated.exists() {
            match fs::remove_dir_all(&generated) {
                Ok(_) => {}
                Err(err) => {
                    return format!(
                        "Could not remove directory at {}: {err}",
                        generated.to_str().unwrap_or("<could not unwrap path>")
                    );
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
                        ret.push_str(&format!("{:#?}\n", e));
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

    "".to_string()
}

fn read_config(root_dir: &Path) -> Result<crate::transpiler::config::Config, DisplayableError> {
    let path = root_dir.join("erpc.json");
    if !path.exists() {
        return Err(format!(
            "Could not find erpc.json at {path_str}",
            path_str = path
                .as_os_str()
                .to_str()
                .unwrap_or("<Unable to unwrap path>")
        )
        .into());
    }

    let result = crate::transpiler::config::parse_config(match File::open(path.clone()) {
        Ok(v) => v,
        Err(err) => {
            return Err(format!(
                "Could not open config at {}: {err}",
                path.to_str().unwrap_or("<could not unwrap path>")
            )
            .into());
        }
    });

    match result {
        Ok(v) => Ok(v),
        Err(err) => Err(format!(
            "Could not parse config at {}: {err}",
            path.to_str().unwrap_or("<could not unwrap path>")
        )
        .into()),
    }
}

fn get_root_dirs(start_dir: PathBuf) -> Result<Vec<PathBuf>, DisplayableError> {
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
            return Err(format!("Could not read root dirs: {err}").into());
        }
    };

    if start_dirs.len() == 0 {
        return Err("Could not find any easy-rpc project root. Make sure the project contains an erpc.json at its root.".to_string().into());
    }

    Ok(start_dirs)
}
