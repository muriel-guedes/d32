pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
impl Color {
    pub fn to_u32(&self) -> u32 {
        ((self.r as u32)<<24) + ((self.g as u32)<<16) + ((self.b as u32)<<8) + (self.a as u32)
    }
}
impl From<(u8, u8, u8)> for Color {
    fn from(v: (u8, u8, u8)) -> Self {
        Self {
            r: v.0,
            g: v.1,
            b: v.2,
            a: 255
        }
    }
}
impl From<(u8, u8, u8, u8)> for Color {
    fn from(v: (u8, u8, u8, u8)) -> Self {
        Self {
            r: v.0,
            g: v.1,
            b: v.2,
            a: v.3
        }
    }
}