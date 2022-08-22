use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, read_dir, DirEntry, File, OpenOptions},
    io::Write,
    path::Path,
};

use self::translator::Translator;

use super::{
    config::Role,
    parser::{
        input_reader::InputReader,
        lexer::TokenReader,
        parser::{custom_type::CustomType, endpoint::Endpoint, parse},
    },
    validator::validate,
    ERPCError,
};

mod tests;
mod translator;

/**
   Generates code in the required directory structure at the target location.

   input_directory is the dir where the sources (.erpc files) live. It's structure is used to generate the output accordingly.

   output_directory is the target dir where the output will be generated.

   selected_role_name is a string which contains the name of the role which is selected through the config.json

   all_roles is a vec of all roles existing in the current setup

   socket_enabled_browser_roles is a vec of all roles which have the type browser and offer endpoints (and therefore need to support websockets)
*/
pub fn generate_for_directory<T: Translator>(
    input_directory: &Path,
    output_directory: &Path,
    selected_role_name: &str,
    all_roles: &Vec<Role>,
    socket_enabled_browser_roles: &Vec<String>,
) -> Result<(), ERPCError> {
    let result = generate_for_directory_recursively::<T>(
        input_directory,
        output_directory,
        "",
        &selected_role_name,
        all_roles,
    )?;

    //TODO make enums for configuration stuff like types?

    let source = if all_roles
        .into_iter()
        .find(|x| x.name == selected_role_name)
        .unwrap()
        .types
        .contains(&"browser".to_string())
    {
        "@easy-rpc/browser"
    } else {
        "@easy-rpc/node" // currently only supports node
    };

    for (role, imports) in result.into_iter() {
        let generated = T::generate_client(
            role != selected_role_name,
            &imports,
            match all_roles.into_iter().find(|x| x.name == role) {
                Some(v) => v,
                None => {
                    return Err(ERPCError::ConfigurationError(format!(
                        "Could not find the specified role '{role}' in the configured role list"
                    )))
                }
            },
            socket_enabled_browser_roles,
            source,
        );

        let mut generated_file_name = String::from(role);
        generated_file_name.push_str(".");
        generated_file_name.push_str(&T::file_suffix());

        fs::create_dir_all(&output_directory)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(output_directory.join(generated_file_name))?;
        file.write_all(&generated.as_bytes())?;
    }

    Ok(())
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
) -> Result<HashMap<String, Vec<String>>, ERPCError> {
    // to achieve consistency when testing, sort the directory entries when not in production build
    let mut paths = read_dir(input_directory.join(relative_path))?
        .collect::<Result<Vec<DirEntry>, std::io::Error>>()?;
    if cfg!(test) {
        paths.sort_by_key(|dir| dir.path());
    }

    /*
        for each processed file we store which classes were generated for what role
        we store them per filename to potentially merge classes from subdirectories and source files on this dir level which have an identical name to hybrid classes
    */
    let mut generated_classnames_per_role_per_filename: HashMap<
        String,
        HashMap<String, Vec<String>>,
    > = HashMap::new();

    /*
       first, iterate over all subdirectories and process them
       this needs to be done before processing files on the current dir level because we potentially need to import subclasses
    */
    for entry in &paths {
        if entry.file_type()?.is_dir() {
            let fi_na = entry.file_name();
            let file_name = match fi_na.to_str() {
                Some(val) => val,
                None => return Err(String::from("Directory name is not valid UTF-8").into()),
            };

            let mut new_rel_path = relative_path.to_string();
            new_rel_path.push_str(file_name);
            new_rel_path.push('/');

            // for the directory, just run this function recursively, but with an adjusted relative path
            let generated_classes_per_role = generate_for_directory_recursively::<T>(
                input_directory,
                output_directory,
                &new_rel_path,
                selected_role,
                all_roles,
            )?;

            // insert for the currently processed dir, all generated classes names per role
            generated_classnames_per_role_per_filename
                .insert(file_name.to_string(), generated_classes_per_role);
        }
    }

    // tracks which classes per role were generated on the current dir level
    let mut generated_classnames_per_role: HashMap<String, Vec<String>> = HashMap::new();

    // now iterate the .erpc files
    for entry in &paths {
        if entry.file_type()?.is_file() {
            let fi_na = entry.file_name();
            let file_name = match fi_na.to_str() {
                Some(val) => match val.strip_suffix(".erpc") {
                    Some(v) => v,
                    None => continue,
                },
                None => return Err(String::from("Filename name is not valid UTF-8").into()),
            };

            let mut reader = TokenReader::new(InputReader::new(File::open(entry.path())?))?;
            let result = parse(&mut reader)?;
            validate(&result.endpoints, &result.custom_types, all_roles)?;

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
                fs::create_dir_all(&parent)?;

                let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(parent.join(generated_file_name))?;
                file.write_all(&class_content.as_bytes())?;

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

            let mut file = OpenOptions::new().write(true).create(true).open(
                output_directory
                    .join(role.clone())
                    .join(relative_path)
                    .join(generated_file_name),
            )?;
            file.write_all(&class_content.as_bytes())?;

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

    Ok(generated_classnames_per_role)
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
