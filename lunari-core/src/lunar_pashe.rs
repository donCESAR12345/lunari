enum LunarPhaseName {
    NewMoon,
    FirstQuarterMoon,
    FullMoon,
    LastQuarterMoon
}

const LUNAR_CYCLES_PER_YEAR: f64 = 365.25 / 29.530588853;

fn elipse_check (moon_lat: f64, phase) {
    const PI: f64 = f64::consts:PI;
    let mut local_moon_lat = (PI / 180.0) * moon_lat;
    local_moon_lat = (
        local_moon_lat - (PI * (local_moon_lat / PI).floor())
    ).abs();
    local_moon_lat = if local_moon_lat > 0.37 {
        PI - local_moon_lat
    } else {
        local_moon_lat
    };
    let phase_name = match phase {
        0 => "Solar",
        2 => "Lunar",
        _ => "",
    }
    match local_moon_lat {
        local_moon_lat if local_moon_lat < 2.42600766e-1
    }

}

fn main() {
    let x: f64 = 6.0;

    let a = x.tan();
    let b = x.sin() / x.cos();

    println!("{} {}", a , b);
    assert_eq!(a,b)
}
