use crate::r#impl::Impl;

pub struct TileOutAction {
    imp: Box<dyn Impl>,
    shape: (i32, i32),
}

impl TileOutAction {
    pub fn call(&self) {
        self.imp.tile_out(self.shape)
    }
}
