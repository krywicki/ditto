pub mod serde_datetime {
    use chrono::NaiveDateTime;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
    where D: Deserializer<'de>
    {
        let val: Option<i64> = Option::deserialize(deserializer)?;
        if let Some(val) = val {
            return Ok(Some(NaiveDateTime::from_timestamp(val, 0)));
        }
        Ok(None)
    }
}

pub mod serde_pathbuf {
    use std::path::PathBuf;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<PathBuf, D::Error>
    where D: Deserializer<'de>
    {
        let v = Vec::<String>::deserialize(deserializer)?;
        let s: String = v.into_iter().collect();
        Ok(PathBuf::from(s.as_str()))
    }
}

pub mod serde_announce_list {
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
    where D: Deserializer<'de>
    {
        let v: Option<Vec<Vec<String>>> = Option::deserialize(deserializer)?;

        if let Some(v) = v {
            return Ok(Some(v.into_iter().flatten().collect()))
        }

        Ok(None)
    }
}
