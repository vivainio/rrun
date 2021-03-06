use std::path::{PathBuf, Path};
use std::process::Command;
use which;
use std::collections::HashMap;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct PackageJson {
    scripts: HashMap<String, String>
}



fn find_adjacent(path: &PathBuf, to_find: &str) -> Option<PathBuf> {
    let tries = vec![
        path.join(to_find),
        path.join(to_find.to_owned() + ".exe"),
        path.join(to_find.to_owned() + ".cmd"),
        path.join(to_find.to_owned() + ".bat"),
        path.join(to_find.to_owned() + ".py"),
        path.join("node_modules/.bin/".to_owned() + to_find + ".cmd")
    ];
    for trie in tries {
        // dbg!(&trie);
        if trie.is_file() {
            return Some(trie);
        }
    }

    // ok, maybe we get package.json?

    let try_json = path.join("package.json");
    let cont = std::fs::read_to_string(try_json).ok()?;
    let read : PackageJson = serde_json::from_str(&cont).ok()?;
    dbg!(read);

    None
}

fn find_in_parents(to_find: &str) -> Option<(PathBuf, PathBuf)> {
    let mut curpath = std::env::current_dir().unwrap();
    loop {
        let found = find_adjacent(&curpath, &to_find);
        match found {
            Some(name ) => {
                return Some((name, curpath));
            }

            None => {
                let popped = curpath.pop();
                if !popped {
                    return None;
                }
            }
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

    // dbg!(&args_to_run);

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


fn do_main() -> Result<(), String>  {
    let mut args = std::env::args();
    // the first one is garbage always, it's rrun.exe itself
    args.next().unwrap();
    let to_run = args.next().ok_or("Usage: rrun <command> [arguments...]")?;
    let command = find_in_parents(&to_run).ok_or("Command not found")?;

    let (cmd, in_path) = command;

    run_cmd_with_current_args(&in_path, &cmd);
    Ok(())
}

fn main() {
    let r = do_main();
    match r  {
        Ok(_) => (),
        Err(text) => {
            println!("{}", text);
        }
    }
}