pub fn path2name(x: String) -> String {
    norm(&x)
        .as_str()
        .split("/")
        .into_iter()
        .last()
        .map(|x| x.to_string())
        .unwrap_or("".to_string())
}

pub fn norm(path: &str) -> String {
    str::replace(path, "\\", "/")
}
