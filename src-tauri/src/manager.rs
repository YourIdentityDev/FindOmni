use crate::search;

#[tauri::command]
pub async fn task_manager(action: String, parameter: String) -> String {
    let action: &str = &action;
    match action {
        "search" => {
            search_manager(&parameter);
            return "Rust Backend: Started search successfully".to_string();
        },
        "stop" => {
            search_manager("close");
            return "Rust Backend: Try to end the search".to_string();
        },
        "full-only" => {
            search::start_search(&parameter, false, true);
            return "Rust Backend: Started search successfully".to_string();
        },
        _ => {return "Rust Backend: ERROR action not found".to_string();}
    }
}

fn search_manager(parameter: &str) {
    //TODO: Make this methode read your settings and based on that choose the correct methode in search.rs
    if parameter == "stop" && search::get_search_runner() {
        search::set_search_runner(false);
        search::set_search_id(0)
    }
    search::set_search_runner(true);
    search::start_search(parameter, true, true);
}