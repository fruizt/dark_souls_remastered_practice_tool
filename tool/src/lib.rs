// mod util;
mod tool;

use tool::Tool;

use once_cell::sync::Lazy;
use std::ffi::c_void;
use std::thread;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};

use hudhook::hooks::dx9::ImguiDx9Hooks;
use hudhook::tracing::{error, trace};
use hudhook::{eject, Hudhook};

use windows::core::{s, w, GUID, HRESULT, PCSTR, PCWSTR};
use windows::Win32::Foundation::{ERROR_SUCCESS, HINSTANCE, HWND, MAX_PATH};
use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
use windows::Win32::System::SystemInformation::GetSystemDirectoryW;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

type FDirectInput8Create = unsafe extern "stdcall" fn(
    hinst: HINSTANCE,
    dwversion: u32,
    riidltf: *const GUID,
    ppvout: *mut *mut c_void,
    punkouter: HINSTANCE,
) -> HRESULT;

static DIRECTINPUT8CREATE: Lazy<FDirectInput8Create> = Lazy::new(|| unsafe {
    let mut dinput8_path = [0u16; MAX_PATH as usize];
    let count = GetSystemDirectoryW(Some(&mut dinput8_path)) as usize;

    // If count == 0, this will be fun
    std::ptr::copy_nonoverlapping(
        w!("\\dinput8.dll").0,
        dinput8_path[count..].as_mut_ptr(),
        12,
    );

    let dinput8 = LoadLibraryW(PCWSTR(dinput8_path.as_ptr())).unwrap();
    let directinput8create = std::mem::transmute(GetProcAddress(dinput8, s!("DirectInput8Create")));

    directinput8create
});

fn start_tool(hmodule: HINSTANCE) {
    let tool = Tool::new();

    if let Err(e) = Hudhook::builder()
        .with::<ImguiDx9Hooks>(tool)
        .with_hmodule(hmodule)
        .build()
        .apply()
    {
        error!("Couldn't apply hooks: {e:?}");
        eject();
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "stdcall" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut c_void) {
    if reason == DLL_PROCESS_ATTACH {
        trace!("DllMain()");

        Lazy::force(&DIRECTINPUT8CREATE);
        thread::spawn(move || {
            MessageBoxA(
                HWND(0),
                PCSTR("DLL Injected!".as_ptr()),
                PCSTR("DLL Injection".as_ptr()),
                MB_OK,
            );
            start_tool(hmodule)
        });
    }
}
