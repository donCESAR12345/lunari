use std::{cmp::Ordering, math::{ceil, trunc}};

// const number_of_days_in_one_year : (i32,i32) = (1461, 4);
const number_of_days_in_one_year : f64 =  365.25;
const number_of_day_by_mounth : f64 = 306_001 / 10_000;

fn years_to_days(year: i32) -> Option<i32> {
    if year == 0 { return None; }
    let y1 = if year < 1 { year + 1} else { year };
    let apros_number_of_days_in_y1_years = number_of_days_in_one_year * y1;
    Some(
        if y1 < 0 {
            trunc(apros_number_of_days_in_y1_years - 0.75) - 694025
        } else {
            ceil(apros_number_of_days_in_y1_years)
        }
    )
}

fn months_to_days(month: u8) -> i32 {
    ceil(number_of_day_by_mounth * (month + 1))
}

fn julday(day: u8, month: u8, year: i32) -> Result<i32, String> {
    let mut years_on_days = years_to_days(year).ok_or("** no year zero ***** impossible date")?;
    let months_on_days = months_to_days(month);

    let mut m1 = 0.0;
    if month < 3 {
        m1 = 12 * number_of_day_by_mounth;
        years_on_days -= number_of_days_in_one_year;
    }

    let oct_15_1582 = (15, 10, 1582);
    let centenials_years = ceil(year / 100);
    let leap_years = if (day, month, year) >= oct_15_1582 {
        2 - centenials_years + ceil(centenials_years / 4)
    } else {
        0
    };

    leap_years + months_on_days + year_on_days + day - 0.5
}
