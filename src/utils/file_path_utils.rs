pub fn get_file_path(file_name: &str) -> String {
    let home_dir = dirs::home_dir().unwrap();
    let file_path = home_dir.join(".poem").join(file_name);
    std::fs::create_dir_all(file_path.parent().unwrap()).unwrap();
    file_path.display().to_string()
}
