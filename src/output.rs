use anyhow::Context;
use chrono::{DateTime, Utc};
use comfy_table::{Table, presets::UTF8_FULL_CONDENSED};

use crate::model::AppointmentDto;

pub fn print_json(appointments: &[AppointmentDto], utc: bool) -> anyhow::Result<()> {
	if utc {
		let json = serde_json::to_string_pretty(appointments).context("Failed to serialize JSON")?;
		println!("{json}");
		return Ok(());
	}

	let converted: Vec<serde_json::Value> = appointments.iter().map(convert_appointment_times).collect::<anyhow::Result<Vec<_>>>()?;

	let json = serde_json::to_string_pretty(&converted).context("Failed to serialize JSON")?;
	println!("{json}");
	Ok(())
}

pub fn print_table(appointments: &[AppointmentDto], utc: bool) -> anyhow::Result<()> {
	let mut table = Table::new();
	table.load_preset(UTF8_FULL_CONDENSED);
	table.set_header(vec!["ID", "Name", "Start", "End", "Location", "Remind Deadline", "Status Deadline", "Active", "Tags"]);

	for appointment in appointments {
		let id = appointment.id.map(|i| i.to_string()).unwrap_or_default();
		let name = appointment.name.as_deref().unwrap_or("");
		let start = format_datetime(appointment.start.as_deref(), appointment.timezone_id.as_deref(), utc);
		let end = format_datetime(appointment.end.as_deref(), appointment.timezone_id.as_deref(), utc);
		let location = appointment.location.as_ref().and_then(|l| l.name.as_deref()).unwrap_or("");
		let status_deadline = format_datetime(appointment.status_deadline.as_deref(), appointment.timezone_id.as_deref(), utc);
		let remind_deadline = format_datetime(appointment.remind_deadline.as_deref(), appointment.timezone_id.as_deref(), utc);
		let active = appointment.active.map(|a| if a { "yes" } else { "no" }).unwrap_or("");
		let tags: String = appointment.tags.iter().filter_map(|t| t.tag.as_deref()).collect::<Vec<_>>().join(", ");

		table.add_row(vec![&id, name, &start, &end, location, &remind_deadline, &status_deadline, active, &tags]);
	}

	println!("{table}");
	Ok(())
}

fn format_datetime(dt_str: Option<&str>, timezone_id: Option<&str>, utc: bool) -> String {
	let Some(dt_str) = dt_str else {
		return String::new();
	};

	if utc {
		return dt_str.to_string();
	}

	convert_to_local(dt_str, timezone_id).unwrap_or_else(|| dt_str.to_string())
}

fn convert_to_local(dt_str: &str, timezone_id: Option<&str>) -> Option<String> {
	let dt = dt_str.parse::<DateTime<Utc>>().ok()?;
	let tz_name = timezone_id?;
	let tz: chrono_tz::Tz = tz_name.parse().ok().or_else(|| {
		eprintln!("Warning: Unknown timezone '{}', displaying as UTC", tz_name);
		None
	})?;
	let local = dt.with_timezone(&tz);
	Some(local.format("%a %Y-%m-%d %H:%M %Z").to_string())
}

fn convert_appointment_times(appointment: &AppointmentDto) -> anyhow::Result<serde_json::Value> {
	let mut value = serde_json::to_value(appointment).context("Failed to serialize appointment")?;

	if let serde_json::Value::Object(ref mut map) = value {
		let tz_id = appointment.timezone_id.as_deref();

		if let Some(start) = appointment.start.as_deref()
			&& let Some(converted) = convert_to_local(start, tz_id)
		{
			map.insert("start".to_string(), serde_json::Value::String(converted));
		}
		if let Some(end) = appointment.end.as_deref()
			&& let Some(converted) = convert_to_local(end, tz_id)
		{
			map.insert("end".to_string(), serde_json::Value::String(converted));
		}
		if let Some(sd) = appointment.status_deadline.as_deref()
			&& let Some(converted) = convert_to_local(sd, tz_id)
		{
			map.insert("statusDeadline".to_string(), serde_json::Value::String(converted));
		}
		if let Some(rd) = appointment.remind_deadline.as_deref()
			&& let Some(converted) = convert_to_local(rd, tz_id)
		{
			map.insert("remindDeadline".to_string(), serde_json::Value::String(converted));
		}
	}

	Ok(value)
}

#[cfg(test)]
mod tests {
	use super::*;

	/// UC-002 | BR-007: Timezone Display Default
	#[test]
	fn uc002_convert_utc_to_zurich() {
		let result = convert_to_local("2026-03-15T17:00:00Z", Some("Europe/Zurich"));
		assert!(result.is_some());
		let local = result.unwrap();
		assert!(local.contains("2026-03-15"));
		assert!(local.contains("18:00"));
	}

	/// UC-002 | BR-007: Timezone Display Default
	#[test]
	fn uc002_convert_unknown_timezone_returns_none() {
		let result = convert_to_local("2026-03-15T17:00:00Z", Some("Invalid/Timezone"));
		assert!(result.is_none());
	}

	/// UC-002 | BR-007: Timezone Display Default
	#[test]
	fn uc002_convert_no_timezone_returns_none() {
		let result = convert_to_local("2026-03-15T17:00:00Z", None);
		assert!(result.is_none());
	}

	/// UC-002 | A5: UTC Output
	#[test]
	fn uc002_format_datetime_utc_returns_raw() {
		let result = format_datetime(Some("2026-03-15T17:00:00Z"), Some("Europe/Zurich"), true);
		assert_eq!(result, "2026-03-15T17:00:00Z");
	}

	/// UC-002 | A6: No Appointments Found
	#[test]
	fn uc002_format_datetime_none_returns_empty() {
		let result = format_datetime(None, Some("Europe/Zurich"), false);
		assert_eq!(result, "");
	}
}
