pub enum DType {
    Uint8,
    Uint32,
}

impl DType {
    pub fn size(&self) -> i32 {
        match self {
            DType::Uint8 => 1,
            DType::Uint32 => 4,
        }
    }
    pub fn c_type(&self) -> &'static str {
        match self {
            DType::Uint8 => "uint8_t",
            DType::Uint32 => "uint32_t",
        }
    }
    pub fn int_fmt_macro(&self) -> &'static str {
        match self {
            DType::Uint8 => "PRIu8",
            DType::Uint32 => "PRIu32",
        }
    }
    pub fn short_name(&self) -> &'static str {
        match self {
            DType::Uint8 => "u8",
            DType::Uint32 => "u32",
        }
    }
}
