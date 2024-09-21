// mod util;

// use once_cell::sync::Lazy;
use std::ffi::c_void;


use windows::Win32::Foundation::HINSTANCE;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut c_void) {
    if reason == DLL_PROCESS_ATTACH {
        // Lazy::force(&DIRECTINPUT8CREATE);
        // Lazy::force(&XINPUTGETSTATE);

        // thread::spawn(move || {
        //     if util::get_dll_path()
        //         .and_then(|path| {
        //             path.file_name()
        //                 .map(|s| s.to_string_lossy().to_lowercase() == "dinput8.dll")
        //         })
        //         .unwrap_or(false)
        //     {
        //         if await_rshift() {
        //             start_practice_tool(hmodule)
        //         }
        //     } else {
        //         start_practice_tool(hmodule)
        //     }
        // });
    }
}
