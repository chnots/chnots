use anyhow::anyhow;
use chin_tools::AResult;
use chinese_lunisolar_calendar::LunisolarDate;
use chrono::{Duration, Local, LocalResult, NaiveDateTime, NaiveTime, TimeZone, Timelike};
use num_traits::ToPrimitive;

use super::chinese::ChnTime;
use crate::krate::toent::logic::timeevent::timeenum::base::{BaseDate, BaseDateTime, BaseTime};
use crate::krate::toent::timeevent::repeater::interval::TimeInterval;

pub(crate) struct ChnTimeCalculator;

impl ChnTimeCalculator {
    pub(crate) fn add(base: &ChnTime, delta: TimeInterval) -> AResult<ChnTime> {
        let (mut year, mut month, mut day, mut hour, mut minute, mut second) =
            base.extract_datetime_parts()?;
        let mut is_leap = base.leap_month;

        let years = delta.date.year.unwrap_or(0);
        let months = delta.date.month.unwrap_or(0);
        let days = i64::from(delta.date.day.unwrap_or(0)) + i64::from(delta.week.unwrap_or(0)) * 7;
        let duration = Duration::days(days)
            + Duration::hours(i64::from(delta.time.hour.unwrap_or(0)))
            + Duration::minutes(i64::from(delta.time.minute.unwrap_or(0)))
            + Duration::seconds(i64::from(delta.time.second.unwrap_or(0)));

        if years != 0 {
            (year, month, is_leap, day) = add_lunar_year(year, month, is_leap, day, years)?;
        }

        if months != 0 {
            (year, month, is_leap, day) = add_lunar_months(year, month, is_leap, day, months)?;
        }

        if duration != Duration::zero() {
            let local_ndt =
                lunar_to_local_naive_datetime(year, month, day, is_leap, hour, minute, second)?
                    + duration;
            let local_dt = match Local.from_local_datetime(&local_ndt) {
                LocalResult::Single(v) => v,
                LocalResult::Ambiguous(v, _) => v,
                LocalResult::None => Local.from_utc_datetime(&local_ndt),
            };

            let lunar = LunisolarDate::from_date(local_dt)?;

            year = lunar.to_solar_year().to_i32();
            month = lunar
                .to_lunar_month()
                .to_u8()
                .to_i32()
                .ok_or_else(|| anyhow!("unable to convert lunar month"))?;
            is_leap = lunar.to_lunar_month().is_leap_month();
            day = lunar
                .to_lunar_day()
                .to_u8()
                .to_i32()
                .ok_or_else(|| anyhow!("unable to convert lunar day"))?;
            hour = local_ndt.time().hour().to_i32().unwrap_or(hour);
            minute = local_ndt.time().minute().to_i32().unwrap_or(minute);
            second = local_ndt.time().second().to_i32().unwrap_or(second);
        }

        Ok(rebuild_with_original_precision(
            base, year, month, day, hour, minute, second, is_leap,
        ))
    }
}

fn add_lunar_year(
    year: i32,
    month: i32,
    is_leap: bool,
    day: i32,
    years: i32,
) -> AResult<(i32, i32, bool, i32)> {
    let target_year = year
        .checked_add(years)
        .ok_or_else(|| anyhow!("lunar year overflow"))?;
    let target_is_leap = if is_leap { false } else { is_leap };

    let target = lunar_with_day_fallback(target_year, month, target_is_leap, day)?;
    Ok((target_year, month, target_is_leap, target.1))
}

fn add_lunar_months(
    mut year: i32,
    mut month: i32,
    mut is_leap: bool,
    day: i32,
    months: i32,
) -> AResult<(i32, i32, bool, i32)> {
    if months > 0 {
        for _ in 0..months {
            (year, month, is_leap) = next_lunar_month(year, month, is_leap)?;
        }
    } else {
        for _ in 0..(-months) {
            (year, month, is_leap) = prev_lunar_month(year, month, is_leap)?;
        }
    }

    let target = lunar_with_day_fallback(year, month, is_leap, day)?;
    Ok((year, month, is_leap, target.1))
}

fn next_lunar_month(year: i32, month: i32, is_leap: bool) -> AResult<(i32, i32, bool)> {
    if !is_leap && has_leap_month(year, month)? {
        return Ok((year, month, true));
    }

    if month == 12 {
        Ok((year + 1, 1, false))
    } else {
        Ok((year, month + 1, false))
    }
}

fn prev_lunar_month(year: i32, month: i32, is_leap: bool) -> AResult<(i32, i32, bool)> {
    if is_leap {
        return Ok((year, month, false));
    }

    let (prev_year, prev_month) = if month == 1 {
        (year - 1, 12)
    } else {
        (year, month - 1)
    };

    if has_leap_month(prev_year, prev_month)? {
        Ok((prev_year, prev_month, true))
    } else {
        Ok((prev_year, prev_month, false))
    }
}

fn has_leap_month(year: i32, month: i32) -> AResult<bool> {
    let y = u16::try_from(year).map_err(|_| anyhow!("invalid lunar year: {year}"))?;
    let m = u8::try_from(month).map_err(|_| anyhow!("invalid lunar month: {month}"))?;
    Ok(LunisolarDate::from_ymd(y, m, true, 1).is_ok())
}

