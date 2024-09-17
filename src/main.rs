use std::mem;

use windows::core::{Error, Result, HRESULT, HSTRING};
use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE};
use windows::Win32::System::Diagnostics::ToolHelp::{
    CreateToolhelp32Snapshot, Process32FirstW, Process32NextW, PROCESSENTRY32W, TH32CS_SNAPPROCESS,
};
use windows::Win32::System::Threading::{OpenProcess, PROCESS_ALL_ACCESS};

fn main() {
    let game_process = get_process_by_name("bhd.exe");
    if let Ok(process) = game_process {
        println!("Got Game Handle {}", process.0);
    } else {
        println!("Error getting handle");
    }
}

// Look for the game
fn get_process_by_name(name: &str) -> Result<HANDLE> {
    unsafe { get_process_by_name64(name) }
}

unsafe fn get_process_by_name64(name_str: &str) -> Result<HANDLE> {
    let name = HSTRING::from(name_str);

    let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
    let mut process_entry32 = PROCESSENTRY32W {
        dwSize: mem::size_of::<PROCESSENTRY32W>() as u32,
        ..Default::default()
    };

    if Process32FirstW(snapshot, &mut process_entry32).is_err() {
        CloseHandle(snapshot)?;
        return Err(Error::from_win32());
    }

    let pid = loop {
        let zero_idx = process_entry32
            .szExeFile
            .iter()
            .position(|&x| x == 0)
            .unwrap_or(process_entry32.szExeFile.len());
        let process_name = HSTRING::from_wide(&process_entry32.szExeFile[..zero_idx])?;

        if name == process_name {
            break Ok(process_entry32.th32ProcessID);
        }

        if Process32NextW(snapshot, &mut process_entry32).is_err() {
            CloseHandle(snapshot)?;
            break Err(Error::from_hresult(HRESULT(-1)));
        }
    }?;

    CloseHandle(snapshot)?;

    OpenProcess(PROCESS_ALL_ACCESS, BOOL(0), pid)
}

// start a GUI

// after taking the game stuff, being able to keep doing shit in there
