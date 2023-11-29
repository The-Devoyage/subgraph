pub fn clean_string(v: &String) -> String {
    v.replace("\n", "").replace("\"", "")
}
