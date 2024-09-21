use std::io;

mod base_addresses;
mod inject;
mod pointers;

fn err_to_string<T: std::fmt::Display>(e: T) -> String {
    format!("Error: {}", e)
}

fn main() {
    let injection_result = perform_injection();
    if let Err(e) = injection_result {
        println!("Error While injecting {}", e);
    } else {
        println!("Injection Performed");
    }
    io::stdin().read_line(&mut String::new()).unwrap();
}

fn perform_injection() -> Result<(), String> {
    let mut dll_path = std::env::current_exe().unwrap();
    dll_path.pop();
    dll_path.push("resident_evil_1_trainer_tool.dll");

    if !dll_path.exists() {
        dll_path.pop();
        dll_path.push("resident_evil_1_trainer_tool");
        dll_path.set_extension("dll");
    }

    let dll_path = dll_path.canonicalize().map_err(err_to_string)?;

    inject::Process::get_process_by_name("bhd.exe")
        .map_err(|e| format!("Could not find process: {e:?}"))?
        .inject(dll_path)
        .map_err(|e| format!("Could not inject DLL: {e:?}"))?;
    Ok(())
}
