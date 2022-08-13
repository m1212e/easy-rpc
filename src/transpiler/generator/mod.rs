use std::{
    collections::{hash_map::Entry, HashMap},
    fs::{self, read_dir, File, OpenOptions},
    io::Write,
    path::Path,
};

use super::{
    parser::{
        input_reader::InputReader,
        lexer::TokenReader,
        parser::{custom_type::CustomType, endpoint::Endpoint, parse},
    },
    ERPCError, Role,
};

mod tests;
mod typescript;

//TODO: some refactoring and documentation would be nice

/**
   Translates various easy-rpc elements into code strings of another language
*/
pub trait Translator {
    /**
       Translates a defined custom type into an interface of the target language
    */
    fn custom_type_to_interface(custom_type: &CustomType) -> String;

    /**
       Generates a class in the target language.

       The class name is the name used as classname.

       The relative path is used to build the endpoint identifier and should be the path of the class relative to the root folder. Path separators should be /.

       endpoints are all endpoints which the class should have.

       foreign indicates if the class should provide callbacks for handling incoming requests or methods for calling foreign endpoints.

       class_imports are all imports of sub classes which this class should import

       type_imports are all imports of custom types which are referenced in this class
    */
    fn generate_class(
        class_name: &str,
        relative_path: &str,
        endpoints: &Vec<Endpoint>,
        foreign: bool,
        class_imports: &Vec<String>,
        custom_types: &Vec<CustomType>,
    ) -> String;

    //TODO: make the file suffix return a slice instead of a string object
    /**
       Returns the file suffix for the generated language.

       E.g. TypeScript -> ts
       E.g. Rust -> rs
    */
    fn file_suffix() -> String;

    /**
       Generate the client class actually used by the user
    */
    fn generate_client(
        foreign: bool,
        class_imports: &Vec<String>,
        role: &Role,
        socket_enabled_browser_roles: &Vec<String>,
    ) -> String;
}

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
    )?;

    for (role, imports) in result.into_iter() {
        let generated =
            T::generate_client(
                role != selected_role_name,
                &imports,
                match all_roles.into_iter().find(|x| x.name == role) {
                    Some(v) => v,
                    None => return Err(ERPCError::ConfigurationError(format!(
                        "Could not find the specified role '{role}' in the configured role list"
                    ))),
                },
                socket_enabled_browser_roles,
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

fn generate_for_directory_recursively<T: Translator>(
    input_directory: &Path,
    output_directory: &Path,
    relative_path: &str,
    selected_role: &str,
) -> Result<HashMap<String, Vec<String>>, ERPCError> {
    let paths = read_dir(input_directory.join(relative_path))?;
    let mut generated_classnames_per_role_per_filename: HashMap<
        String,
        HashMap<String, Vec<String>>,
    > = HashMap::new();

    for entry in paths {
        match entry {
            Ok(entry) => {
                if entry.file_type()?.is_dir() {
                    let fi_na = entry.file_name();
                    let file_name = match fi_na.to_str() {
                        Some(val) => val,
                        None => {
                            return Err(String::from("Directory name is not valid UTF-8").into())
                        }
                    };

                    let mut new_rel_path = relative_path.to_string();
                    new_rel_path.push_str(file_name);
                    new_rel_path.push('/');

                    let generated_classes_per_role = generate_for_directory_recursively::<T>(
                        input_directory,
                        output_directory,
                        &new_rel_path,
                        selected_role,
                    )?;

                    generated_classnames_per_role_per_filename
                        .insert(file_name.to_string(), generated_classes_per_role);
                }
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

    let paths = read_dir(input_directory.join(relative_path))?;
    let mut generated_classnames_per_role: HashMap<String, Vec<String>> = HashMap::new();

    for entry in paths {
        match entry {
            Ok(entry) => {
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

                        match generated_classnames_per_role.entry(role) {
                            Entry::Occupied(mut entry) => {
                                entry.get_mut().push(file_name.to_string());
                            }
                            Entry::Vacant(entry) => {
                                entry.insert(vec![file_name.to_string()]);
                            }
                        }
                    }

                    generated_classnames_per_role_per_filename.remove(file_name);
                }
            }
            Err(err) => {
                return Err(err.into());
            }
        }
    }

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

fn generate_classes_per_role<T: Translator>(
    class_name: &str,
    relative_path: &str,
    endpoints: Vec<Endpoint>,
    selected_role: &str,
    custom_types: &Vec<CustomType>,
    classes_to_import_per_role: &HashMap<String, Vec<String>>,
) -> HashMap<String, String> {
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
