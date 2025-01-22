use rayon::prelude::*;
use walkdir::WalkDir;
use std::thread;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::{atomic::{AtomicU16, AtomicBool, Ordering}};
use regex::Regex;

pub static SEARCH_RUNNING: AtomicBool = AtomicBool::new(false); //When false stop all threads
pub static CURRENT_ID: AtomicU16 = AtomicU16::new(0);   //To check if the current thread the newest one is | If not stop the search

pub fn start_search(query: &str, smart: bool, full: bool) {
    set_search_id(1);
    let my_id: u16 = get_current_search_id();
    if let Ok(user_profile) = env::var("USERPROFILE") {
        println!("Using user profile: {}", user_profile);
        if smart == true {
            let smart_paths = vec![
                PathBuf::from(user_profile.clone() + r"\Desktop"),
                PathBuf::from(user_profile.clone() + r"\Documents"),
                PathBuf::from(user_profile.clone() + r"\Downloads"),
                PathBuf::from(user_profile.clone() + r"\Pictures"),
                match env::var("ProgramFiles") {
                    Ok(program_files) => PathBuf::from(program_files),
                    Err(_) => PathBuf::from("C:\\Program Files"),
                },
                match env::var("OneDrive") {
                    Ok(one_drive) => PathBuf::from(one_drive),
                    Err(_) => PathBuf::from(user_profile.clone() + r"\OneDrive"),
                }
            ];

            let query = query.to_string();
            let mut handles = vec![];

            for path in smart_paths.iter() {
                let path = path.clone();
                let query = query.clone();
                let my_id = my_id;
                let handle = thread::spawn(move || {
                    if my_id < get_current_search_id() && !get_search_runner() {
                        println!("Closing thread with ID: {}", my_id);
                        return; //to stop the thread
                    }
                    search_files(&path, &query, my_id);
                });

                handles.push(handle);
            }
            for handle in handles {
                if let Err(err) = &handle.join() {
                    eprintln!("Thread error: {:?}", err);
                }
            }
            
            if full == true {
                thread::sleep(Duration::new(5, 0));
                if my_id < get_current_search_id() && !get_search_runner() {
                    println!("Closing thread with ID: {}", my_id);
                    return; //Stop the threads
                }
                thread::sleep(Duration::new(5, 0));
            }
        }
         
        if full == true {
            //Get all Drives
            let mut drives: Vec<String> = Vec::new();
            for letter in b'A'..=b'Z' {
                let drive = format!("{}:\\", letter as char);
                if fs::metadata(&drive).is_ok() {
                    drives.push(drive);
                }
            }

            for drive in drives {
                let query = query.to_string();
                let additional_handle = thread::spawn(move || {
                    search_files(&PathBuf::from(drive), &query, my_id);
                });
                additional_handle.join().unwrap();
            }
        }
    } else {
        println!("No USER profile");
    }
}

fn search_files(base_path: &PathBuf, query: &str, my_id: u16) {
    let re = Regex::new(r"^[A-Z]:\\").unwrap();
    if re.is_match(base_path.to_str().unwrap()) {
        println!("Full search for {}", query);
        
    } else {
        println!("Searching for \"{}\" in \"{}\"", query, base_path.display());
    }
    if my_id < get_current_search_id() && !get_search_runner() {
        println!("Closing thread with ID: {}", my_id);
        return;
    }
    WalkDir::new(base_path)
        .into_iter()
        .par_bridge()
        .filter_map(|entry| {
            if my_id < get_current_search_id() && !get_search_runner() {
                println!("Closing thread with ID: {}", my_id);
                None
            } else {
                entry.map_err(|err| eprintln!("Error reading entry: {}", err)).ok().filter(|e| e.file_name().to_string_lossy().contains(query))
            }
        })
        .for_each(|entry| println!("{}", entry.path().display()));
}

pub fn get_search_runner() -> bool {
    SEARCH_RUNNING.load(Ordering::SeqCst)
}

pub fn set_search_runner(val: bool) {
    SEARCH_RUNNING.store(val, Ordering::SeqCst);
}

pub fn get_current_search_id() -> u16 {
    CURRENT_ID.load(Ordering::SeqCst)
}

pub fn set_search_id(val: u16) {
    CURRENT_ID.fetch_add(val, Ordering::SeqCst);
}