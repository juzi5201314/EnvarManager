use std::ptr::null_mut;
use std::sync::OnceLock;

use serde::Serialize;
use tauri::command;
use winapi::um::processthreadsapi::{GetCurrentProcess, OpenProcessToken};
use winapi::um::securitybaseapi::GetTokenInformation;
use winapi::um::winnt::{TokenElevation, HANDLE, TOKEN_ELEVATION, TOKEN_QUERY};
use winreg::enums::{HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE};
use winreg::types::FromRegValue;
use winreg::RegKey;

use crate::error::Error;

static IS_ELEVATED: OnceLock<bool> = OnceLock::new();

#[derive(Serialize)]
pub struct Var {
    name: String,
    val: String,
    in_path: bool,
}

#[derive(Serialize)]
pub struct PkgInfo {
    version: &'static str,
    commit_hash: &'static str,
    repo: &'static str,
}

#[command]
pub fn is_elevated() -> Result<bool, Error> {
    IS_ELEVATED.get_or_try_init(|| {
        unsafe {
            let mut handle: HANDLE = null_mut();
            if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut handle) == 0 {
                Err(std::io::Error::last_os_error())?;
            }
            let mut elevation = TOKEN_ELEVATION::default();
            let size = std::mem::size_of::<TOKEN_ELEVATION>() as u32;
            let mut ret_size = size;
            if GetTokenInformation(
                handle,
                TokenElevation,
                &mut elevation as *mut _ as *mut _,
                size,
                &mut ret_size,
            ) == 0
            {
                Err(std::io::Error::last_os_error())?;
            }
            Ok(elevation.TokenIsElevated != 0)
        }
    }).copied()
}

#[command]
pub fn set_var(key: &str, val: &str, is_elevated: bool) -> Result<(), Error> {
    reg_key(is_elevated)?.set_value(key, &val).map_err(Into::into)
}

#[command]
pub fn del_var(key: &str, is_elevated: bool) -> Result<(), Error> {
    reg_key(is_elevated)?.delete_value(key).map_err(Into::into)
}

#[command]
pub fn add_path(key: &str, is_elevated: bool) -> Result<(), Error> {
    let reg = reg_key(is_elevated)?;
    let mut path = reg.get_value::<String, _>("Path")?;
    if !path.ends_with(";") {
        path.push(';');
    }
    reg.set_value(
        "Path",
        &format!("{}%{key}%;", path),
    )
    .map_err(Into::into)
}

#[command]
pub fn del_path(key: &str, is_elevated: bool) -> Result<(), Error> {
    let reg = reg_key(is_elevated)?;
    let path = reg.get_value::<String, _>("Path")?;
    let replaced = if path.starts_with(&format!("%{key}%;")) {
        path.trim_start_matches(&format!("%{key}%;")).to_owned()
    } else {
        path.replace(&format!(";%{key}%;"), ";")
    };
    reg.set_value(
        "Path",
        &replaced,
    )
    .map_err(Into::into)
}

#[command]
pub fn get_path(is_elevated: bool) -> Result<Vec<String>, Error> {
    Ok(reg_key(is_elevated)?
        .get_value::<String, _>("Path")?
        .split(";")
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

#[command]
pub fn get_vars(is_elevated: bool) -> Result<Vec<Var>, Error> {
    let path = get_path(is_elevated)?;
    reg_key(is_elevated)?
        .enum_values()
        .filter(|res| {
            !res.as_ref()
                .map_err(|_| ())
                .and_then(|(k, _)| (k == "Path").then(|| ()).ok_or(()))
                .is_ok()
        })
        .map(|res| {
            let (k, v) = res?;
            Ok(Var {
                in_path: path.contains(&format!("%{}%", &k)),
                name: k,
                val: String::from_reg_value(&v)?,
            })
        })
        .collect()
}

#[command]
pub fn pkg_info() -> PkgInfo {
    PkgInfo {
        version: env!("__VERSION__"),
        commit_hash: env!("GIT_COMMIT_HASH_SHORT"),
        repo: env!("__REPO__"),
    }
}

#[command]
pub fn open(url: &str) -> Result<(), Error> {
    open::that(url).map_err(Into::into)
}

fn reg_key(is_elevated: bool) -> Result<RegKey, Error> {
    if is_elevated {
        Ok(RegKey::predef(HKEY_LOCAL_MACHINE).create_subkey("System\\CurrentControlSet\\Control\\Session Manager\\Environment")?.0)
    } else {
        Ok(RegKey::predef(HKEY_CURRENT_USER).create_subkey("Environment")?.0)
    }
}
