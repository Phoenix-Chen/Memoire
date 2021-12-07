use std::process::Command;


pub fn search(dir_path: &str, keyword: &str) {
    let bash = Command::new("bash")
        .arg("-c")
        .arg(build_bash_command(dir_path, keyword))
        .output()
        .expect("failed bash command");
    println!("{}", String::from_utf8_lossy(&bash.stdout));
}


fn build_select(keyword: &str) -> String{
    let contains: String = format!("contains(\"{}\")", keyword);
    format!("select(\
            (.command | {}) or \
            (.annotation | {}) or \
            (.collection | {}) or \
            (.tags | select(.[] | {}))\
        )", &contains, &contains, &contains, &contains)
}


fn build_bash_command(dir_path: &str, keyword: &str) -> String {
    format!("cat {}/*.json | jq -s '\
            [\
                map(to_entries | \
                map(.value + {{\"index\": .key}})) | \
                flatten | \
                .[] | \
                {}\
            ]'", &dir_path, &build_select(keyword))
}