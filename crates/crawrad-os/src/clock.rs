//! Local clock queries from the operating system.
//!
//! Provides the current local clock time formatted as `"HH:MM"` (24-hour, zero-padded).
//! On Windows, this reads the system local time via `GetLocalTime`.
//! On non-Windows platforms, this falls back to UTC derived from `std::time::SystemTime`.

use crawrad_note::note;
#[note("clock-hh-mm")]
pub fn hh_mm(hour: u32, minute: u32) -> String {
    format!("{hour:02}:{minute:02}")
}

#[note("clock-local-hh-mm")]
#[cfg(target_os = "windows")]
pub fn local_hh_mm() -> String {
    use windows_sys::Win32::Foundation::SYSTEMTIME;
    use windows_sys::Win32::System::SystemInformation::GetLocalTime;

    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    // SAFETY: `st` is a stack-allocated Win32 `SYSTEMTIME` struct. `GetLocalTime`
    // writes the current local system date and time into the buffer.
    unsafe {
        GetLocalTime(&mut st);
    }
    hh_mm(st.wHour as u32, st.wMinute as u32)
}

#[note("clock-local-hh-mm-2")]
#[cfg(not(target_os = "windows"))]
pub fn local_hh_mm() -> String {
    let now = std::time::SystemTime::now();
    let duration = now
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let day_secs = secs % 86400;
    let hour = (day_secs / 3600) as u32;
    let minute = ((day_secs % 3600) / 60) as u32;
    hh_mm(hour, minute)
}

#[note("clock-local-stamp")]
pub fn local_stamp() -> String {
    let (y, mo, d, h, mi) = local_parts();
    format!("{y:04}-{mo:02}-{d:02} {}", hh_mm(h, mi))
}

#[cfg(target_os = "windows")]
fn local_parts() -> (u32, u32, u32, u32, u32) {
    use windows_sys::Win32::Foundation::SYSTEMTIME;
    use windows_sys::Win32::System::SystemInformation::GetLocalTime;

    let mut st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    // SAFETY: as in `local_hh_mm`: `GetLocalTime` fills the stack `SYSTEMTIME`.
    unsafe {
        GetLocalTime(&mut st);
    }
    let f = |v: u16| v as u32;
    (
        f(st.wYear),
        f(st.wMonth),
        f(st.wDay),
        f(st.wHour),
        f(st.wMinute),
    )
}

#[cfg(not(target_os = "windows"))]
fn local_parts() -> (u32, u32, u32, u32, u32) {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let (y, m, d) = civil_from_days((secs / 86400) as i64);
    let day = secs % 86400;
    (
        y as u32,
        m,
        d,
        (day / 3600) as u32,
        ((day % 3600) / 60) as u32,
    )
}

#[note("clock-civil-from-days")]
pub fn civil_from_days(z: i64) -> (i64, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = (if mp < 10 { mp + 3 } else { mp - 9 }) as u32;
    (yoe + era * 400 + i64::from(m <= 2), m, d)
}

#[note("clock-days-from-civil")]
pub fn days_from_civil(y: i64, m: u32, d: u32) -> i64 {
    let y = if m <= 2 { y - 1 } else { y };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let mp = i64::from(if m > 2 { m - 3 } else { m + 9 });
    let doy = (153 * mp + 2) / 5 + d as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

#[note("clock-local-offset-seconds")]
#[cfg(target_os = "windows")]
pub fn local_offset_seconds() -> i64 {
    use windows_sys::Win32::Foundation::SYSTEMTIME;
    use windows_sys::Win32::System::SystemInformation::{GetLocalTime, GetSystemTime};

    let mut local_st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    let mut sys_st: SYSTEMTIME = unsafe { core::mem::zeroed() };
    // SAFETY: `local_st` and `sys_st` are stack-allocated Win32 `SYSTEMTIME` structs.
    // `GetLocalTime` and `GetSystemTime` write the current local and UTC times into the buffers.
    unsafe {
        GetLocalTime(&mut local_st);
        GetSystemTime(&mut sys_st);
    }
    let to_secs = |st: &SYSTEMTIME| -> i64 {
        let days = days_from_civil(st.wYear as i64, st.wMonth as u32, st.wDay as u32);
        days * 86_400 + (st.wHour as i64) * 3600 + (st.wMinute as i64) * 60 + (st.wSecond as i64)
    };
    let diff = to_secs(&local_st) - to_secs(&sys_st);
    let minutes = if diff >= 0 {
        (diff + 30) / 60
    } else {
        (diff - 30) / 60
    };
    minutes * 60
}

#[note("clock-local-offset-seconds-2")]
#[cfg(not(target_os = "windows"))]
pub fn local_offset_seconds() -> i64 {
    0
}

#[cfg(test)]
#[path = "test/clock/test.rs"]
mod tests;
