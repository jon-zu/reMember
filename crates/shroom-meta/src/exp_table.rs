use core::f64;


pub const MAX_LEVEL: u8 = u8::MAX;

#[derive(Debug)]
pub struct ExpTable(pub [i32; MAX_LEVEL as usize + 1]);

impl ExpTable {
    pub fn build() -> Self {
        let mut table = [0; MAX_LEVEL as usize + 1];
    
        let mut prev = table[0];
        for (i, data) in table.iter_mut().enumerate() {
            *data = match i as u8 {
                0 => 0,
                1 => 15,
                2 => 34,
                3 => 57,
                4 => 92,
                5 => 135,
                6 => 372,
                7 => 560,
                8 => 840,
                9 => 1242,
                10..=14 | 30..=34 | 70..=74 | 120..=124 => prev,
                15..=29 | 35..=39 => (prev as f64 * 1.2 + 0.5) as i32,
                40..=69 => (prev as f64 * 1.08 + 0.5) as i32,
                75..=119 | 125..=159 => (prev as f64 * 1.07 + 0.5) as i32,
                160..=199 => (prev as f64 * 1.06 + 0.5) as i32,
                200..=254 => i32::MAX,
                255 => 0
            };
            prev = *data;
        }
    
        Self(table)
    }

    pub fn get_exp(&self, level: u8) -> i32 {
        self.0[level as usize]
    }
}