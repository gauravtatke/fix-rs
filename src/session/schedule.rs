use crate::session::SessionConfig;
use chrono::{DateTime, Datelike, NaiveTime, TimeZone, Utc, Weekday};
use chrono_tz::Tz;

#[derive(Debug)]
pub struct SessionSchedule {
    start_time: Option<NaiveTime>,
    end_time: Option<NaiveTime>,
    start_day: Option<Weekday>,
    end_day: Option<Weekday>,
    time_zone: Tz,
    is_non_stop: bool,
}

impl SessionSchedule {
    pub(crate) fn new(
        start_time: Option<NaiveTime>,
        start_day: Option<Weekday>,
        end_time: Option<NaiveTime>,
        end_day: Option<Weekday>,
        timezone: Tz,
    ) -> Self {
        let is_non_stop =
            matches!((start_time, start_day, end_time, end_day), (None, None, None, None));
        Self {
            start_time,
            start_day,
            end_time,
            end_day,
            time_zone: timezone,
            is_non_stop,
        }
    }

    pub fn is_session_time(&self) -> bool {
        if self.is_non_stop {
            return true;
        }
        let now_utc = Utc::now();
        self.is_session_time_at(now_utc)
    }

    fn is_session_time_at(&self, now_utc: DateTime<Utc>) -> bool {
        let now_datetime = self.time_zone.from_utc_datetime(&now_utc.naive_utc()).naive_local();
        let start_time = self.start_time.unwrap();
        let end_time = self.end_time.unwrap();
        // get today's session start and end datetime
        let today_start_datetime = now_datetime.date().and_time(start_time);
        let today_end_datetime = now_datetime.date().and_time(end_time);
        if self.start_day.is_none() && self.end_day.is_none() {
            // daily session start and end
            // now should be between today's session start and end datetimes
            return today_start_datetime <= now_datetime && now_datetime <= today_end_datetime;
        }

        // if weekdays are given, calculate the weekly start and end datetime
        let mut weekly_start_date = today_start_datetime.date();
        let mut weekly_end_date = today_end_datetime.date();
        let session_start_weekday = self.start_day.unwrap();
        let session_end_weekday = self.end_day.unwrap();
        // using only the date, start going back until you find the date which has
        // same weekday as self.start_day
        while weekly_start_date.weekday() != session_start_weekday {
            // go back one date prior
            weekly_start_date = weekly_start_date.pred_opt().unwrap();
            if weekly_start_date.weekday() == session_end_weekday {
                // early exit. going backwards if we encounter end_day then we are already out of session window
                // consider mon-fri session & we are on saturday/sunday
                return false;
            }
        }
        // weekly start_date is on correct weekday for session
        // update the date with time of self.start_time
        let weekly_start_datetime = weekly_start_date.and_time(start_time);

        while weekly_end_date.weekday() != session_end_weekday {
            // go forward one day
            weekly_end_date = weekly_end_date.succ_opt().unwrap();
            if weekly_end_date.weekday() == session_start_weekday {
                // early exit. going forward if we encounter start_day then we are already out of session window
                // consider mon-fri session & we are on saturday/sunday
                return false;
            }
        }
        let weekly_end_datetime = weekly_end_date.and_time(end_time);
        weekly_start_datetime <= now_datetime && now_datetime <= weekly_end_datetime
    }
}

impl From<&SessionConfig> for SessionSchedule {
    fn from(config: &SessionConfig) -> Self {
        SessionSchedule::new(
            config.start_time(),
            config.start_day(),
            config.end_time(),
            config.end_day(),
            config.timezone(),
        )
    }
}

#[cfg(test)]
mod schedule_tests {
    use super::*;
    use chrono::NaiveDate;

    fn daily_schedule(start: &str, end: &str) -> SessionSchedule {
        SessionSchedule::new(
            Some(start.parse().unwrap()),
            None,
            Some(end.parse().unwrap()),
            None,
            chrono_tz::UTC,
        )
    }

