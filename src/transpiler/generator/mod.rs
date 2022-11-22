use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, read_dir, DirEntry, File, OpenOptions},
    io::Write,
    path::Path,
};

use crate::error::{Diagnostic, DisplayableError};

use self::translator::Translator;

use super::{
    config::{parse_roles, Role},
    parser::{
        input_reader::InputReader,
        lexer::TokenReader,
        parser::{custom_type::CustomType, endpoint::Endpoint, parse},
    },
    validator::validate,
};

mod tests;
pub mod translator;

/**
   Generates code in the required directory structure at the target location.

   input_directory is the dir where the sources (.erpc files) live. It's structure is used to generate the output accordingly.

   output_directory is the target dir where the output will be generated.

   selected_role_name is a string which contains the name of the role which is selected through the config.json
*/
pub fn generate_for_directory<T: Translator>(
    source_directory: &Path,
    output_directory: &Path,
    selected_role_name: &str,
) -> Vec<DisplayableError> {
    let path = source_directory.join("roles.json");
    if !path.exists() {
        return vec![format!(
            "Could not find roles.json at {path_str}",
            path_str = path
                .as_os_str()
                .to_str()
                .unwrap_or("<Unable to unwrap path>")
        )
        .into()];
    }

    let all_roles = match parse_roles(match File::open(path) {
        Ok(v) => v,
        Err(err) => {
            return vec![format!(
                "Could not open {path_str}: {err}",
                path_str = path
                    .as_os_str()
                    .to_str()
                    .unwrap_or("<Unable to unwrap path>")
            )
            .into()];
        }
    }) {
        Ok(v) => v,
        Err(err) => {
            return vec![format!(
                "Could not parse roles at {path_str}: {err}",
                path_str = path
                    .as_os_str()
                    .to_str()
                    .unwrap_or("<Unable to unwrap path>")
            )
            .into()];
        }
    };

    let result = generate_for_directory_recursively::<T>(
        source_directory,
        output_directory,
        "",
        &selected_role_name,
        &all_roles,
    );

    let errors = result.1;
    let classes_per_role = result.0;

    // all roles which have endpoints and are configured as browser
    let socket_enabled_browser_roles = &all_roles
        .iter()
        .filter_map(|role| {
            if classes_per_role.contains_key(&role.name)
                && role.role_type.contains(&"browser".to_string())
            {
                return Some(role.name.to_owned());
            }
            None
        })
        .collect();

    let source = if all_roles
        .iter()
        .find(|x| x.name == selected_role_name)
        .unwrap()
        .role_type
        .contains(&"browser".to_string())
    {
        "@easy-rpc/browser"
    } else {
        "@easy-rpc/node" // currently only supports node
    };

    for (role, imports) in classes_per_role.into_iter() {
        let role = match all_roles.iter().find(|x| x.name == role) {
            Some(v) => v,
            None => {
                errors.push(
                    format!(
                        "Could not find the specified role '{role}' in the configured role list"
                    )
                    .into(),
                );
                continue;
            }
        };

        let generated = T::generate_client(
            role.name != selected_role_name,
            &imports,
            role,
            socket_enabled_browser_roles,
            source,
        );

        let mut generated_file_name = String::from(role.name);
        generated_file_name.push_str(".");
        generated_file_name.push_str(&T::file_suffix());

        match fs::create_dir_all(&output_directory) {
            Ok(_) => {}
            Err(err) => {
                errors.push(
                    format!(
                        "Could not create directory structure for '{}': {err}",
                        output_directory
                            .to_str()
                            .unwrap_or("<could not unwrap path>")
                    )
                    .into(),
                );
                continue;
            }
        };

        let mut file = match OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(output_directory.join(generated_file_name))
        {
            Ok(v) => v,
            Err(err) => {
                errors.push(
                    format!(
                        "Could not open '{}' for write: {err}",
                        output_directory
                            .join(generated_file_name)
                            .to_str()
                            .unwrap_or("<could not unwrap path>")
                    )
                    .into(),
                );
                continue;
            }
        };
        match file.write_all(&generated.as_bytes()) {
            Ok(_) => {}
            Err(err) => {
                errors.push(
                    format!(
                        "Could not write to '{}': {err}",
                        output_directory
                            .join(generated_file_name)
                            .to_str()
                            .unwrap_or("<could not unwrap path>")
                    )
                    .into(),
                );
                continue;
            }
        };
    }

    errors
}

