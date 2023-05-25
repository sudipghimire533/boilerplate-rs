const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

pub mod utc {
    use super::FORMAT;
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::offset::TimeZone;
    use chrono::DateTime;
    use chrono::Utc;
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct Something {
        #[serde(serialize_with = "utc::serialize")]
        #[serde(deserialize_with = "utc::deserialize")]
        pub time: DateTime<Utc>,
        pub name: String,
    }

    #[test]
    fn time_utc_serde() {
        let time = Utc.with_ymd_and_hms(2001, 03, 16, 20, 56, 32).unwrap();
        let something = Something {
            time,
            name: "Sudip".to_string(),
        };

        let expected = r#"{"time":"2001-03-16 20:56:32","name":"Sudip"}"#;

        let serialized = serde_json::to_string(&something).unwrap();
        let deserialized = serde_json::from_str(&expected).unwrap();

        assert_eq!(expected, serialized.trim());
        assert_eq!(something, deserialized);
    }
}