    fn weekly_schedule(
        start: &str,
        start_day: Weekday,
        end: &str,
        end_day: Weekday,
    ) -> SessionSchedule {
        SessionSchedule::new(
            Some(start.parse().unwrap()),
            Some(start_day),
            Some(end.parse().unwrap()),
            Some(end_day),
            chrono_tz::UTC,
        )
    }

    fn utc(year: i32, month: u32, day: u32, hour: u32, min: u32, sec: u32) -> DateTime<Utc> {
        NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap()
            .and_utc()
    }

    // --- non-stop ---

    #[test]
    fn test_non_stop_always_true() {
        let schedule = SessionSchedule::new(None, None, None, None, chrono_tz::UTC);
        assert!(schedule.is_session_time());
    }

    // --- daily schedule (no weekdays) ---

    #[test]
    fn test_daily_within_window() {
        let schedule = daily_schedule("09:00:00", "17:00:00");
        assert!(schedule.is_session_time_at(utc(2026, 8, 20, 12, 0, 0)));
    }

    #[test]
    fn test_daily_before_start() {
        let schedule = daily_schedule("09:00:00", "17:00:00");
        assert!(!schedule.is_session_time_at(utc(2026, 8, 20, 8, 59, 59)));
    }

    #[test]
    fn test_daily_after_end() {
        let schedule = daily_schedule("09:00:00", "17:00:00");
        assert!(!schedule.is_session_time_at(utc(2026, 8, 20, 17, 0, 1)));
    }

    #[test]
    fn test_daily_at_exact_start() {
        let schedule = daily_schedule("09:00:00", "17:00:00");
        assert!(schedule.is_session_time_at(utc(2026, 8, 20, 9, 0, 0)));
    }

    #[test]
    fn test_daily_at_exact_end() {
        let schedule = daily_schedule("09:00:00", "17:00:00");
        assert!(schedule.is_session_time_at(utc(2026, 8, 20, 17, 0, 0)));
    }

    // --- weekly schedule (Mon-Fri) ---

    #[test]
    fn test_weekly_mid_week_in_session() {
        // 2026-08-19 is a Wednesday
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(schedule.is_session_time_at(utc(2026, 8, 19, 12, 0, 0)));
    }

    #[test]
    fn test_weekly_start_day_within_hours() {
        // 2026-08-17 is a Monday
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(schedule.is_session_time_at(utc(2026, 8, 17, 10, 0, 0)));
    }

    #[test]
    fn test_weekly_start_day_before_start_time() {
        // Monday before 09:00
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(!schedule.is_session_time_at(utc(2026, 8, 17, 8, 59, 59)));
    }

    #[test]
    fn test_weekly_end_day_within_hours() {
        // 2026-08-21 is a Friday
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(schedule.is_session_time_at(utc(2026, 8, 21, 16, 0, 0)));
    }

    #[test]
    fn test_weekly_end_day_after_end_time() {
        // Friday after 17:00
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(!schedule.is_session_time_at(utc(2026, 8, 21, 17, 0, 1)));
    }

    #[test]
    fn test_weekly_saturday_outside_session() {
        // 2026-08-22 is a Saturday
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(!schedule.is_session_time_at(utc(2026, 8, 22, 12, 0, 0)));
    }

    #[test]
    fn test_weekly_sunday_outside_session() {
        // 2026-08-23 is a Sunday
        let schedule = weekly_schedule("09:00:00", Weekday::Mon, "17:00:00", Weekday::Fri);
        assert!(!schedule.is_session_time_at(utc(2026, 8, 23, 12, 0, 0)));
    }

    // --- timezone handling ---

    #[test]
    fn test_timezone_conversion() {
        // Schedule is 09:00-17:00 in Asia/Kolkata (UTC+5:30)
        // 03:30 UTC = 09:00 IST → at start boundary
        let schedule = SessionSchedule::new(
            Some("09:00:00".parse().unwrap()),
            None,
            Some("17:00:00".parse().unwrap()),
            None,
            chrono_tz::Asia::Kolkata,
        );
        assert!(schedule.is_session_time_at(utc(2026, 8, 20, 3, 30, 0)));
        // 03:29 UTC = 08:59 IST → before start
        assert!(!schedule.is_session_time_at(utc(2026, 8, 20, 3, 29, 0)));
    }
}
