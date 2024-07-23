pub mod error;
pub mod backend;

use backend::*;

pub type VMTable = Vec<usize>;

pub enum RenderType {
    D3D9,
    D3D11,
    OpenGL,
    Vulkan
}

pub struct Kiero {
    render_type: RenderType,
    table: VMTable
}

impl Kiero {
    pub fn new() -> Kiero {
        let kind = render_type();
        let table = initialize().unwrap();

        Kiero {
            render_type: kind,
            table
        }
    }
}