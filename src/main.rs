use windows::Win32::System::Diagnostics::ToolHelp::{CreateToolhelp32Snapshot, TH32CS_SNAPPROCESS};


fn main() {
    println!("Hello, world!");
}

fn look_for_the_game() {
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
    }
    
}



// Look for the game


// start a GUI


// after taking the game stuff, being able to keep doing shit in there


