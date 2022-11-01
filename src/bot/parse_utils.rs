use chrono::{NaiveDate, Duration, NaiveTime};

pub fn parse_date(date: &str) -> Result<NaiveDate, String> {
    let date_split = date.split('/').collect::<Vec<_>>();

    if date_split.len() != 3 {
        return Err("Date must have three slash-delimited numbers".to_string())
    }

    let month = date_split[0].parse::<u32>();
    let month = if let Ok(month) = month {
        month
    }
    else {
        return Err("Date month must be a number".to_string());
    };

    let day = date_split[1].parse::<u32>();
    let day = if let Ok(day) = day {
        day
    }
    else {
        return Err("Date day must be a number".to_string());
    };

    let year = date_split[1].parse::<i32>();
    let year = if let Ok(year) = year {
        year
    }
    else {
        return Err("Date year must be a number".to_string());
    };

    Ok(NaiveDate::from_ymd(year, month, day))
}

pub fn parse_time(time: &str) -> Result<NaiveTime, String> {
    if !(6..=7).contains(&time.len()) {
        return Err("Time length must be 6..=7".to_string())
    }

    let meridiem = &time[(time.len()-2)..].to_lowercase();

    let hour_offset = if meridiem == "am" {
        0
    }
    else if meridiem == "pm" {
        12
    }
    else {
        return Err("Time meridiem must be am or pm".to_string());
    };

    let time = &time[..(time.len()-2)];
    let time_split = time.split(':').collect::<Vec<_>>();

    if time_split.len() != 2 {
        return Err("Time must have two colon-delimited numbers".to_string())
    }

    let hour = time_split[0].parse::<u32>();
    let hour = if let Ok(hour) = hour {
        if !(1..=12).contains(&hour) {
            return Err("Time hour must be 1..=12".to_string());
        }
        hour
    }
    else {
        return Err("Time hour must be a number".to_string());
    };

    let min = time_split[1].parse::<u32>();
    let min = if let Ok(min) = min {
        if !(0..=59).contains(&min) {
            return Err("Time min must be 0..=59".to_string());
        }
        min
    }
    else {
        return Err("Time hour must be a number".to_string());
    };
    
    let time = NaiveTime::from_hms(hour_offset + hour, min, 0);

    Ok(time)
}