use serde::{Deserialize, Serialize};

pub const ALL_TYPE_IDS: [i32; 6] = [1, 2, 3, 4, 5, 6];

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

#[derive(Serialize, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AppointmentFilterInput {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_start: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_end: Option<String>,
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

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberPropertyInput {
	pub property_ext_id: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_string: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_number: Option<f64>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_boolean: Option<bool>,
}

pub enum MemberPropertyValue {
	String(String),
	Number(f64),
	Date(String),
	Boolean(bool),
}

impl MemberPropertyInput {
	pub fn new(property_ext_id: String, value: MemberPropertyValue) -> Self {
		let mut input = Self {
			property_ext_id,
			value_string: None,
			value_number: None,
			value_date: None,
			value_boolean: None,
		};
		match value {
			MemberPropertyValue::String(value) => input.value_string = Some(value),
			MemberPropertyValue::Number(value) => input.value_number = Some(value),
			MemberPropertyValue::Date(value) => input.value_date = Some(value),
			MemberPropertyValue::Boolean(value) => input.value_boolean = Some(value),
		}
		input
	}
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberInput {
	pub mail: String,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub firstname: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lastname: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub mobile_phone: Option<String>,
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub properties: Vec<MemberPropertyInput>,
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
	pub publicsite: Option<bool>,
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

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberDto {
	pub km_user_id: Option<i64>,
	pub mail: Option<String>,
	pub firstname: Option<String>,
	pub lastname: Option<String>,
	pub mobile_phone: Option<String>,
	pub address: Option<AddressDto>,
	#[serde(default)]
	pub properties: Vec<MemberPropertyDto>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddressDto {
	pub id: Option<i64>,
	pub country_iso3: Option<String>,
	pub city: Option<String>,
	pub region: Option<String>,
	pub postal_code: Option<String>,
	pub streetline1: Option<String>,
	pub streetline2: Option<String>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberPropertyDto {
	pub id: Option<i64>,
	pub member_property_ext_id: Option<String>,
	pub member_property_name: Option<String>,
	pub member_property_active: Option<bool>,
	#[serde(default)]
	pub member_property_options: Vec<SelectOptionDto>,
	pub property_type: Option<String>,
	pub value_number: Option<f64>,
	pub value_string: Option<String>,
	pub value_date: Option<String>,
	pub value_boolean: Option<bool>,
	pub last_modified_date: Option<String>,
	pub value_select_option: Option<SelectOptionDto>,
	pub association_property: Option<bool>,
	pub li_self_edit_allowed: Option<bool>,
}

#[derive(Deserialize, Serialize)]
pub struct SelectOptionDto {
	pub id: Option<i64>,
	pub name: Option<String>,
}

#[derive(clap::ValueEnum, Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Reply {
	Positive,
	Maybe,
	Negative,
	Unanswered,
	#[serde(other)]
	#[value(skip)]
	Unknown,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplyDto {
	pub km_user_id: Option<i64>,
	pub km_user_first_name: Option<String>,
	pub km_user_last_name: Option<String>,
	pub km_user_email: Option<String>,
	pub external_invite: Option<bool>,
	pub reply: Option<Reply>,
	pub reply_comment: Option<String>,
	pub replied_at: Option<String>,
}

#[cfg(test)]
mod tests {
	use super::*;

	/// UC-002 | Main Success Scenario
	#[test]
	fn uc002_filter_input_skips_empty_fields() {
		let filter = AppointmentFilterInput::default();
		let json = serde_json::to_string(&filter).unwrap();
		assert_eq!(json, "{\"typeIds\":[]}");
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

	/// UC-003 | A3: Override Description
	#[test]
	fn uc003_create_input_with_description() {
		let input = CreateAppointmentInput {
			name: None,
			description: Some("Annual concert".to_string()),
			start_zoned: "2026-06-15T19:30:00Z".to_string(),
			appointment_template_ext_id: "tmpl-1".to_string(),
			creator_mail: "admin@example.com".to_string(),
		};

		let json = serde_json::to_string(&input).unwrap();
		assert!(json.contains("\"description\":\"Annual concert\""));
		assert!(!json.contains("\"name\""));
	}
	/// UC-005 | Main Success Scenario
	#[test]
	fn uc005_member_dto_deserializes_properties() {
		let member: MemberDto =
			serde_json::from_str(r#"{"kmUserId":42,"mail":"a@example.com","address":{"city":"Zurich","postalCode":"8000"},"properties":[{"memberPropertyExtId":"section","propertyType":"SELECT","valueSelectOption":{"id":2,"name":"Brass"}}]}"#).unwrap();
		assert_eq!(member.km_user_id, Some(42));
		assert_eq!(member.address.as_ref().unwrap().city.as_deref(), Some("Zurich"));
		assert!(member.properties[0].member_property_options.is_empty());
		assert_eq!(member.properties[0].value_select_option.as_ref().unwrap().name.as_deref(), Some("Brass"));
	}

	/// UC-006 | Main Success Scenario
	#[test]
	fn uc006_member_input_serializes_typed_fields() {
		let input = MemberInput {
			mail: "a@example.com".into(),
			firstname: None,
			lastname: None,
			mobile_phone: None,
			properties: vec![MemberPropertyInput {
				property_ext_id: "active".into(),
				value_string: None,
				value_number: None,
				value_date: None,
				value_boolean: Some(false),
			}],
		};
		let value = serde_json::to_value(input).unwrap();
		assert_eq!(value["properties"][0]["valueBoolean"], false);
		assert!(value.get("firstname").is_none());
	}

	/// UC-008 | Main Success Scenario
	#[test]
	fn uc008_reply_dto_deserializes() {
		let reply: ReplyDto = serde_json::from_str(r#"{"kmUserFirstName":"Alex","reply":"MAYBE","repliedAt":"2026-01-01T12:00:00Z"}"#).unwrap();
		assert_eq!(reply.reply, Some(Reply::Maybe));
	}

	/// UC-008 | BR-001: Unrecognized reply values remain visible
	#[test]
	fn uc008_unknown_reply_deserializes() {
		let reply: ReplyDto = serde_json::from_str(r#"{"reply":"LATER"}"#).unwrap();
		assert_eq!(reply.reply, Some(Reply::Unknown));
	}
}
