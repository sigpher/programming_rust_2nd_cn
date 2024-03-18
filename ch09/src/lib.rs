#[derive(Debug, Default)]
pub struct GrayscaleMap {
    pub pixels: Vec<u8>,
    pub size: (usize, usize),
}

impl GrayscaleMap {
    pub fn new(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
        assert_eq!(pixels.len(), size.0 * size.1);
        GrayscaleMap { pixels, size }
    }
}

pub struct Broom {
    pub name: String,
    pub hieght: u32,
    pub health: u32,
    pub position: (f32, f32, f32),
    pub intent: BroomIntent,
}

pub enum BroomIntent {
    FetchWater,
    DumpWater,
}

impl Broom {
    pub fn chop(b: Broom) -> (Broom, Broom) {
        let mut broom1 = Broom {
            health: b.health / 2,
            ..b
        };
        let mut broom2 = Broom {
            name: broom1.name.clone(),
            ..broom1
        };
        broom1.name.push_str(" I");
        broom2.name.push_str(" II");

        (broom1, broom2)
    }
}
