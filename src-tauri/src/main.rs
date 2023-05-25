#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![feature(once_cell_try)]

mod error;
mod utils;

use utils::{
    is_elevated, add_path, del_path, del_var, get_path, get_vars, set_var, pkg_info, open,
};

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            is_elevated,
            get_path,
            get_vars,
            set_var,
            del_var,
            add_path,
            del_path,
            pkg_info,
            open,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

//const HWND_BROADCAST: HWND = HWND(0xffff);

/*
dbg!("path;dasd;113131;".replacen("dasd;", "", 1));*/

/*let e = CString::new("Environment").unwrap();
let code = unsafe {
    SendMessageTimeoutW(
        HWND_BROADCAST,
        WM_SETTINGCHANGE,
        WPARAM(WM_NULL as usize),
        LPARAM(e.as_ptr() as isize),
        SMTO_ABORTIFHUNG,
        2000,
        None,
    )
};

if code.0 == 0 {
    let err = unsafe { GetLastError() };
    if err != ERROR_TIMEOUT {
        println!("err {}", err.0);
    } else {
        println!("timeout");
    }
}*/
