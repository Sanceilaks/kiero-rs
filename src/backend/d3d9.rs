use winapi::{
    shared::{
        d3d9::{
            IDirect3D9,
            D3DADAPTER_DEFAULT,
            D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
            D3DCREATE_SOFTWARE_VERTEXPROCESSING,
            D3D_SDK_VERSION
        }, 
        d3d9types::{
            D3DDEVTYPE_NULLREF, 
            D3DFMT_UNKNOWN, 
            D3DMULTISAMPLE_NONE, 
            D3DMULTISAMPLE_TYPE,
            D3DPRESENT_PARAMETERS, 
            D3DSWAPEFFECT_DISCARD
        }
    },
    um::libloaderapi::{
        GetModuleHandleA, 
        GetModuleHandleW, 
        GetProcAddress
    }
};
use crate::win32_wide_string;

use super::{win32_ansi_string, WindowWrapper};

pub fn render_type() -> crate::RenderType {
    crate::RenderType::D3D9
}

type Direct3DCreate9Fn = unsafe extern "stdcall" fn(u32) -> *mut std::ffi::c_void;

pub fn initialize() -> Result<crate::VMTable, crate::error::KieroError> {
    let window = WindowWrapper::create_window()?;

    let d3d9_module = unsafe { GetModuleHandleW(win32_wide_string("d3d9.dll").as_ptr()) };
    if d3d9_module.is_null() {
        return Err(crate::error::KieroError::new(
            crate::error::ErrorKind::ModuleNotFound("d3d9.dll".to_string()),
            std::io::Error::last_os_error().to_string(),
        ));
    }

    let d3d9_create = unsafe {
        let smb = GetProcAddress(d3d9_module, win32_ansi_string("Direct3DCreate9").as_ptr() as _);
        if smb.is_null() {
            return Err(crate::error::KieroError::new(
                crate::error::ErrorKind::NotSupported,
                "Direct3DCreate9 not found".to_string(),
            ));
        }

        std::mem::transmute::<_, Direct3DCreate9Fn>(smb)
    };

    let direct3d: *const IDirect3D9 = unsafe { d3d9_create(D3D_SDK_VERSION) as _  };

    if direct3d.is_null() {
        return Err(crate::error::KieroError::new(
            crate::error::ErrorKind::NotSupported,
            "Direct3DCreate9 failed".to_string(),
        ));
    }

    let mut params = D3DPRESENT_PARAMETERS {
        BackBufferWidth: 0,
        BackBufferHeight: 0,
        BackBufferFormat: D3DFMT_UNKNOWN,
        BackBufferCount: 0,
        MultiSampleType: D3DMULTISAMPLE_NONE,
        MultiSampleQuality: 0,
        SwapEffect: D3DSWAPEFFECT_DISCARD,
        Windowed: 0,
        EnableAutoDepthStencil: 0,
        AutoDepthStencilFormat: D3DFMT_UNKNOWN,
        Flags: 0,
        FullScreen_RefreshRateInHz: 0,
        PresentationInterval: 0,
        hDeviceWindow: window.window,
    };

    let mut device = std::ptr::null_mut();
    let result = unsafe {
        (*direct3d).CreateDevice(
            D3DADAPTER_DEFAULT,
            D3DDEVTYPE_NULLREF,
            window.window,
            D3DCREATE_SOFTWARE_VERTEXPROCESSING | D3DCREATE_DISABLE_DRIVER_MANAGEMENT,
            &mut params,
            &mut device,
        )
    };

    if !device.is_null() && result < 0 {
        unsafe { (*direct3d).Release() };

        return Err(crate::error::KieroError::new(
            crate::error::ErrorKind::NotSupported,
            "CreateDevice failed".to_string(),
        ));
    }

    let mut table = Vec::new();
    for i in 0..119 {
        table.push(unsafe { (*device).lpVtbl.offset(i) as usize });
    }

    Ok(table)
}


#[cfg(test)]
mod tests {
    use winapi::um::libloaderapi::{LoadLibraryA, LoadLibraryW};

    use crate::win32_wide_string;

    #[test]
    fn test_initialize() {
        unsafe {
            LoadLibraryW(win32_wide_string("d3d9.dll").as_ptr());
        }

        let table = super::initialize().unwrap();

        assert!(!table.is_empty());
        assert!(table.len() == 119);

        for i in 0..119 {
            assert!(!(table[i] as *mut std::ffi::c_void).is_null());
        }
    }
}
