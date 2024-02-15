extern crate winapi;
extern crate winreg;

use std::ptr;
use std::env;
use winapi::um::winnt::{KEY_SET_VALUE, KEY_WOW64_64KEY, KEY_WRITE, REG_OPTION_NON_VOLATILE, REG_SZ};
use winapi::um::winreg::{RegCreateKeyExW, RegSetValueExW, HKEY_CLASSES_ROOT, RegCloseKey, RegQueryValueExW};
use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use winapi::shared::minwindef::BYTE;

pub fn main() {
    add_context_menu();
}

pub fn add_context_menu() {
    match env::current_exe()  {
        Ok(mut exe_path) => {
            exe_path.pop();
            exe_path.push("nmuidi.exe");

            let command_string = format!(r#"{} %1"#, exe_path.display());
            create_key(r"Directory\shell\FastDelete", "Fast Delete").unwrap();
            create_key(r"Directory\shell\FastDelete\command", &command_string).unwrap();
        },
        Err(_) => {
            let default_command = read_default_command(r"Directory\shell\FastDelete\command").unwrap();
            let command_string = format!(r#"{} %1"#, default_command);
            create_key(r"Directory\shell\FastDelete", "Fast Delete").unwrap();
            create_key(r"Directory\shell\FastDelete\command", &command_string).unwrap();
            println!("Using default command: {}", default_command);
        }
    }
}

fn create_key(key_path: &str, value: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut hkey = ptr::null_mut();
    let result = unsafe {
        RegCreateKeyExW(
            HKEY_CLASSES_ROOT,
            to_wstring(key_path).as_ptr(),
            0,
            ptr::null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE | KEY_WRITE | KEY_WOW64_64KEY,
            ptr::null_mut(),
            &mut hkey,
            ptr::null_mut(),
        )
    };

    if result == 0 {
        let wide_value = to_wstring(value);
        let wide_value_ptr = wide_value.as_ptr() as *const BYTE;

        let result = unsafe {
            RegSetValueExW(
                hkey,
                to_wstring("").as_ptr(),
                0,
                REG_SZ,
                wide_value_ptr,
                (wide_value.len() * std::mem::size_of::<u16>()) as u32,
            )
        };

        if result == 0 {
            unsafe { RegCloseKey(hkey); }
            Ok(())
        } else {
            Err(format!("Failed to set registry value, error code {}", result).into())
        }
    } else {
        Err(format!("Failed to open registry key, error code {}", result).into())
    }
}

fn to_wstring(str : &str) -> Vec<u16> {
    OsStr::new(str).encode_wide().chain(Some(0)).collect()
}

fn read_default_command(key_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut hkey = ptr::null_mut();
    let result = unsafe {
        RegCreateKeyExW(
            HKEY_CLASSES_ROOT,
            to_wstring(key_path).as_ptr(),
            0,
            ptr::null_mut(),
            REG_OPTION_NON_VOLATILE,
            KEY_SET_VALUE | KEY_WRITE | KEY_WOW64_64KEY,
            ptr::null_mut(),
            &mut hkey,
            ptr::null_mut(),
        )
    };

    if result == 0 {
        let mut buffer: Vec<u16> = vec![0; 256]; // Adjust buffer size as needed
        let mut buffer_size = (buffer.len() * std::mem::size_of::<u16>()) as u32;
        let result = unsafe {
            RegQueryValueExW(
                hkey,
                ptr::null(),
                ptr::null_mut(),
                ptr::null_mut(),
                buffer.as_mut_ptr() as *mut BYTE,
                &mut buffer_size,
            )
        };

        if result == 0 {
            let value = String::from_utf16_lossy(&buffer[..(buffer_size / 2) as usize]);
            unsafe { RegCloseKey(hkey); }
            Ok(value)
        } else {
            unsafe { RegCloseKey(hkey); }
            Err(format!("Failed to read registry value, error code {}", result).into())
        }
    } else {
        Err(format!("Failed to open registry key, error code {}", result).into())
    }
}