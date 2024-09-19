mod inject;
mod pointers;
mod base_addresses;

fn main() {
    let game_process = inject::get_process_by_name("bhd.exe");
    if let Ok(process) = game_process {
        println!("Got Game Handle {}", process.0);
    } else {
        println!("Error getting handle");
    }
}

// fn set_pistol_ammo_to_max() {
//     let pointer_chains = pointers::PointerChain::new();
// }

// start a GUI

// after taking the game stuff, being able to keep doing shit in there
