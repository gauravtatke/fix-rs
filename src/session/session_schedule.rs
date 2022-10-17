use super::{Properties, SessionId};
use chrono::{DateTime, Datelike, NaiveTime, Offset, TimeZone, Utc, Weekday};
use chrono_tz::{Tz, Tz::GMT};
use derive_builder::Builder;

// schedule related settings
const START_DAY_SETTING: &str = "start_day";
const END_DAY_SETTING: &str = "end_day";
const START_TIME_SETTING: &str = "start_time";
const END_TIME_SETTING: &str = "end_time";
const TIMEZONE_SETTING: &str = "default_timezone";

#[derive(Debug, Builder)]
pub struct SessionSchedule {
    start_time: NaiveTime,
    end_time: NaiveTime,
    #[builder(setter(strip_option), default)]
    start_day: Option<Weekday>,
    #[builder(setter(strip_option), default)]
    end_day: Option<Weekday>,
    #[builder(default = "chrono_tz::Tz::GMT")]
    time_zone: chrono_tz::Tz,
    #[builder(default)]
    is_non_stop: bool,
}

impl SessionSchedule {
    pub fn new(
        start_time: NaiveTime, start_day: Option<Weekday>, end_time: NaiveTime,
        end_day: Option<Weekday>, timezone: Tz, non_stop: bool,
    ) -> Self {
        Self {
            start_time,
            start_day,
            end_time,
            end_day,
            time_zone: timezone,
            is_non_stop: non_stop,
        }
    }

    pub fn create_schedule(session_id: &SessionId, settings: &Properties) -> Self {
        let start_time = settings.get_optional_config::<NaiveTime>(session_id, START_TIME_SETTING);
        let end_time = settings.get_optional_config::<NaiveTime>(session_id, END_TIME_SETTING);

        let mut is_non_stop = false;
        if start_time.is_none() && end_time.is_none() {
            is_non_stop = true;
        } else if start_time.is_none() || end_time.is_none() {
            panic!("start_time and end_time both are mandatory");
        }

        let start_day = settings.get_optional_config::<Weekday>(session_id, START_DAY_SETTING);
        let end_day = settings.get_optional_config::<Weekday>(session_id, END_DAY_SETTING);
        if is_non_stop && (start_day.is_some() || end_day.is_some()) {
            panic!("start or end day specified without start time or end time");
        }

        let time_zone: chrono_tz::Tz =
            settings.get_optional_config(session_id, TIMEZONE_SETTING).unwrap_or(chrono_tz::UTC);
        SessionSchedule::new(
            start_time.unwrap(),
            start_day,
            end_time.unwrap(),
            end_day,
            time_zone,
            is_non_stop,
        )
    }

    pub fn is_session_time(&self) -> bool {
        if self.is_non_stop {
            return true;
        }

        let now_datetime = self.time_zone.from_utc_datetime(&Utc::now().naive_utc());
        // get today's session start and end datetime
        let today_start_datetime = now_datetime.date().and_time(self.start_time).unwrap();
        let today_end_datetime = now_datetime.date().and_time(self.end_time).unwrap();
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
            weekly_start_date = weekly_start_date.pred();
            if weekly_start_date.weekday() == session_end_weekday {
                // means that today's date if already out of sesssion window
                // because going back end day is encountered
                return false;
            }
        }
        // weekly start_date is on correct weekday for sesssion
        // update the date with time of self.start_time
        let weekly_start_datetime =
            weekly_start_date.and_time(today_start_datetime.time()).unwrap();

