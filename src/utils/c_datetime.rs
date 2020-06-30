use chrono::offset::TimeZone;
use chrono::{NaiveDateTime, Utc, ParseError, DateTime, NaiveDate, Datelike};
use std::time::SystemTime;

pub struct CDateTime {
    now: SystemTime,
}

impl CDateTime {
    //保存当前时间
    pub fn new() -> CDateTime {
        CDateTime {
            now: SystemTime::now(),
        }
    }
    
    //距离创建时间过去多少毫秒
    pub fn elapsed_millis(&self) -> u128 {
        let u = self.now.elapsed().unwrap_or_default();
        let secs = u.as_secs();
        let millis = u.subsec_millis();
        ((secs * 1000) + (millis as u64)) as u128
    }
    
    //距离创建时间过去多少毫秒
    pub fn println_elapsed_millis(&self) {
        let s = self.elapsed_millis();
        let str = format!("{}", s);
        debug!("{}", str);
    }
}

/// *************************************************************
/// 返回 1900-01-01 00:00:00
/// *************************************************************
pub fn default_dt() -> NaiveDateTime {
    Utc.datetime_from_str("1900-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_utc()
}

/// *************************************************************
/// 将dt转换为dt 00:00:00
/// *************************************************************
pub fn c_dt_to_dt_day(dt: NaiveDateTime) -> NaiveDateTime {
    NaiveDate::from_ymd(dt.year(), dt.month(), dt.day()).and_hms(0, 0, 0)
}

/// *************************************************************
/// 将dt转换为dt HH:MZM:SS
/// *************************************************************
pub fn c_dt_to_dt_HMS(dt: NaiveDateTime,hour:u32,min:u32,sec:u32) -> NaiveDateTime {
    NaiveDate::from_ymd(dt.year(), dt.month(), dt.day()).and_hms(hour, min, sec)
}

/// *************************************************************
/// 获取UTC时间字符串
/// （格式为yyyy-MM-ddTHH:mm:ssZ）
/// *************************************************************
pub fn c_dt_now_to_utc_str1() -> String {
    let mut datetime_str = Utc::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string()
        .replace(' ', "T");
    datetime_str.push('Z');
    datetime_str
}

/// *************************************************************
/// 格式为yyyy-MM-dd HH:mm:ss
/// *************************************************************
pub fn c_dt_to_str(dt: chrono::NaiveDateTime) -> String {
    let datetime_str = dt
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();
    datetime_str
}

/// *************************************************************
/// 格式为yyyy-MM-dd
/// *************************************************************
pub fn c_dt_to_date_str(dt: chrono::NaiveDateTime) -> String {
    let datetime_str = dt
        .format("%Y-%m-%d")
        .to_string();
    datetime_str
}

/// *************************************************************
/// 字符串(yyyy-mm-dd HH:mm:ss)转换为NaiveDateTime ,转换失败返回 1900-01-01 00:00:00
/// *************************************************************
pub fn c_dt_str2dt(dt_str: Option<String>) -> Result<NaiveDateTime, &'static str> {
    let mut dt_r = Utc.datetime_from_str("1900-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_utc();
    
    if let Some(dt) = dt_str {
        //验证时间格式
        let start_dt = Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(t) = start_dt {
            dt_r = t.naive_utc();
        } else {
            // return Err("请输入正确的开始时间");
        }
    }
    
    Ok(dt_r)
}

/// *************************************************************
/// 字符串(yyyy-mm-dd HH:mm:ss)转换为NaiveDateTime ,转换失败返回 1900-01-01 00:00:00
/// *************************************************************
pub fn c_dt_str2dt_default(dt_str: Option<String>) -> NaiveDateTime {
    let mut dt_r = Utc.datetime_from_str("1900-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_utc();
    
    if let Some(dt) = dt_str {
        //验证时间格式
        let start_dt = Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(t) = start_dt {
            dt_r = t.naive_utc();
        } else {
            // return Err("请输入正确的开始时间");
        }
    }
    
    dt_r
}

/// *************************************************************
/// 字符串(yyyy-mm-dd)转换为NaiveDateTime ,转换失败返回 1900-01-01 00:00:00
/// *************************************************************
pub fn c_yyyymmdd_str2dt_default(dt_str: Option<String>) -> NaiveDateTime {
    let mut dt_r = Utc.datetime_from_str("1900-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_utc();
    // println!("======> {:?}", dt_str);
    if let Some(mut dt) = dt_str {
        if dt.len() == 10 {
            dt.push_str(" 00:00:00");
        }
        match Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S") {
            Ok(t) => {
                dt_r = t.naive_utc();
            }
            Err(e) => {
                // eprintln!("======> {:?}", e);
            }
        }
    }
    
    dt_r
}

/// *************************************************************
/// 字符串(yyyy-mm-dd HH:mm)转换为NaiveDateTime ,转换失败返回 None
/// *************************************************************
pub fn c_yyyymmddHHmm_str2dt_option(dt_str: Option<String>) -> Option<NaiveDateTime> {
    let mut dt_r = None;
    if let Some(mut dt) = dt_str {
        if dt.len() == 16 {
            dt.push_str(":00");
        }
        match Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S") {
            Ok(t) => {
                dt_r = Some(t.naive_utc());
            }
            Err(e) => {
                // eprintln!("======> {:?}", e);
            }
        }
    }
    
    dt_r
}

/// *************************************************************
/// 字符串(yyyy/mm/dd)转换为NaiveDateTime ,转换失败返回 1900-01-01 00:00:00
/// *************************************************************
pub fn c_yyyymmdd2_str2dt_default(dt_str: Option<String>) -> NaiveDateTime {
    let mut dt_r = Utc.datetime_from_str("1900-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap().naive_utc();
    // println!("======> {:?}", dt_str);
    if let Some(mut dt) = dt_str {
        if dt.len() == 10 {
            dt.push_str(" 00:00:00");
        }
        match Utc.datetime_from_str(dt.as_str(), "%Y/%m/%d %H:%M:%S") {
            Ok(t) => {
                dt_r = t.naive_utc();
            }
            Err(e) => {
                // eprintln!("======> {:?}", e);
            }
        }
    }
    
    dt_r
}

/// *************************************************************
/// 字符串(yyyy/mm/dd)转换为NaiveDateTime ,转换失败返回 None
/// *************************************************************
pub fn c_yyyymmdd2_str2dt_option(dt_str: Option<String>) -> Option<NaiveDateTime> {
    let mut dt_r = None;
    if let Some(mut dt) = dt_str {
        if dt.len() == 10 {
            dt.push_str(" 00:00:00");
        }
        match Utc.datetime_from_str(dt.as_str(), "%Y/%m/%d %H:%M:%S") {
            Ok(t) => {
                dt_r = Some(t.naive_utc());
            }
            Err(e) => {
                // eprintln!("======> {:?}", e);
            }
        }
    }
    
    dt_r
}

/// *************************************************************
/// 字符串(yyyy-mm-dd HH:mm:ss)转换为NaiveDateTime
/// *************************************************************
pub fn c_dt_str2dt_and_check(
    start_dt: Option<String>,
    end_dt: Option<String>,
) -> Result<(Option<NaiveDateTime>, Option<NaiveDateTime>), &'static str> {
    let mut start_dt_r = None;
    let mut end_dt_r = None;
    
    let base_dt = Utc.datetime_from_str("1970-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
    
    if let Some(dt) = start_dt {
        //验证时间格式
        let start_dt = Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(t) = start_dt {
            if t > base_dt {
                start_dt_r = Some(t.naive_utc());
            } else {
                return Err("开始时间小于 1970-01-01 00:00:00");
            }
        } else {
            return Err("请输入正确的开始时间");
        }
    }
    
    if let Some(dt) = end_dt {
        //验证时间格式
        let end_dt = Utc.datetime_from_str(dt.as_str(), "%Y-%m-%d %H:%M:%S");
        if let Ok(t) = end_dt {
            if t > base_dt {
                end_dt_r = Some(t.naive_utc());
            } else {
                return Err("结束时间小于 1970-01-01 00:00:00");
            }
        } else {
            return Err("请输入正确的结束时间");
        }
    }
    
    if (end_dt_r != None && start_dt_r == None) || (end_dt_r == None && start_dt_r != None) {
        return Err("请输入正确的时间范围");
    }
    
    if (end_dt_r != None && start_dt_r != None) && (end_dt_r < start_dt_r) {
        return Err("请输入正确的时间范围");
    }
    
    Ok((start_dt_r, end_dt_r))
}
