use crate::error::Result;
use chrono::{DateTime, NaiveDate, NaiveDateTime, TimeZone, Utc};

pub fn date_to_unix_nanos(date_str: &str) -> Result<i64> {
    let naive_datetime = if date_str.len() == 10 {
        // Parse date-only format YYYY-MM-DD
        let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
        naive_date.and_hms_opt(0, 0, 0).unwrap() // Set time to midnight
    } else {
        // Parse datetime format YYYY-MM-DD HH:MM:SS
        NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?
    };
    // Convert the NaiveDateTime to a DateTime<Utc>
    let datetime_utc: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive_datetime, Utc);

    // Convert to Unix time in nanoseconds
    let unix_nanos = datetime_utc.timestamp_nanos_opt().unwrap();

    Ok(unix_nanos)
}

pub fn unix_nanos_to_date(unix_nanos: i64) -> Result<String> {
    // Convert the Unix nanoseconds to a DateTime<Utc>
    let datetime_utc: DateTime<Utc> = Utc.timestamp_nanos(unix_nanos);

    // Format the DateTime<Utc> to a string in the format "YYYY-MM-DD HH:MM:SS"
    let formatted_date = datetime_utc.format("%Y-%m-%d %H:%M:%S").to_string();

    Ok(formatted_date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_datetime_to_unix_nanos() -> Result<()> {
        let date_str = "2021-11-01 01:01:01";

        // Test
        let unix_nanos = date_to_unix_nanos(date_str)?;

        // Validate
        assert_eq!(1635728461000000000, unix_nanos);
        Ok(())
    }

    #[test]
    fn test_date_to_unix_nanos() -> Result<()> {
        let date_str = "2021-11-01";

        // Test
        let unix_nanos = date_to_unix_nanos(date_str)?;

        // Validate
        assert_eq!(1635724800000000000, unix_nanos);

        Ok(())
    }

    #[test]
    fn test_unix_to_date() -> Result<()> {
        let unix = 1635728461000000000;

        //Test
        let iso = unix_nanos_to_date(unix)?;

        // Validate
        assert_eq!("2021-11-01 01:01:01", iso);
        Ok(())
    }
}
