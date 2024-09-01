use std::path::PathBuf;

pub fn path_from_object_name(object: String) -> PathBuf {
    let dir = &object[0..2];
    let file_name = &object[2..];
    let path = PathBuf::new()
        .join(".git")
        .join("objects")
        .join(dir)
        .join(file_name);

    path
}