fn lunar_with_day_fallback(
    year: i32,
    month: i32,
    is_leap: bool,
    day: i32,
) -> AResult<(LunisolarDate, i32)> {
    let y = u16::try_from(year).map_err(|_| anyhow!("invalid lunar year: {year}"))?;
    let m = u8::try_from(month).map_err(|_| anyhow!("invalid lunar month: {month}"))?;

    let mut d = day.max(1);
    while d >= 1 {
        if let Ok(date) = LunisolarDate::from_ymd(y, m, is_leap, d as u8) {
            return Ok((date, d));
        }
        d -= 1;
    }

    Err(anyhow!(
        "unable to construct lunar date: year={year}, month={month}, is_leap={is_leap}, day={day}"
    ))
}

fn lunar_to_local_naive_datetime(
    year: i32,
    month: i32,
    day: i32,
    is_leap: bool,
    hour: i32,
    minute: i32,
    second: i32,
) -> AResult<NaiveDateTime> {
    let y = u16::try_from(year).map_err(|_| anyhow!("invalid lunar year: {year}"))?;
    let m = u8::try_from(month).map_err(|_| anyhow!("invalid lunar month: {month}"))?;
    let d = u8::try_from(day).map_err(|_| anyhow!("invalid lunar day: {day}"))?;

    let lunar = LunisolarDate::from_ymd(y, m, is_leap, d)?;
    let time = NaiveTime::from_hms_opt(hour as u32, minute as u32, second as u32)
        .ok_or_else(|| anyhow!("invalid time: {hour}:{minute}:{second}"))?;

    Ok(NaiveDateTime::new(lunar.to_naive_date(), time))
}

fn rebuild_with_original_precision(
    base: &ChnTime,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    second: i32,
    leap_month: bool,
) -> ChnTime {
    let ts = BaseDateTime {
        date: BaseDate {
            year: year.into(),
            month: base.timestamp.date.month.as_ref().map(|_| month).into(),
            day: base.timestamp.date.day.as_ref().map(|_| day).into(),
        },
        time: BaseTime {
            hour: base.timestamp.time.hour.as_ref().map(|_| hour).into(),
            minute: base.timestamp.time.minute.as_ref().map(|_| minute).into(),
            second: base.timestamp.time.second.as_ref().map(|_| second).into(),
        },
    };

    ChnTime {
        leap_month,
        timestamp: ts,
    }
}

#[cfg(test)]
mod test {
    use super::ChnTimeCalculator;
    use crate::krate::toent::logic::timeevent::timeenum::{base::BaseDateTime, chinese::ChnTime};
    use crate::krate::toent::timeevent::repeater::interval::TimeInterval;
    use chinese_lunisolar_calendar::LunisolarDate;
    use chrono::Duration;
    use num_traits::ToPrimitive;

    fn chn(y: i32, m: i32, d: i32, leap: bool) -> ChnTime {
        ChnTime {
            leap_month: leap,
            timestamp: BaseDateTime::default()
                .with_year(y)
                .with_month(m)
                .with_day(d)
                .with_hour(9)
                .with_minute(30)
                .with_second(0),
        }
    }

    fn ti(years: i32, months: i32, weeks: i32, days: i32) -> TimeInterval {
        let mut interval = TimeInterval::default();
        interval.date.year = years.into();
        interval.date.month = months.into();
        interval.week = weeks.into();
        interval.date.day = days.into();
        interval
    }

    #[test]
    fn add_year_from_leap_month_clears_leap_flag() {
        let base = chn(2023, 2, 1, true);
        let got = ChnTimeCalculator::add(&base, ti(1, 0, 0, 0)).unwrap();

        assert_eq!(got.timestamp.date.year.as_ref(), Some(&2024));
        assert_eq!(got.timestamp.date.month.as_ref(), Some(&2));
        assert_eq!(got.timestamp.date.day.as_ref(), Some(&1));
        assert!(!got.leap_month);
    }

    #[test]
    fn add_month_counts_leap_month_in_sequence() {
        let base = chn(2023, 2, 1, false);
        let got = ChnTimeCalculator::add(&base, ti(0, 1, 0, 0)).unwrap();

        assert_eq!(got.timestamp.date.year.as_ref(), Some(&2023));
        assert_eq!(got.timestamp.date.month.as_ref(), Some(&2));
        assert_eq!(got.timestamp.date.day.as_ref(), Some(&1));
        assert!(got.leap_month);
    }

    #[test]
    fn add_day_uses_solar_conversion_then_back_to_lunar() {
        let base = chn(2023, 1, 20, false);
        let got = ChnTimeCalculator::add(&base, ti(0, 0, 0, 10)).unwrap();

        let start = LunisolarDate::from_ymd(2023, 1, false, 20).unwrap();
        let expected_lunar =
            LunisolarDate::from_date(start.to_naive_date() + Duration::days(10)).unwrap();

        let expected_year = expected_lunar.to_solar_year().to_i32();
        let expected_month = expected_lunar.to_lunar_month().to_u8().to_i32().unwrap();
        let expected_day = expected_lunar.to_lunar_day().to_u8().to_i32().unwrap();

        assert_eq!(got.timestamp.date.year.as_ref(), Some(&expected_year));
        assert_eq!(got.timestamp.date.month.as_ref(), Some(&expected_month));
        assert_eq!(got.timestamp.date.day.as_ref(), Some(&expected_day));
        assert_eq!(
            got.leap_month,
            expected_lunar.to_lunar_month().is_leap_month()
        );
    }
}
