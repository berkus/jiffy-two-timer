#![feature(test)]
extern crate two_timer;
use two_timer::{parse, Config};
extern crate chrono;
use chrono::{Duration, TimeZone, Utc};

#[test]
fn always() {
    let alpha = chrono::MIN_DATE.and_hms_milli(0, 0, 0, 0);
    let omega = chrono::MAX_DATE.and_hms_milli(23, 59, 59, 999);
    for phrase in [
        "always",
        "ever",
        "all time",
        "forever",
        "from beginning to end",
        "from the beginning to the end",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(alpha, start);
        assert_eq!(omega, end);
    }
}

#[test]
fn yesterday() {
    let now = Utc::now();
    let (start, end) = parse("yesterday", Some(Config::default().now(now))).unwrap();
    assert!(start < now);
    assert!(end < now);
    let then = now - Duration::days(1);
    assert!(start < then);
    assert!(then < end);
    let then = then - Duration::days(1);
    assert!(then < start);
}

#[test]
fn tomorrow() {
    let now = Utc::now();
    let (start, end) = parse("tomorrow", Some(Config::default().now(now))).unwrap();
    assert!(start > now);
    assert!(end > now);
    let then = now + Duration::days(1);
    assert!(start < then);
    assert!(then < end);
    let then = then + Duration::days(1);
    assert!(then > end);
}

#[test]
fn today() {
    let now = Utc::now();
    let (start, end) = parse("today", Some(Config::default().now(now))).unwrap();
    assert!(start < now);
    assert!(end > now);
    let then = now + Duration::days(1);
    assert!(start < then);
    assert!(then > end);
    let then = now - Duration::days(1);
    assert!(then < start);
    assert!(then < end);
}

