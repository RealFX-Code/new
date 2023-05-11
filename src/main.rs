use json;
use std::fs;
use std::env;
use std::io::Write;
use std::path::Path;
use directories::UserDirs;
use std::process::Command;

fn start_template(template : &str, _template_location : &str) {
    let template_path : String = _template_location.to_owned() + &template.replace("/","-") + ".json";
    
    let content : String = fs::read_to_string(template_path)
        .expect("[xx] Reading template json file");
    let content_obj: json::JsonValue = json::parse(&content).unwrap();
    let argument_array: &json::JsonValue = &content_obj["command"]["args"];
    let mut _cmd_to_run: String = content_obj["command"]["exec"][0].to_string();
    println!("[!!] Fill in the following information to create your project");
    for arg in argument_array.members() {
        print!("{}: ",arg["placeholder"].to_string());
        std::io::stdout().flush().unwrap();
        let mut user_input = String::new();
        let stdin = std::io::stdin();
        stdin.read_line(&mut user_input)
            .expect("[xx] error while gathering user input!");
        _cmd_to_run = _cmd_to_run.replace(&arg["replace"].to_string(), &user_input);
    }
    _cmd_to_run = _cmd_to_run
        .replace("\r", "")
        .replace("\n", "");
    println!("[VV] finished command: {}", &_cmd_to_run);
    let _cmd_run: std::process::ExitStatus = Command::new("bash")
                    .args(["-c", &_cmd_to_run])
                    .status()
                    .expect("[xx] failed to execute process");
}

fn show_help() {
    println!("new");
    println!(" - starting new projects you won't finish, but easier!");
    println!("");
    println!("Usage:");
    println!(" $ new                                        | Shows this help message");
    println!(" $ new < --help | -help | -h | /? >           | Shows this help message");
    println!(" $ new < --search | -search | -s > %template% | Searches for a template");
    println!(" $ new < --list | -list | -l >                | Lists all templates.");
    println!(" $ new %template&                             | Starts the specified template");
}

fn search(template : &str, _template_location : &str) {
    println!("Searching for {}...", template);
    let template_path : String = _template_location.to_owned() + &template.replace("/","-") + ".json";
    if Path::new(&template_path).exists() {
        println!("[00] Template found: {}", template);
    } else {
        println!("[xx] Template not found: {}", template);
    }
}

fn list_templates(_template_location : &str) {
    println!("[!!] Listing all avaiable templates!");

    // Powered by ChatGPT! ( idk how to write rust code :sobs: )
    if let Ok(entries) = fs::read_dir(_template_location) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(file_name) = entry.file_name().to_str() {
                    let file_name = Path::new(file_name)
                        .file_stem()
                        .and_then(|stem| stem.to_str())
                        .unwrap_or(file_name)
                        .replace("-", "/");

                    println!(" - {}", file_name);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let argument_search: [&str; 3] = [
        "--search",
        "-search",
        "-s"
    ];

    let argument_help: [&str; 4] = [
        "--help",
        "-help",
        "-h",
        "/?" // gotta care somewhat about the legacy windows users :D
    ];

    let argument_list : [&str; 3] = [
        "--list",
        "-list",
        "-l"
    ];
    
    let mut template_location : String = String::new();
    let mut _home_dir : String = String::new();

    if let Some(user_dirs) = UserDirs::new() {
        _home_dir = Path::new(user_dirs.home_dir()).display().to_string();
        template_location = _home_dir + "/.config/new-config/templates/";
    }

    if !Path::new(&template_location).exists() {
        println!("[XX] Looks like you don't have any templates!");
        println!(" - You're about to experience an error!");
    }

    if args.len() > 1 {
        let action: &str = &args[1] as &str;
        if argument_search.contains(&action as &&str) {
            if args.len() > 2 {
                let query : &str = &args[2] as &str;
                search(&query, &template_location);
            } else {
                println!("[xx] You have to supply a search query!");
            }
        } else if argument_help.contains(&action as &&str) {
            show_help();
        } else if argument_list.contains(&action as &&str) {
            list_templates(&template_location);
        } else {
            start_template(&action, &template_location);
        }
    } else {
        show_help();
    }
}