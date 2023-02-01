use rofi;
use clap::Parser;
use std::fs;
use std::path::Path;
use std::process;

use std::process::Command;

static TEMPLATE_PATTERN: &str = "[topicname]";

#[derive(Parser, Debug)]
#[command(author, version, about = "Notes manager", long_about = None)]
struct Args {
   /// Path to notes direcotry root
   #[arg(short, long)]
   notes_dir: String,

   /// Path to template directory
   #[arg(short, long)]
   template_dir: String,

   /// Path to startup script
   #[arg(short, long)]
   startup_script: String,
}


fn run_startup_script(script_path: &Path, notes_root: &Path) {
    // execute startup script
    let mut command = Command::new(script_path);

    let _process = match command
                        .args(&[notes_root.to_str().unwrap()])
                        .spawn() {
            Ok(process) => process,
            Err(err)    => panic!("Error while trying to run startup script: {}", err),
    };
}


fn prompt_user(prompt: &str, options: &Vec<String>) -> Result<String, rofi::Error>  {
    rofi::Rofi::new(options)
        .prompt(prompt)
        .run()
}


fn prompt_user_yes_no(prompt: &str) -> bool {
    match rofi::Rofi::new(&["yes", "no (go back)"])
        .prompt(prompt)
        .run()
    {
        Ok(c) => {
            if c == "yes" {
                true
            } else {
                false
            }
        }
        Err(_) => false
    }
}


// TODO: figure out return type
fn init_topic_directory(topic_path: &Path, template_dir: &Path) {
    // copy template files
    let mut options = fs_extra::dir::CopyOptions::new();
    options.content_only = true;

    match fs_extra::dir::copy(&template_dir, &topic_path, &options) {
        Ok(_) => {},
        Err(e) => {
            // TODO: move exit out of function and into caller
            eprintln!("Error while trying to copy template directory: {}", e);
            process::exit(1);
        }
    }

    // rename pattern in filenames
    let pattern = &TEMPLATE_PATTERN;
    let topic = topic_path.file_name().unwrap().to_str().unwrap();

    let paths = fs::read_dir(&topic_path).unwrap();

    for entry in paths {
        let e = entry.unwrap();
        let mut path = e.path(); // -> PathBuf

        if !(e.file_name().to_str().unwrap().contains(&*pattern)) {
            continue;
        }

        let s = e.file_name().to_str().unwrap().replace(&*pattern, &topic);

        path.set_file_name(&s);

        match fs::rename(e.path(), path) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error while trying to rename template files: {}", e);
                process::exit(1);
            }
        }
    }

}


fn main() {

    let args = Args::parse();

    // convert arguments to paths
    let notes_root     = Path::new(&args.notes_dir);
    let template_dir   = Path::new(&args.template_dir);
    let startup_script = Path::new(&args.startup_script);

    // check that they exist
    if !notes_root.is_dir() {
        eprintln!("{}: No such directory", notes_root.to_str().unwrap());
        process::exit(1);
    }

    if !template_dir.is_dir() {
        eprintln!("{}: No such directory", template_dir.to_str().unwrap());
        process::exit(1);
    }

    if !startup_script.is_file() {
        eprintln!("{}: No such file", startup_script.to_str().unwrap());
        process::exit(1);
    }


    // figure out the subject.
    // a state machine would be cool but it is outside of my current rust knowledge
    let mut subject_path;
    loop {

        // get all subject directories
        // and filter out non-directory entries
        let dir_entries = fs::read_dir(&notes_root)
            .unwrap()
            .filter(|d| d.as_ref().unwrap().path().is_dir())
            .map(|e| e.unwrap().path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))).unwrap())
            .collect::<Vec<String>>();

        // ask user which subject
        let subject = match prompt_user("select subject", &dir_entries) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        };

        subject_path = notes_root.join(subject);

        // check if subject is a new subject that does not exist yet.
        // if it does not exist ask user if they want to create it.
        // if yes create it
        // if no go back to subject selection
        if !subject_path.is_dir() {
            if prompt_user_yes_no("create new subject {}?") {
                // println!("user said yes!!");
                match fs::create_dir(&subject_path) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error while trying to create subject directory: {}", e);
                        process::exit(1);
                    }
                }

                println!("created new subject directory");

                break;

            } else {
                // println!("user said no or aborted");
            }
        } else {
            break;
        }
    }

    // let user select topic
    let mut topic_path;
    loop {

        let dir_entries = fs::read_dir(&subject_path)
            .unwrap()
            .filter(|d| d.as_ref().unwrap().path().is_dir())
            .map(|e| e.unwrap().path().file_name().and_then(|n| n.to_str().map(|s| String::from(s))).unwrap())
            .collect::<Vec<String>>();

        // ask user which topic
        let topic = match prompt_user("select topic", &dir_entries) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        };

        topic_path = subject_path.join(topic);

        // TODO:
        // put both prompt steps (subject, topic) into function
        // to save repeating code
        // maybe use callback for things like init_topic_directory(...)
        // that are not present in the subject step
        // or throw error and handle with return value

        // check if topic is a new topic that does not exist yet.
        // if it does not exist ask user if they want to create it.
        // if yes create it and copy template files
        // if no go back to topic selection
        if !topic_path.is_dir() {
            if prompt_user_yes_no("create new topic {}?") {

                match fs::create_dir(&topic_path) {
                    Ok(_) => {},
                    Err(e) => {
                        eprintln!("Error while trying to create topic directory: {}", e);
                        process::exit(1);
                    }
                }

                println!("created new topic directory");

                init_topic_directory(&topic_path, &template_dir);

                break;

            } else {
                // println!("user said no or aborted");
            }
        } else {
            break;
        }
    }

    // TODO: error handling
    run_startup_script(startup_script, topic_path.as_path());

    /*
    match rofi::Rofi::new_message("Something went wrong").run() {
        Err(rofi::Error::Blank) => (), // the expected case
        Ok(_) => (),  // should not happen
        Err(_) => () // Something went wrong
    }

    match rofi::Rofi::new_message("just kidding").run() {
        Err(rofi::Error::Blank) => (), // the expected case
        Ok(_) => (),  // should not happen
        Err(_) => println!("ERRORORORO") // Something went wrong
    }
    */
}
