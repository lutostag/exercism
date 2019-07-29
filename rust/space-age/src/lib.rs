#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration { seconds }
    }
}

pub trait Planet {
    const SOLAR_YEAR: Duration;
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / Self::SOLAR_YEAR.seconds as f64
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const SOLAR_YEAR: Duration = Duration { seconds: 7600544 };
}
impl Planet for Venus {
    const SOLAR_YEAR: Duration = Duration { seconds: 19414149 };
}
impl Planet for Earth {
    const SOLAR_YEAR: Duration = Duration { seconds: 31557600 };
}
impl Planet for Mars {
    const SOLAR_YEAR: Duration = Duration { seconds: 59354032 };
}
impl Planet for Jupiter {
    const SOLAR_YEAR: Duration = Duration { seconds: 374355659 };
}
impl Planet for Saturn {
    const SOLAR_YEAR: Duration = Duration { seconds: 929292362 };
}
impl Planet for Uranus {
    const SOLAR_YEAR: Duration = Duration {
        seconds: 2651370019,
    };
}
impl Planet for Neptune {
    const SOLAR_YEAR: Duration = Duration {
        seconds: 5200418560,
    };
}
