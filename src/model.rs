use serde::{Deserialize, Serialize};

// --- Request DTOs ---

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppointmentInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub name: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<String>,
	pub start_zoned: String,
	pub appointment_template_ext_id: String,
	pub creator_mail: String,
}

#[derive(Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentFilterInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_start: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_end: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub type_ids: Vec<i32>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub activation_status_list: Vec<ActivationStatus>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub published_status: Option<PublishedStatus>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub tags: Vec<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sort_mode: Option<SortModeApi>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_mode: Option<DateMode>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub page: Option<i32>,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ActivationStatus {
	Active,
	Cancelled,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PublishedStatus {
	Published,
	Unpublished,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SortModeApi {
	Startdate,
	Deadline,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DateMode {
	Upcoming,
	FromDate,
}

// --- Response DTOs ---

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentDto {
	pub id: Option<i64>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub start: Option<String>,
	pub end: Option<String>,
	pub timezone_id: Option<String>,
	pub active: Option<bool>,
	pub published: Option<bool>,
	pub typ_id: Option<i32>,
	pub status_deadline: Option<String>,
	pub remind_deadline: Option<String>,
	pub created_at: Option<String>,
	pub time_undefined: Option<bool>,
	pub cancel_description: Option<String>,
	pub public_sharing_url: Option<String>,
	#[serde(alias = "privateLinkURL")]
	pub private_link_url: Option<String>,
	pub external_appointment_link: Option<String>,
	pub checkin_qr_code_image_url: Option<String>,
	pub location: Option<LocationDto>,
	pub meeting_point: Option<MeetingPointDto>,
	pub group: Option<GroupDto>,
	pub org: Option<OrgDto>,
	#[serde(default)]
	pub tags: Vec<TagDto>,
	pub room: Option<RoomDto>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationDto {
	pub id: Option<i64>,
	pub name: Option<String>,
	pub geo: Option<bool>,
	pub formatted_address: Option<String>,
	pub latitude: Option<f64>,
	pub longitude: Option<f64>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MeetingPointDto {
	pub id: Option<i64>,
	pub meeting_date_time: Option<String>,
	pub meeting_location: Option<LocationDto>,
	pub description: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GroupDto {
	pub id: Option<i64>,
	pub name: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrgDto {
	pub id: Option<i64>,
	pub name: Option<String>,
	pub timezone_id: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagDto {
	pub id: Option<i64>,
	pub tag: Option<String>,
	pub color: Option<String>,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RoomDto {
	pub id: Option<i64>,
	pub name: Option<String>,
	pub description: Option<String>,
	pub capacity: Option<i32>,
}

#[cfg(test)]
mod tests {
	use super::*;

	/// UC-002 | Main Success Scenario
	#[test]
	fn uc002_filter_input_serializes_camel_case() {
		let filter = AppointmentFilterInput {
			date_mode: Some(DateMode::Upcoming),
			filter_start: Some("2026-01-01T00:00:00Z".to_string()),
			type_ids: vec![1, 2],
			activation_status_list: vec![ActivationStatus::Active],
			sort_mode: Some(SortModeApi::Startdate),
			..Default::default()
		};

		let json = serde_json::to_string(&filter).unwrap();
		assert!(json.contains("\"dateMode\":\"UPCOMING\""));
		assert!(json.contains("\"filterStart\":\"2026-01-01T00:00:00Z\""));
		assert!(json.contains("\"typeIds\":[1,2]"));
		assert!(json.contains("\"activationStatusList\":[\"ACTIVE\"]"));
		assert!(json.contains("\"sortMode\":\"STARTDATE\""));
	}

	/// UC-002 | Main Success Scenario
	#[test]
	fn uc002_filter_input_skips_empty_fields() {
		let filter = AppointmentFilterInput::default();
		let json = serde_json::to_string(&filter).unwrap();
		assert_eq!(json, "{}");
	}

	/// UC-002 | Main Success Scenario
	#[test]
	fn uc002_appointment_dto_deserializes() {
		let json = r##"{
			"id": 123,
			"name": "Rehearsal",
			"start": "2026-03-15T18:00:00Z",
			"end": "2026-03-15T20:00:00Z",
			"timezoneId": "Europe/Zurich",
			"active": true,
			"published": true,
			"tags": [{"id": 1, "tag": "Music", "color": "#ff0000"}],
			"location": {"id": 1, "name": "Town Hall", "formattedAddress": "Main St 1"}
		}"##;

		let dto: AppointmentDto = serde_json::from_str(json).unwrap();
		assert_eq!(dto.id, Some(123));
		assert_eq!(dto.name.as_deref(), Some("Rehearsal"));
		assert_eq!(dto.timezone_id.as_deref(), Some("Europe/Zurich"));
		assert_eq!(dto.tags.len(), 1);
		assert_eq!(dto.tags[0].tag.as_deref(), Some("Music"));
		assert_eq!(dto.location.as_ref().unwrap().name.as_deref(), Some("Town Hall"));
	}

	/// UC-003 | Main Success Scenario
	#[test]
	fn uc003_create_input_serializes_camel_case() {
		let input = CreateAppointmentInput {
			name: Some("Concert".to_string()),
			description: None,
			start_zoned: "2026-06-15T19:30:00+02:00".to_string(),
			appointment_template_ext_id: "tmpl-abc".to_string(),
			creator_mail: "admin@example.com".to_string(),
		};

		let json = serde_json::to_string(&input).unwrap();
		assert!(json.contains("\"startZoned\":\"2026-06-15T19:30:00+02:00\""));
		assert!(json.contains("\"appointmentTemplateExtId\":\"tmpl-abc\""));
		assert!(json.contains("\"creatorMail\":\"admin@example.com\""));
		assert!(json.contains("\"name\":\"Concert\""));
		assert!(!json.contains("\"description\""));
	}
}
