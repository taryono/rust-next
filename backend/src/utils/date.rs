use crate::errors::AppError;
use chrono::NaiveDate;

/// Format standar aplikasi
pub const DATE_FORMAT: &str = "%Y-%m-%d";

/// Parse Option<String> → Option<NaiveDate>
pub fn parse_opt_date(value: Option<String>) -> Result<Option<NaiveDate>, AppError> {
    match value {
        Some(v) if !v.trim().is_empty() => NaiveDate::parse_from_str(&v, DATE_FORMAT)
            .map(Some)
            .map_err(|_| AppError::BadRequest(format!("Format tanggal harus {}", DATE_FORMAT))),
        _ => Ok(None),
    }
}

/// Parse String wajib
pub fn parse_date(value: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(value, DATE_FORMAT)
        .map_err(|_| AppError::BadRequest(format!("Format tanggal harus {}", DATE_FORMAT)))
}

/// NaiveDate → String
pub fn format_date(date: NaiveDate) -> String {
    date.format(DATE_FORMAT).to_string()
}

// ✅ 2. Cara pakai di service
// use crate::utils::date::parse_opt_date;

// active_model.dob = Set(parse_opt_date(request.dob.clone())?);
