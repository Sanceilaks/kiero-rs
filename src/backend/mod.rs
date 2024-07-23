#![allow(dead_code)]

use winapi::{
    shared::{ntdef::LPWSTR, windef::HWND},
    um::winuser::{
        CreateWindowExA, DefWindowProcW, DestroyWindow, RegisterClassExA, CS_HREDRAW, CS_VREDRAW, WNDCLASSEXA, WNDCLASSEXW, WS_OVERLAPPED
    },
};

#[cfg(feature = "d3d11")]
pub mod d3d11;
#[cfg(feature = "d3d12")]
pub mod d3d12;
#[cfg(feature = "d3d9")]
pub mod d3d9;
#[cfg(feature = "opengl")]
pub mod gl;
#[cfg(feature = "vulkan")]
pub mod vulkan;

#[cfg(feature = "d3d11")]
pub use d3d11::*;
#[cfg(feature = "d3d12")]
pub use d3d12::*;
#[cfg(feature = "d3d9")]
pub use d3d9::*;
#[cfg(feature = "opengl")]
pub use gl::*;
#[cfg(feature = "vulkan")]
pub use vulkan::*;

pub(crate) struct WindowWrapper {
    pub window: HWND,
}

impl WindowWrapper {
    pub fn create_window() -> Result<WindowWrapper, crate::error::KieroError> {
        unsafe {
            let class_name = std::ffi::CString::new("kiero").unwrap();
    
            let window_clsas = WNDCLASSEXA {
                cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
                style: CS_HREDRAW | CS_VREDRAW,
                lpfnWndProc: Some(DefWindowProcW as _),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: std::ptr::null_mut(),
                hIcon: std::ptr::null_mut(),
                hCursor: std::ptr::null_mut(),
                hbrBackground: std::ptr::null_mut(),
                lpszMenuName: std::ptr::null_mut(),
                lpszClassName: class_name.as_ptr(),
                hIconSm: std::ptr::null_mut(),
            };
    
            if RegisterClassExA(&window_clsas) == 0 {
                return Err(crate::error::KieroError::new(
                    crate::error::ErrorKind::IoError(Box::new(std::io::Error::last_os_error())),
                    "Failed to register window class".to_string(),
                ));
            }
    
            let window = CreateWindowExA(
                0,
                class_name.as_ptr(),
                "kiero".as_ptr() as _,
                WS_OVERLAPPED,
                0,
                0,
                100,
                100,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                window_clsas.hInstance,
                std::ptr::null_mut(),
            );
    
            Ok(WindowWrapper { window })
        }
    }
    
    pub fn destroy_window(window: HWND) {
        unsafe {
            DestroyWindow(window);
        }
    }
}

impl Drop for WindowWrapper {
    fn drop(&mut self) {
        unsafe {
            DestroyWindow(self.window);
        }
    }
}

pub(crate) fn win32_wide_string(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect::<Vec<_>>()
}

pub(crate) fn win32_ansi_string(s: &str) -> Vec<u8> {
    s.as_bytes().iter().map(|c| *c as u8).chain(std::iter::once(0)).collect::<Vec<_>>()
}