pub struct Random {
    a: u64,
    b: u64,
    m: u64,
    x: u64,
}

impl Random {
    pub fn new(init: u64) -> Self {
        Random {
            a: 214013,
            b: 2531011,
            m: 2147483648,
            x: init,
        }
    }

    pub fn gen_u64(&mut self) -> u64 {
        self.x = (self.a * self.x + self.b) % self.m;
        self.x
    }
    pub fn gen_f32(&mut self) -> f32 {
        self.gen_u64() as f32 / self.m as f32
    }
    // pub fn gen_f32_range(&mut self, min: f32, max: f32) -> f32 {
    //     self.gen_f32() * (max - min) + min
    // }
    pub fn chance(&mut self, chance: f32) -> bool {
        self.gen_f32() <= chance
    }
}
