mod inject;

use std::io;

use hudhook::tracing::trace;
use tracing_subscriber::filter::LevelFilter;

use windows::core::PCSTR;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{
    MessageBoxA, IDYES, MB_ICONERROR, MB_ICONINFORMATION, MB_OK, MB_YESNO,
};

fn err_to_string<T: std::fmt::Display>(e: T) -> String {
    format!("Error: {}", e)
}

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::TRACE)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .with_thread_names(true)
        .init();

    if let Err(e) = perform_injection() {
        let error_msg = format!("{}\0", e);
        unsafe {
            MessageBoxA(
                HWND(0),
                PCSTR(error_msg.as_str().as_ptr()),
                PCSTR("Error\0".as_ptr()),
                MB_OK | MB_ICONERROR,
            );
        }
    }
    // io::stdin().read_line(&mut String::new()).unwrap();
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
    trace!("Injecting {:?}", dll_path);

    inject::Process::get_process_by_name("DarkSoulsRemastered.exe") //  bhd.exe
        .map_err(|e| format!("Could not find process: {e:?}"))?
        .inject(dll_path)
        .map_err(|e| format!("Could not inject DLL: {e:?}"))?;
    Ok(())
}
