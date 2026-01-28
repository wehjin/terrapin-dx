#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ownership {
    pub level: OwnerLevel,
    pub excess_shares: usize,
    pub deficit_shares: usize,
}

impl std::fmt::Display for Ownership {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}⧸{} → {}",
            self.level, self.excess_shares, self.deficit_shares
        )
    }
}

impl Ownership {
    pub fn new(quantity: f64, outstanding: usize) -> Self {
        let level = OwnerLevel::new(quantity / outstanding as f64);
        let floor_shares = level.floor() * outstanding as f64;
        let excess_shares = (quantity - floor_shares).floor() as usize;
        let ceiling_shares = level.ceiling() * outstanding as f64;
        let deficit_shares = (ceiling_shares - quantity).ceil() as usize;
        Self {
            level,
            excess_shares,
            deficit_shares,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum OwnerLevel {
    S,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

impl std::fmt::Display for OwnerLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let symbol = match self {
            OwnerLevel::S => "S",
            OwnerLevel::A => "A",
            OwnerLevel::B => "B",
            OwnerLevel::C => "C",
            OwnerLevel::D => "D",
            OwnerLevel::E => "E",
            OwnerLevel::F => "F",
            OwnerLevel::G => "G",
        };
        write!(f, "{}", symbol)
    }
}
impl OwnerLevel {
    const F_FLOOR: f64 = 0.00000001;
    const E_FLOOR: f64 = 0.0000001;
    const D_FLOOR: f64 = 0.000001;
    const C_FLOOR: f64 = 0.00001;
    const B_FLOOR: f64 = 0.0001;
    const A_FLOOR: f64 = 0.001;
    const S_FLOOR: f64 = 0.01;
    pub fn new(fraction: f64) -> Self {
        match fraction {
            fraction if fraction < Self::F_FLOOR => OwnerLevel::G,
            Self::F_FLOOR..Self::E_FLOOR => OwnerLevel::F,
            Self::E_FLOOR..Self::D_FLOOR => OwnerLevel::E,
            Self::D_FLOOR..Self::C_FLOOR => OwnerLevel::D,
            Self::C_FLOOR..Self::B_FLOOR => OwnerLevel::C,
            Self::B_FLOOR..Self::A_FLOOR => OwnerLevel::B,
            Self::A_FLOOR..Self::S_FLOOR => OwnerLevel::A,
            _ => OwnerLevel::S,
        }
    }
    fn ceiling(&self) -> f64 {
        match self {
            OwnerLevel::S => 1.0,
            OwnerLevel::A => Self::S_FLOOR,
            OwnerLevel::B => Self::A_FLOOR,
            OwnerLevel::C => Self::B_FLOOR,
            OwnerLevel::D => Self::C_FLOOR,
            OwnerLevel::E => Self::D_FLOOR,
            OwnerLevel::F => Self::E_FLOOR,
            OwnerLevel::G => Self::F_FLOOR,
        }
    }

    fn floor(&self) -> f64 {
        match self {
            OwnerLevel::S => Self::S_FLOOR,
            OwnerLevel::A => Self::A_FLOOR,
            OwnerLevel::B => Self::B_FLOOR,
            OwnerLevel::C => Self::C_FLOOR,
            OwnerLevel::D => Self::D_FLOOR,
            OwnerLevel::E => Self::E_FLOOR,
            OwnerLevel::F => Self::F_FLOOR,
            OwnerLevel::G => 0.0,
        }
    }
}