/**
   Internal recursive function to process a directory of erpc sources. Input/Output directory and selected role stay the same.
   The relative path specifies at which level relative of the root input dir this function should run.
   Returns which classes were generated for what role.
*/
fn generate_for_directory_recursively<T: Translator>(
    input_directory: &Path,
    output_directory: &Path,
    relative_path: &str,
    selected_role: &str,
    all_roles: &Vec<Role>,
) -> (HashMap<String, Vec<String>>, Vec<DisplayableError>) {
    // tracks which classes per role were generated on the current dir level
    let mut generated_classnames_per_role: HashMap<String, Vec<String>> = HashMap::new();
    let errors = vec![];

    /*
        for each processed file we store which classes were generated for what role
        we store them per filename to potentially merge classes from subdirectories and source files on this dir level which have an identical name to hybrid classes
    */
    let mut generated_classnames_per_role_per_filename: HashMap<
        String,
        HashMap<String, Vec<String>>,
    > = HashMap::new();

    let mut paths = match match read_dir(input_directory.join(relative_path)) {
        Ok(v) => v,
        Err(err) => {
            errors.push(
                format!(
                    "Could not read dir '{}': {err}",
                    input_directory
                        .join(relative_path)
                        .to_str()
                        .unwrap_or("<could not unwrap path>")
                )
                .into(),
            );
            return (generated_classnames_per_role, errors);
        }
    }
    .collect::<Result<Vec<DirEntry>, std::io::Error>>()
    {
        Ok(v) => v,
        Err(err) => {
            errors.push(
                format!(
                    "Could not collect dir entries for '{}': {err}",
                    input_directory
                        .join(relative_path)
                        .to_str()
                        .unwrap_or("<could not unwrap path>")
                )
                .into(),
            );
            return (generated_classnames_per_role, errors);
        }
    };

    // to achieve consistency when testing, sort the directory entries when not in production build
    if cfg!(test) {
        paths.sort_by_key(|dir| dir.path());
    }

    /*
       first, iterate over all subdirectories and process them
       this needs to be done before processing files on the current dir level because we potentially need to import subclasses
    */
    for entry in &paths {
        if match entry.file_type() {
            Ok(v) => v,
            Err(err) => {
                errors.push(
                    format!(
                        "Could not get file type for '{}': {err}",
                        entry.path().to_str().unwrap_or("<could not unwrap path>")
                    )
                    .into(),
                );
                continue;
            }
        }
        .is_dir()
        {
            let fi_na = entry.file_name();
            let file_name = match fi_na.to_str() {
                Some(val) => val,
                None => {
                    errors.push(
                        format!(
                            "File name is not valid UTF-8 for {}",
                            entry.path().to_str().unwrap_or("<could not unwrap path>")
                        )
                        .into(),
                    );
                    continue;
                }
            };

            // converting to a string may seem unnessecary but is required for code generation anyway.
            // it might be good to check if we can work with a path here and do the string conversion when its actually needed for code generation
            let mut new_rel_path = relative_path.to_string();
            new_rel_path.push_str(file_name);
            new_rel_path.push('/');

            // for the directory, just run this function recursively, but with an adjusted relative path
            let mut result = generate_for_directory_recursively::<T>(
                input_directory,
                output_directory,
                &new_rel_path,
                selected_role,
                all_roles,
            );

            let generated_classes_per_role = result.0;
            errors.append(&mut result.1);

            // insert for the currently processed dir, all generated classes names per role
            generated_classnames_per_role_per_filename
                .insert(file_name.to_string(), generated_classes_per_role);
        }
    }

    // now iterate the .erpc files
    for entry in &paths {
        if match entry.file_type() {
            Ok(v) => v,
            Err(err) => {
                errors.push(
                    format!(
                        "Could not get file type for '{}': {err}",
                        entry.path().to_str().unwrap_or("<could not unwrap path>")
                    )
                    .into(),
                );
                continue;
            }
        }
        .is_file()
        {
            let fi_na = entry.file_name();
            let file_name = match fi_na.to_str() {
                Some(val) => match val.strip_suffix(".erpc") {
                    Some(v) => v,
                    None => continue,
                },
                None => {
                    errors.push(
                        format!(
                            "File name is not valid UTF-8 for {}",
                            entry.path().to_str().unwrap_or("<could not unwrap path>")
                        )
                        .into(),
                    );
                    continue;
                }
            };

            let mut reader =
                match TokenReader::new(InputReader::new(match File::open(entry.path()) {
                    Ok(v) => v,
                    Err(err) => {
                        errors.push(
                            format!(
                                "Could not open file {}: {err}",
                                entry.path().to_str().unwrap_or("<could not unwrap path>")
                            )
                            .into(),
                        );
                        continue;
                    }
                })) {
                    Ok(v) => v,
                    Err(err) => {
                        errors.push(
                            format!(
                                "Input reader error occurred at {}: {err}",
                                entry.path().to_str().unwrap_or("<could not unwrap path>")
                            )
                            .into(),
                        );
                        continue;
                    }
                };
            let result = match parse(&mut reader) {
                Ok(val) => val,
                Err(err) => {
                    errors.push(DisplayableError::Diagnostic(Diagnostic {
                        source: entry.path(),
                        range: err.range,
                        message: err.message,
                    }));
                    continue;
                }
            };

            let mut validation_error_occurred = false;
            for validation_error in
                validate(&result.endpoints, &result.custom_types, all_roles).into_iter()
            {
                validation_error_occurred = true;
                errors.push(DisplayableError::Diagnostic(Diagnostic {
                    source: entry.path(),
                    range: validation_error.range,
                    message: validation_error.message,
                }));
            }
            if validation_error_occurred {
                continue;
            }

            // generate class strings per role
            let generated_class_content_per_role = generate_classes_per_role::<T>(
                file_name,
                relative_path,
                result.endpoints,
                selected_role,
                &result.custom_types,
                generated_classnames_per_role_per_filename
                    .get(file_name)
                    .unwrap_or(&HashMap::new()),
            );

            for (role, class_content) in generated_class_content_per_role {
                // write all generated files to the disk
                let mut generated_file_name = file_name.to_string();
                generated_file_name.push_str(".");
                generated_file_name.push_str(T::file_suffix().as_str());

                let parent = output_directory.join(role.clone()).join(relative_path);
                match fs::create_dir_all(&parent) {
                    Ok(_) => {}
                    Err(err) => {
                        errors.push(
                            format!(
                                "Could not create directory structure for (2) {}: {err}",
                                parent.to_str().unwrap_or("<could not unwrap path>")
                            )
                            .into(),
                        );
                        continue;
                    }
                };

                let mut file = match OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(parent.join(generated_file_name))
                {
                    Ok(v) => v,
                    Err(err) => {
                        errors.push(
                            format!(
                                "Could not open '{}' for write (2): {err}",
                                parent
                                    .join(generated_file_name)
                                    .to_str()
                                    .unwrap_or("<could not unwrap path>")
                            )
                            .into(),
                        );
                        continue;
                    }
                };
                match file.write_all(&class_content.as_bytes()) {
                    Ok(_) => {}
                    Err(err) => {
                        errors.push(
                            format!(
                                "Could not write to '{}' (2): {err}",
                                parent
                                    .join(generated_file_name)
                                    .to_str()
                                    .unwrap_or("<could not unwrap path>")
                            )
                            .into(),
                        );
                        continue;
                    }
                };

                // and store that the class has been generated
                match generated_classnames_per_role.entry(role) {
                    Entry::Occupied(mut entry) => {
                        entry.get_mut().push(file_name.to_string());
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(vec![file_name.to_string()]);
                    }
                }
            }

            // if a class with the exact same name of a subclass has been generated, remove it from the map since it is now already imported
            generated_classnames_per_role_per_filename.remove(file_name);
        }
    }

    // for the remaining classes, create import wrappers which do nothing but import the subclasses
    for (file_name, classnames_per_role) in generated_classnames_per_role_per_filename {
        let generated_class_content_per_role = generate_classes_per_role::<T>(
            &file_name,
            relative_path,
            vec![],
            selected_role,
            &vec![],
            &classnames_per_role,
        );

        for (role, class_content) in generated_class_content_per_role {
            let mut generated_file_name = file_name.to_string();
            generated_file_name.push_str(".");
            generated_file_name.push_str(T::file_suffix().as_str());

            let mut file = match OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(
                    output_directory
                        .join(role.clone())
                        .join(relative_path)
                        .join(generated_file_name),
                ) {
                Ok(v) => v,
                Err(err) => {
                    errors.push(
                        format!(
                            "Could not open '{}' for write (3): {err}",
                            output_directory
                                .join(role.clone())
                                .join(relative_path)
                                .join(generated_file_name)
                                .to_str()
                                .unwrap_or("<could not unwrap path>")
                        )
                        .into(),
                    );
                    continue;
                }
            };
            match file.write_all(&class_content.as_bytes()) {
                Ok(_) => {}
                Err(err) => {
                    errors.push(
                        format!(
                            "Could not write to '{}' (3): {err}",
                            output_directory
                                .join(role.clone())
                                .join(relative_path)
                                .join(generated_file_name)
                                .to_str()
                                .unwrap_or("<could not unwrap path>")
                        )
                        .into(),
                    );
                    continue;
                }
            };

            match generated_classnames_per_role.entry(role) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().push(file_name.to_string());
                }
                Entry::Vacant(entry) => {
                    entry.insert(vec![file_name.to_string()]);
                }
            }
        }
    }

    (generated_classnames_per_role, errors)
}

/**
   Generates various classes from one erpc source input. The classes are separated according to the role they belong to.
   Returns a map of classes generated per role.
*/
fn generate_classes_per_role<T: Translator>(
    class_name: &str,
    relative_path: &str,
    endpoints: Vec<Endpoint>,
    selected_role: &str,
    custom_types: &Vec<CustomType>,
    classes_to_import_per_role: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
    // sort endpoints by their role
    let mut endpoints_per_role: HashMap<String, Vec<Endpoint>> = HashMap::new();
    for endpoint in endpoints {
        match endpoints_per_role.entry(endpoint.role.clone()) {
            Entry::Occupied(mut e) => {
                e.get_mut().push(endpoint);
            }
            Entry::Vacant(e) => {
                e.insert(vec![endpoint]);
            }
        }
    }

    let mut classes_by_role: HashMap<String, String> = HashMap::new();

    for (current_role, value) in endpoints_per_role.iter() {
        classes_by_role.insert(
            current_role.to_owned(),
            T::generate_class(
                class_name,
                relative_path,
                value,
                selected_role != current_role,
                classes_to_import_per_role
                    .get(current_role)
                    .unwrap_or(&vec![]),
                custom_types,
            ),
        );
    }
    classes_by_role
}
