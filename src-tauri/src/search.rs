use rayon::prelude::*;
use walkdir::WalkDir;
use std::thread;
use std::env;
use std::path::PathBuf;
use std::time::Duration;
use std::sync::{atomic::{AtomicUsize, AtomicBool, Ordering}};

pub static SEARCH_RUNNING: AtomicBool = AtomicBool::new(false); //When false stop all threads
pub static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);   //To check if the current thread the newest one is | If not stop the search

pub fn start_search(query: &str) {
    set_search_id(1);
    let my_id = get_current_search_id();
    if let Ok(user_profile) = env::var("USERPROFILE") {
        println!("Using user profile: {}", user_profile);
        let smart_paths = vec![
            PathBuf::from(user_profile.clone() + r"\Desktop"),
            PathBuf::from(user_profile.clone() + r"\Documents"),
            PathBuf::from(user_profile.clone() + r"\Downloads"),
            PathBuf::from(user_profile.clone() + r"\Pictures"),
            PathBuf::from(user_profile.clone() + r"\Pictures"),
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

        if my_id < get_current_search_id() && !get_search_runner() {
            println!("Closing thread with ID: {}", my_id);
            return; //Stop the threads
        } else {
            let additional_handle = thread::spawn(move || {
                search_files(&PathBuf::from(r"C:\"), &query, my_id);
            });
            additional_handle.join().unwrap();
        }
    } else {
        println!("No USER profile");
    }
}

fn search_files(base_path: &PathBuf, query: &str, my_id: usize) {
    println!("My ID: {}", my_id);
    if base_path == &PathBuf::from(r"C:\") {
        println!("Full search for {}", query);
        thread::sleep(Duration::new(9, 0));
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
    return SEARCH_RUNNING.load(Ordering::SeqCst);
}

pub fn set_search_runner(val: bool) {
    SEARCH_RUNNING.store(val, Ordering::SeqCst);
}

pub fn get_current_search_id() -> usize {
    return CURRENT_ID.load(Ordering::SeqCst);
}

pub fn set_search_id(val: usize) {
    CURRENT_ID.fetch_add(val, Ordering::SeqCst);
}