#[test]
fn day_5_6_69_at_3_30_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 0);
    for phrase in [
        "at 3:30 PM on 5-6-69",
        "3:30 p.m. on 5-6-69",
        "at 15:30 on 5-6-69",
        "15:30 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in [
        "at 3 PM on 5-6-69",
        "3 p.m. on 5-6-69",
        "at 15 on 5-6-69",
        "15 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_00_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 0);
    for phrase in [
        "at 3:30:00 PM on 5-6-69",
        "3:30:00 p.m. on 5-6-69",
        "at 15:30:00 on 5-6-69",
        "15:30:00 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_pm() {
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 30, 1);
    for phrase in [
        "at 3:30:01 PM on 5-6-69",
        "3:30:01 p.m. on 5-6-69",
        "at 15:30:01 on 5-6-69",
        "15:30:01 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn day_5_6_69_at_3_30_01_am() {
    let then = Utc.ymd(1969, 5, 6).and_hms(3, 30, 1);
    for phrase in [
        "at 3:30:01 AM on 5-6-69",
        "3:30:01 a.m. on 5-6-69",
        "at 3:30:01 on 5-6-69",
        "3:30:01 on 5-6-69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3 PM", "3 pm", "15"].iter() {
        let (start, end) = parse(phrase, Some(Config::default().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn at_3_00_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3:00 PM", "3:00 pm", "15:00"].iter() {
        let (start, end) = parse(phrase, Some(Config::default().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn at_3_00_00_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(16, 0, 0);
    let then = Utc.ymd(1969, 5, 6).and_hms(15, 0, 0);
    for phrase in ["3:00:00 PM", "3:00:00 pm", "15:00:00"].iter() {
        let (start, end) = parse(phrase, Some(Config::default().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::seconds(1), end);
    }
}

#[test]
fn at_3_pm_yesterday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(14, 0, 0);
    let then = Utc.ymd(1969, 5, 5).and_hms(15, 0, 0);
    for phrase in ["3 PM yesterday", "3 pm yesterday", "15 yesterday"].iter() {
        let (start, end) = parse(phrase, Some(Config::default().now(now))).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::hours(1), end);
    }
}

#[test]
fn alphabetic_5_6_69() {
    let then = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    for phrase in [
        "May 6, 1969",
        "May 6, '69",
        "May 6, 69",
        "6 May 1969",
        "6 May '69",
        "6 May 69",
        "Tuesday, May 6, 1969",
        "Tuesday, May 6, '69",
        "Tuesday, May 6, 69",
        "Tues, May 6, 1969",
        "Tues, May 6, '69",
        "Tues, May 6, 69",
        "Tue, May 6, 1969",
        "Tue, May 6, '69",
        "Tue, May 6, 69",
        "Tu, May 6, 1969",
        "Tu, May 6, '69",
        "Tu, May 6, 69",
        "Tues., May 6, 1969",
        "Tues., May 6, '69",
        "Tues., May 6, 69",
        "Tue., May 6, 1969",
        "Tue., May 6, '69",
        "Tue., May 6, 69",
        "Tu., May 6, 1969",
        "Tu., May 6, '69",
        "Tu., May 6, 69",
        "T, May 6, 1969",
        "T, May 6, '69",
        "T, May 6, 69",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn ymd_5_31_69() {
    let then = Utc.ymd(1969, 5, 31).and_hms(0, 0, 0);
    for phrase in [
        "5-31-69",
        "5/31/69",
        "5.31.69",
        "5/31/1969",
        "5-31-1969",
        "5.31.1969",
        "69-5-31",
        "69/5/31",
        "69.5.31",
        "1969/5/31",
        "1969-5-31",
        "1969.5.31",
        "5-31-'69",
        "5/31/'69",
        "5.31.'69",
        "'69-5-31",
        "'69/5/31",
        "'69.5.31",
        "31-5-69",
        "31/5/69",
        "31.5.69",
        "31/5/1969",
        "31-5-1969",
        "31.5.1969",
        "69-31-5",
        "69/31/5",
        "69.31.5",
        "1969/31/5",
        "1969-31-5",
        "1969.31.5",
        "31-5-'69",
        "31/5/'69",
        "31.5.'69",
        "'69-31-5",
        "'69/31/5",
        "'69.31.5",
        "05-31-69",
        "05/31/69",
        "05.31.69",
        "05/31/1969",
        "05-31-1969",
        "05.31.1969",
        "69-05-31",
        "69/05/31",
        "69.05.31",
        "1969/05/31",
        "1969-05-31",
        "1969.05.31",
        "05-31-'69",
        "05/31/'69",
        "05.31.'69",
        "'69-05-31",
        "'69/05/31",
        "'69.05.31",
        "31-05-69",
        "31/05/69",
        "31.05.69",
        "31/05/1969",
        "31-05-1969",
        "31.05.1969",
        "69-31-05",
        "69/31/05",
        "69.31.05",
        "1969/31/05",
        "1969-31-05",
        "1969.31.05",
        "31-05-'69",
        "31/05/'69",
        "31.05.'69",
        "'69-31-05",
        "'69/31/05",
        "'69.31.05",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::days(1), end);
    }
}

#[test]
fn leap_day() {
    let rv = parse("2019-02-29", None);
    assert!(rv.is_err());
    let rv = parse("2020-02-29", None);
    assert!(rv.is_ok());
}

#[test]
fn may_1969() {
    let m1 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let m2 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    for phrase in ["May 1969", "May '69"].iter() {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(m1, start);
        assert_eq!(m2, end);
    }
}

#[test]
fn this_month() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    let (start, end) = parse("this month", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_month() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 7, 1).and_hms(0, 0, 0);
    let (start, end) = parse("next month", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_month() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let (start, end) = parse("last month", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_year() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 1, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    let (start, end) = parse("this year", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_year() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1970, 1, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1971, 1, 1).and_hms(0, 0, 0);
    let (start, end) = parse("next year", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_year() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1968, 1, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 1, 1).and_hms(0, 0, 0);
    let (start, end) = parse("last year", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_week() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 5).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 12).and_hms(0, 0, 0);
    let (start, end) = parse("this week", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_week() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 12).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 19).and_hms(0, 0, 0);
    let (start, end) = parse("next week", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_week() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 28).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 5).and_hms(0, 0, 0);
    let (start, end) = parse("last week", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_week_sunday_starts() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 11).and_hms(0, 0, 0);
    let (start, end) = parse(
        "this week",
        Some(Config::default().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_week_sunday_starts() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 11).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 18).and_hms(0, 0, 0);
    let (start, end) = parse(
        "next week",
        Some(Config::default().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_week_sunday_starts() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 27).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let (start, end) = parse(
        "last week",
        Some(Config::default().now(now).monday_starts_week(false)),
    )
    .unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_pay_period() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1968, 5, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 18).and_hms(0, 0, 0);
    let (start, end) = parse("this pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_pay_period() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1968, 5, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 5, 18).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    let (start, end) = parse("next pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_pay_period() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year before "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1968, 5, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 4, 20).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let (start, end) = parse("last pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_pay_period_weird() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1970, 4, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 18).and_hms(0, 0, 0);
    let (start, end) = parse("this pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_pay_period_weird() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1970, 4, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 5, 18).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    let (start, end) = parse("next pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_pay_period_weird() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    // two-week pay period beginning about a year *after* "now" on a Sunday
    let config = Config::default()
        .pay_period_start(Some(Utc.ymd(1970, 4, 5)))
        .pay_period_length(14)
        .now(now);
    let d1 = Utc.ymd(1969, 4, 20).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 4).and_hms(0, 0, 0);
    let (start, end) = parse("last pay period", Some(config)).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_april() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let (start, end) = parse("this april", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_april() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1970, 4, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1970, 5, 1).and_hms(0, 0, 0);
    let (start, end) = parse("next april", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_april() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1968, 4, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1968, 5, 1).and_hms(0, 0, 0);
    let (start, end) = parse("last april", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_friday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 9).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 10).and_hms(0, 0, 0);
    let (start, end) = parse("this friday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_friday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 16).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 17).and_hms(0, 0, 0);
    let (start, end) = parse("next friday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_friday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 2).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 3).and_hms(0, 0, 0);
    let (start, end) = parse("last friday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn this_monday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 5).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let (start, end) = parse("this monday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn next_monday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 12).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 13).and_hms(0, 0, 0);
    let (start, end) = parse("next monday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn last_monday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 28).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 4, 29).and_hms(0, 0, 0);
    let (start, end) = parse("last monday", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn dawn_of_time() {
    let then = chrono::MIN_DATE.and_hms_milli(0, 0, 0, 0);
    for phrase in [
        "the beginning",
        "the beginning of time",
        "the first moment",
        "the start",
        "the very start",
        "the first instant",
        "the dawn of time",
        "the big bang",
        "the birth of the universe",
    ]
        .iter()
    {
        let (start, end) = parse(phrase, None).unwrap();
        assert_eq!(then, start);
        assert_eq!(then + Duration::minutes(1), end);
    }
}

#[test]
fn the_crack_of_doom() {
    let then = chrono::MAX_DATE.and_hms_milli(23, 59, 59, 999);
    for phrase in [
        "the end",
        "the end of time",
        "the very end",
        "the last moment",
        "eternity",
        "infinity",
        "doomsday",
        "the crack of doom",
        "armageddon",
        "ragnarok",
        "the big crunch",
        "the heat death of the universe",
        "doom",
        "death",
        "perdition",
        "the last hurrah",
        "ever after",
        "the last syllable of recorded time",
    ]
        .iter()
    {
        let (_, end) = parse(phrase, None).unwrap();
        assert_eq!(then, end);
    }
}

#[test]
fn friday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 5, 2).and_hms(0, 0, 0);
    let (start, end) = parse("Friday", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn tuesday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 4, 29).and_hms(0, 0, 0);
    let (start, end) = parse("Tuesday", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn monday() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 5, 5).and_hms(0, 0, 0);
    let (start, end) = parse("Monday", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::days(1), end);
}

#[test]
fn friday_at_3_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 5, 2).and_hms(15, 0, 0);
    let (start, end) = parse("Friday at 3 pm", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::hours(1), end);
}

#[test]
fn tuesday_at_3_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 4, 29).and_hms(15, 0, 0);
    let (start, end) = parse("Tuesday at 3 pm", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::hours(1), end);
}

#[test]
fn monday_at_3_pm() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let then = Utc.ymd(1969, 5, 5).and_hms(15, 0, 0);
    let (start, end) = parse("Monday at 3 pm", Some(Config::default().now(now))).unwrap();
    assert_eq!(then, start);
    assert_eq!(then + Duration::hours(1), end);
}

#[test]
fn just_may() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 6, 1).and_hms(0, 0, 0);
    let (start, end) = parse("May", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn just_april() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1969, 4, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1969, 5, 1).and_hms(0, 0, 0);
    let (start, end) = parse("April", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}

#[test]
fn just_june() {
    let now = Utc.ymd(1969, 5, 6).and_hms(0, 0, 0);
    let d1 = Utc.ymd(1968, 6, 1).and_hms(0, 0, 0);
    let d2 = Utc.ymd(1968, 7, 1).and_hms(0, 0, 0);
    let (start, end) = parse("June", Some(Config::default().now(now))).unwrap();
    assert_eq!(d1, start);
    assert_eq!(d2, end);
}