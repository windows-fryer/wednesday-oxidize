pub trait Precision32 {
    fn to_f64_decomposition(self) -> f64;
    fn to_f64_bit_rearrangement(self) -> f64;
}

pub trait Precision64 {
    fn to_f32_bit_rearrangement(self) -> f32;
}

impl Precision32 for f32 {
    fn to_f64_decomposition(self) -> f64 {
        let f32_bits = self.to_bits();

        let f32_sign = (f32_bits & 0x8000_0000) >> 31;
        let f32_exponent = (f32_bits & 0x7F80_0000) >> 23;
        let f32_decimals = f32_bits & 0x7F_FFFF;

        let mut f32_gate =  1 << 22;

        let f64_exponent = (f32_exponent as i32) - 127;
        let mut f64_decimal = 1f64;
        let f64_sign = if f32_sign == 1 { -1f64 } else { 1f64 };

        for i in 1..=23 {
            f64_decimal += ((f32_decimals & f32_gate) >> (23 - i)) as f64 * 2f64.powi(-i);
            f32_gate >>= 1;
        }

        f64_sign * f64_decimal * 2f64.powi(f64_exponent)
    }

    fn to_f64_bit_rearrangement(self) -> f64 {
        let f32_bits = self.to_bits();

        let f32_mantissa = f32_bits & 0x007F_FFFF;
        let f32_exponent = (f32_bits & 0x7F80_0000) >> 23;
        let f32_sign = (f32_bits & 0x8000_0000) >> 31;

        let f64_sign = (f32_sign as u64) << 63;
        let f64_exponent = ((f32_exponent as u64 + 0x380) << 52) & 0x7FF0_0000_0000_0000;
        let f64_mantissa = (f32_mantissa as u64) << 29;

        f64::from_bits(f64_sign | f64_exponent | f64_mantissa)
    }
}

impl Precision64 for f64 {
    fn to_f32_bit_rearrangement(self) -> f32 {
        let f64_bits = self.to_bits();

        let f64_mantissa = f64_bits & 0x000F_FFFF_FFFF_FFFF;
        let f64_exponent = (f64_bits & 0x7FF0_0000_0000_0000) >> 52;
        let f64_sign = (f64_bits & 0x8000_0000_0000_0000) >> 63;

        let f32_sign = (f64_sign as u32) << 31;
        let f32_exponent = ((f64_exponent as u32) - 0x380) << 23;
        let f32_mantissa = (f64_mantissa >> 29) as u32;

        f32::from_bits(f32_sign | f32_exponent | f32_mantissa)
    }
}