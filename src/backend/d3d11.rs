use winapi::{
    shared::{
        dxgi::{
            IDXGIAdapter, 
            IDXGISwapChain, 
            DXGI_SWAP_CHAIN_DESC, 
            DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH, 
            DXGI_SWAP_EFFECT_DISCARD
        }, 
        dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM, 
        dxgitype::{
            DXGI_MODE_DESC, 
            DXGI_MODE_SCALING_UNSPECIFIED, 
            DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED, 
            DXGI_RATIONAL, 
            DXGI_SAMPLE_DESC, 
            DXGI_USAGE_RENDER_TARGET_OUTPUT
        }, 
        minwindef::HINSTANCE__
    }, 
    um::{
        d3d11::{
            ID3D11Device, 
            ID3D11DeviceContext, 
            D3D11_SDK_VERSION
        }, 
        d3dcommon::{
            D3D_DRIVER_TYPE_HARDWARE, 
            D3D_FEATURE_LEVEL, 
            D3D_FEATURE_LEVEL_10_1, 
            D3D_FEATURE_LEVEL_11_0
        }, 
        libloaderapi::{
            GetModuleHandleW, 
            GetProcAddress
        }
    }
};
use crate::win32_wide_string;

use super::{win32_ansi_string, WindowWrapper};

pub fn render_type() -> crate::RenderType {
    crate::RenderType::D3D11
}

type Direct3DCreate11Fn = unsafe extern "stdcall" fn(
    *mut IDXGIAdapter,
    u32,
    *mut HINSTANCE__,
    u32,
    *const u32,
    u32,
    u32,
    *const DXGI_SWAP_CHAIN_DESC,
    *mut *mut IDXGISwapChain,
    *mut *mut ID3D11Device,
    *mut u32,
    *mut *mut ID3D11DeviceContext
) -> i32;

pub fn initialize() -> Result<crate::VMTable, crate::error::KieroError> {
    let window = WindowWrapper::create_window()?;

    let d3d11_module = unsafe { GetModuleHandleW(win32_wide_string("d3d11.dll").as_ptr()) };
    if d3d11_module.is_null() {
        return Err(crate::error::KieroError::new(
            crate::error::ErrorKind::ModuleNotFound("d3d11.dll".to_string()),
            std::io::Error::last_os_error().to_string(),
        ));
    }

    let d3d11_create = unsafe { 
        let smb = GetProcAddress(
            d3d11_module,
            win32_ansi_string("D3D11CreateDeviceAndSwapChain").as_ptr() as _
        );
        if smb.is_null() {
            return Err(crate::error::KieroError::new(
                crate::error::ErrorKind::NotSupported,
                "D3D11CreateDeviceAndSwapChain not found".to_string(),
            ));
        }

        std::mem::transmute::<_, Direct3DCreate11Fn>(smb)
    };
    
    let mut feature_level: D3D_FEATURE_LEVEL = D3D_FEATURE_LEVEL_11_0;
    let feature_levels: Vec<D3D_FEATURE_LEVEL> = vec![D3D_FEATURE_LEVEL_10_1, D3D_FEATURE_LEVEL_11_0];

    let refresh_rate = DXGI_RATIONAL {
        Numerator: 60,
        Denominator: 1
    };

    let buffer_desc = DXGI_MODE_DESC {
        Width: 100,
        Height: 100,
        RefreshRate: refresh_rate,
        Format: DXGI_FORMAT_R8G8B8A8_UNORM,
        ScanlineOrdering: DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
        Scaling: DXGI_MODE_SCALING_UNSPECIFIED
    };

    let sample_desc = DXGI_SAMPLE_DESC {
        Count: 1,
        Quality: 0
    };

    let swap_chain_desc = DXGI_SWAP_CHAIN_DESC {
        BufferDesc: buffer_desc,
        SampleDesc: sample_desc,
        BufferUsage: DXGI_USAGE_RENDER_TARGET_OUTPUT,
        BufferCount: 1,
        OutputWindow: window.window,
        Windowed: 1,
        SwapEffect: DXGI_SWAP_EFFECT_DISCARD,
        Flags: DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH
    };

    let mut swap_chain = std::ptr::null_mut();
    let mut device = std::ptr::null_mut();
    let mut context = std::ptr::null_mut();

    let result = unsafe { 
        d3d11_create(
            std::ptr::null_mut(),
            D3D_DRIVER_TYPE_HARDWARE,
            std::ptr::null_mut(),
            0,
            feature_levels.as_ptr(),
            2,
            D3D11_SDK_VERSION,
            &swap_chain_desc,
            &mut swap_chain,
            &mut device,
            &mut feature_level,
            &mut context
        )
    };
    if !swap_chain.is_null() && !device.is_null() && context.is_null() && result < 0 {
        return Err(crate::error::KieroError::new(
            crate::error::ErrorKind::NotSupported, 
            "D3D11CreateDeviceAndSwapChain failed".to_string()
        ));
    }

    let mut table = Vec::new();
    for i in 0..18 {
        table.push(unsafe { (*swap_chain).lpVtbl.offset(i) as usize })
    }

    for i in 0..43 {
        table.push(unsafe { (*device).lpVtbl.offset(i) as usize })
    }

    for i in 0..144 {
        table.push(unsafe { (*context).lpVtbl.offset(i) as usize })
    }

    Ok(table)
}

#[cfg(test)]
mod tests {
    use winapi::um::libloaderapi::LoadLibraryW;

    use crate::win32_wide_string;

    #[test]
    fn test_initialize() {
        unsafe {
            LoadLibraryW(win32_wide_string("d3d11.dll").as_ptr());
        }

        let table = super::initialize().unwrap();

        assert!(!table.is_empty());
        assert!(table.len() == 205);

        for i in 0..205 {
            assert!(!(table[i] as *mut std::ffi::c_void).is_null());
        }
    }
}
