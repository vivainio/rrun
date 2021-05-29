use std::path::{PathBuf, Path};
use std::process::Command;
use which;

fn find_adjacent(path: &PathBuf, to_find: &str) -> Option<PathBuf> {
    let tries = vec![
        path.join(to_find),
        path.join(to_find.to_owned() + ".cmd"),
        path.join(to_find.to_owned() + ".py"),
        path.join(to_find).join("node_modules/.bin/".to_owned() + to_find + ".cmd")
    ];
    for trie in tries {
        dbg!(&trie);
        if trie.exists() {
            return Some(trie);
        }
    }
    None
}

fn find_in_parents(to_find: &str) -> Option<PathBuf> {
    let mut curpath = std::env::current_dir().unwrap();
    loop {
        let found = find_adjacent(&curpath, &to_find);
        if found.is_some() {
            return found;
        }
        let popped = curpath.pop();
        if !popped {
            return None
        }
    }
}

fn find_in_path(cmd: &str) -> String {
    let found = which::which(&cmd).expect("Could not find binary in path");

    let asstr = found.to_string_lossy();
    asstr.into()
}

fn run_cmd_with_current_args(in_path: &Path, command: &PathBuf) {

    let mut args_to_run: Vec<String> = std::env::args().skip(2).collect();

    args_to_run.insert(0, command.to_string_lossy().into());
    if command.to_string_lossy().ends_with(".py") {
        args_to_run.insert(0, find_in_path("python.exe").into())
    }

    dbg!(&args_to_run);

    let main_command = args_to_run.first().unwrap();

    let _cmd = Command::new(main_command)
        .args(&args_to_run[1..])
        .current_dir(&in_path)
        .status()
        .expect("Could not launch command");
    ();

}

#[test]
fn adj() {
    let got = find_in_parents("test");
    assert!(got.is_some());
}


fn main() {
    let mut args = std::env::args();
    args.next().unwrap();
    let to_run = args.next().expect("Did not specify command to run");
    let command = find_in_parents(&to_run);
    match command {
        None => {
            println!("No matching command found for: {}", &to_run);
        }
        Some(cmd) => {
            let in_path = cmd.parent().unwrap();
            run_cmd_with_current_args(&in_path, &cmd);
        }
    }
}