        while weekly_end_date.weekday() != session_end_weekday {
            // go forward one day
            weekly_end_date = weekly_end_date.succ();
            if weekly_end_date.weekday() == session_start_weekday {
                // means that today's date if already out of sesssion window
                // because going forward start day is encountered
                return false;
            }
        }
        let weekly_end_datetime = weekly_end_date.and_time(today_end_datetime.time()).unwrap();
        weekly_start_datetime <= now_datetime && now_datetime <= weekly_end_datetime
    }

    // this is for testing purposes
    pub fn find_nearest_interval(&self) {
        let local_date_time = self.time_zone.from_utc_datetime(&Utc::now().naive_utc());
        let start_date_time = local_date_time.date().and_time(self.start_time).unwrap();
        let end_date_time = local_date_time.date().and_time(self.end_time).unwrap();
        println!("local_date_time {}", local_date_time);
        if self.start_day.is_none() && self.end_day.is_none() {
            // daily start and end time
        }
        // start going back 1 day until you get to same day of the week
        let mut weekly_start = start_date_time.date();
        let start_weekday = self.start_day.unwrap();
        let end_weekday = self.end_day.unwrap();
        while weekly_start.weekday() != start_weekday {
            weekly_start = weekly_start.pred();
            if weekly_start.weekday() == end_weekday {
                // going back if it encounters end weekday first then
                // it means if was already outside of the
                panic!("Out of session: end day going back");
            }
        }
        // start date is weekly start
        let weekly_start = weekly_start.and_time(start_date_time.time()).unwrap();

        let mut weekly_end = end_date_time.date();
        while weekly_end.weekday() != end_weekday {
            weekly_end = weekly_end.succ();
            if weekly_end.weekday() == start_weekday {
                // start weekdat encountered going forward in time
                // means current datetime is already out of session time
                panic!("Out of session: start day goind forwward");
            }
        }
        let weekly_end = weekly_end.and_time(end_date_time.time()).unwrap();
        println!("\n\n session interval start {}, end {}\n\n", weekly_start, weekly_end);
    }
}

pub fn session_time(time_zone: chrono_tz::Tz) -> bool {
    // create a current datetime = Utc::now()
    // extract the time from above
    // start and end time to utc start & end
    let naive_utc_datetime = Utc::now().naive_utc();
    let naive_utc_date = naive_utc_datetime.date();
    let naive_utc_time = naive_utc_datetime.time();
    println!(
        "UTC datetime {}, date {}, time {}\n",
        naive_utc_datetime, naive_utc_date, naive_utc_time
    );

    let sc_datetime = time_zone.from_utc_datetime(&naive_utc_datetime);
    println!(
        "TZ {} datetime {}, date {}, time {}\n",
        time_zone,
        sc_datetime,
        sc_datetime.naive_local(),
        sc_datetime.time()
    );

    let curr_offset = time_zone.offset_from_utc_datetime(&naive_utc_datetime);
    println!("offset {:?}", curr_offset);
    true
}

pub fn is_current_time_between(curr_timezone: Tz, start: &str, end: &str) -> bool {
    let utc_naive_date_time = Utc::now().naive_utc();
    let curr_naive_tz_time = curr_timezone.from_utc_datetime(&utc_naive_date_time).time();
    let start = start.parse::<NaiveTime>().unwrap();
    let end = end.parse::<NaiveTime>().unwrap();
    println!(
        "tz: {}, curr_utc: {}, curr_tz_time: {}, start: {}, end: {}",
        curr_timezone, utc_naive_date_time, curr_naive_tz_time, start, end
    );
    start <= curr_naive_tz_time && curr_naive_tz_time <= end
}

#[cfg(test)]
mod schedule_tests {
    use std::str::FromStr;

    use super::*;
    use chrono::Local;
    use chrono_tz::Tz;

    #[test]
    fn test_session_time() {
        session_time(Tz::Europe__London);
        let local_time = Local::now();
        println!(
            "\n\nlocal_time {}, naive_local {}, naive_utc: {}\n\n",
            local_time,
            local_time.naive_local(),
            local_time.naive_utc()
        );
        assert_eq!(is_current_time_between(Tz::Asia__Kolkata, "05:00:00", "13:00:00"), false);
    }

    #[test]
    fn test_between_session() {
        let schedule = SessionScheduleBuilder::default()
            .start_time(NaiveTime::from_str("9:00:01").unwrap())
            .end_time(NaiveTime::from_str("15:29:59").unwrap())
            .build()
            .unwrap();
        println!(
            "time between {}",
            is_current_time_between(Tz::Asia__Kolkata, "9:00:01", "19:30:00")
        );
    }
}
