pub mod c_datetime_format {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(|dt| dt.naive_local())
            .map_err(serde::de::Error::custom)
    }
}

pub mod c_datetime_option_format {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    
    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    
    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => {
                serializer.serialize_none()
            }
        }
        
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(|dt| Some(dt.naive_local()))
            .map_err(serde::de::Error::custom)
    }
}

pub mod c_datetime_no_second_option_format {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    
    const FORMAT: &'static str = "%Y-%m-%d %H:%M";
    
    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => {
                serializer.serialize_none()
            }
        }
        
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(|dt| Some(dt.naive_local()))
            .map_err(serde::de::Error::custom)
    }
}

pub mod c_datetime_short_option_format {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};
    
    const FORMAT: &'static str = "%Y/%m/%d";
    
    pub fn serialize<S>(date: &Option<NaiveDateTime>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        match date {
            Some(date) => {
                let s = format!("{}", date.format(FORMAT));
                serializer.serialize_str(&s)
            }
            None => {
                serializer.serialize_none()
            }
        }
        
    }
    
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(|dt| Some(dt.naive_local()))
            .map_err(serde::de::Error::custom)
    }
}

pub mod c_datetime_utc_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

//    const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S";
    const FORMAT: &'static str = "%+";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod c_date_format {
    use chrono::{NaiveDateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(|dt| dt.naive_local())
            .map_err(serde::de::Error::custom)
    }
}
