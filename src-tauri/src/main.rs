// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod manager;
mod search;

use device_query::{DeviceQuery, DeviceState, Keycode};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args.contains(&"--help".to_string()) || args.contains(&"-h".to_string()) {
            //TODO: Improve/modify settings sub-options
            let help_text = "\
\u{001B}[1;44m\u{001B}[97m FINDOMNI - FAST FILE FINDER \u{001B}[0m
\u{001B}[1;37m──────────────────────────────────────────\u{001B}[0m
\u{001B}[1;92mA program to quickly find files and directories with Rust.\u{001B}[0m

\u{001B}[1;4mUSAGE:\u{001B}[0m
\u{001B}[1;37m  findomni [OPTIONS] [SUB-OPTIONS] <SEARCH_TERM>\u{001B}[0m

\u{001B}[1;4mOPTIONS:\u{001B}[0m
\u{001B}[1;35m  -h, --help          \u{001B}[0m\u{001B}[90mShow this help message and exit.\u{001B}[0m
\u{001B}[1;36m  --nogui             \u{001B}[0m\u{001B}[90mRun the search in CLI mode without a graphical user interface.\u{001B}[0m
\u{001B}[1;34m  --setting           \u{001B}[0m\u{001B}[90mChange the settings\u{001B}[0m

\u{001B}[1;4mSUB-OPTIONS for --nogui:\u{001B}[0m
\u{001B}[1;36m  -so, -smart-only        \u{001B}[0m\u{001B}[90mEnable only smart search (skips full search after a timeout).\u{001B}[0m
\u{001B}[1;36m  -fo, -full-only         \u{001B}[0m\u{001B}[90mEnable only full search (disables smart search).\u{001B}[0m
\u{001B}[1;36m  -p, -path <DIRECTORY>   \u{001B}[0m\u{001B}[90mRestrict the search to a specific directory path.\u{001B}[0m
\u{001B}[1;36m  -t, -timeout <SECONDS>  \u{001B}[0m\u{001B}[90mStop the search after the specified time (in seconds).\u{001B}[0m
\u{001B}[1;36m  -e, -exclude <PATTERN>  \u{001B}[0m\u{001B}[90mExclude files or directories matching the given pattern from the search results. (Example: -exclude \"*.tmp\"\u{001B}[0m
\u{001B}[1;36m  -l, -limit <NUMBER>     \u{001B}[0m\u{001B}[90mLimit the number of search results.\u{001B}[0m
\u{001B}[1;36m  -o, -output <FILE>      \u{001B}[0m\u{001B}[90mSave search results to a specified file.\u{001B}[0m
\u{001B}[1;36m  -v, -verbose            \u{001B}[0m\u{001B}[90mShow detailed information about the search process (e.g., directories scanned, skipped files).\u{001B}[0m
\u{001B}[1;36m  -s, -size <FILE-SIZE>   \u{001B}[0m\u{001B}[90mFilter by file size\u{001B}[0m
\u{001B}[1;36m  -d, -date <YYYY-MM-DD>  \u{001B}[0m\u{001B}[90mFilter by last modified date.\u{001B}[0m

\u{001B}[1;4mSUB-OPTIONS for --setiings:\u{001B}[0m
\u{001B}[1;35m  -add-smart <PATH>       \u{001B}[0m\u{001B}[90mAdd smart search path in settings\u{001B}[0m
\u{001B}[1;35m  -remove-smart <PATH>    \u{001B}[0m\u{001B}[90mRemove smart search path in settings\u{001B}[0m
\u{001B}[1;35m  -e, -exclude            \u{001B}[0m\u{001B}[90mABCD\u{001B}[0m

\u{001B}[1;4mFEATURES:\u{001B}[0m
\u{001B}[1;33m• Smart Search:  \u{001B}[0m\u{001B}[90mPrioritize common directories like \u{001B}[1mC:\\Users\\<user>\\Documents\u{001B}[0m\u{001B}[90m for faster results.\u{001B}[0m
\u{001B}[1;33m• Full Search:   \u{001B}[0m\u{001B}[90mDeep scan of the entire file system for comprehensive results.\u{001B}[0m
\u{001B}[1;33m• No GUI Mode:   \u{001B}[0m\u{001B}[90mRun entirely from the command line for minimal resource usage.\u{001B}[0m";
            
            println!("{}", help_text);
            return;
        } else if args.contains(&"--nogui".to_string()) {
            println!("NO GUI");
            let query = &args[args.len() - 1];
            if args.contains(&"-so".to_string()) || args.contains(&"-smart-only".to_string()) {
                manager::task_manager("smart-only".to_string(), query.to_string());
            } else if args.contains(&"-fo".to_string()) || args.contains(&"-full-only".to_string()) {
                manager::task_manager("full-only".to_string(), query.to_string());
            } else if args.contains(&"-p".to_string()) || args.contains(&"-path".to_string()) {
                manager::task_manager("path".to_string(), query.to_string());
            }
        } else if args.contains(&"--settings".to_string()) {
            println!("SETTINGS");
        }
        println!("{:?}", args)
    } else {
        // Create a new DeviceState object to query the keyboard state
        let device_state = DeviceState::new();

        loop {
            // Get the current state of the keys
            let keys: Vec<Keycode> = device_state.get_keys();

            // Check if ALT and 's' keys are pressed simultaneously
            if keys.contains(&Keycode::LAlt) && keys.contains(&Keycode::S) {
                println!("Keys pressed");
                findomni_lib::run();
                println!("GUI closed")
            }
        }
    }
}