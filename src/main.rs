mod api;
mod cli;
mod config;
mod model;
mod output;

use anyhow::Context;
use clap::Parser;

use cli::{
	AppointmentAction, AttendanceAction, AttendanceListArgs, Cli, Commands, ConfigAction, CreateArgs, ListArgs, MemberAction, MemberListArgs, MemberWriteArgs, OutputFormat, ReplyAction, ReplyListArgs,
};
use model::{
	ALL_TYPE_IDS, ActivationStatus, AppointmentFilterInput, AttendanceDto, CreateAppointmentInput, DateMode, MemberDto, MemberInput, MemberPropertyInput, MemberPropertyValue, PublishedStatus, Reply,
	ReplyDto, SortModeApi,
};

fn main() {
	env_logger::Builder::from_default_env().format_timestamp_millis().init();
	if let Err(e) = run() {
		eprintln!("Error: {e:#}");
		std::process::exit(1);
	}
}

fn run() -> anyhow::Result<()> {
	let cli = Cli::parse();

	match cli.command {
		Commands::Config { action } => handle_config(action),
		Commands::Appointment { action } => match action {
			AppointmentAction::List(args) => handle_list(args),
			AppointmentAction::Create(args) => handle_create(args),
		},
		Commands::Member { action } => match action {
			MemberAction::List(args) => handle_member_list(args),
			MemberAction::Add(args) => handle_member_write(args, false),
			MemberAction::Update(args) => handle_member_write(args, true),
		},
		Commands::Reply { action } => match action {
			ReplyAction::List(args) => handle_reply_list(args),
		},
		Commands::Attendance { action } => match action {
			AttendanceAction::List(args) => handle_attendance_list(args),
		},
	}
}

fn handle_config(action: ConfigAction) -> anyhow::Result<()> {
	match action {
		ConfigAction::Set { name, api_key, creator_mail } => config::handle_set(&name, api_key.as_deref(), creator_mail.as_deref()),
		ConfigAction::Default { name } => config::handle_default(&name),
		ConfigAction::Edit => config::handle_edit(),
		ConfigAction::Path => config::handle_path(),
	}
}

fn handle_list(args: ListArgs) -> anyhow::Result<()> {
	let (_, profile) = selected_profile(args.association.as_deref())?;

	let filter = build_filter(&args);
	let appointments = api::list_appointments(&profile.api_key, &filter)?;

	match args.format {
		OutputFormat::Json => output::print_json(&appointments, args.utc),
		OutputFormat::Table => output::print_table(&appointments, args.utc),
	}
}

fn handle_create(args: CreateArgs) -> anyhow::Result<()> {
	let (profile_name, profile) = selected_profile(args.association.as_deref())?;

	let creator_mail = profile
		.creator_mail
		.as_deref()
		.ok_or_else(|| anyhow::anyhow!("Creator email not configured for profile '{profile_name}'. Run 'km config set {profile_name}' to add it."))?;

	let start_zoned = resolve_start_zoned(&args.start)?;

	let input = CreateAppointmentInput {
		name: args.name,
		description: args.description,
		start_zoned,
		appointment_template_ext_id: args.template,
		creator_mail: creator_mail.to_string(),
	};

	if args.dry_run {
		let json = serde_json::to_string_pretty(&input).context("Failed to serialize request")?;
		println!("{json}");
		return Ok(());
	}

	let appointment = api::create_appointment(&profile.api_key, &input)?;
	let json = serde_json::to_string_pretty(&appointment).context("Failed to serialize response")?;
	println!("{json}");
	Ok(())
}

fn selected_profile(association: Option<&str>) -> anyhow::Result<(String, config::Profile)> {
	let cfg = config::load_config().context("Failed to load configuration")?;
	let (name, profile) = config::resolve_profile(&cfg, association)?;
	Ok((name.to_string(), profile.clone()))
}

fn profile_key(association: Option<&str>) -> anyhow::Result<String> {
	Ok(selected_profile(association)?.1.api_key)
}

fn filter_members(mut members: Vec<MemberDto>, mail: Option<&str>) -> Vec<MemberDto> {
	if let Some(mail) = mail {
		members.retain(|member| member.mail.as_deref().is_some_and(|value| value.eq_ignore_ascii_case(mail)));
	}
	members
}

fn filter_replies(mut replies: Vec<ReplyDto>, reply: Option<&Reply>) -> Vec<ReplyDto> {
	if let Some(reply) = reply {
		replies.retain(|item| item.reply.as_ref() == Some(reply));
	}
	replies
}

fn filter_attendances(mut attendances: Vec<AttendanceDto>, attending: bool, absent: bool) -> Vec<AttendanceDto> {
	if attending {
		attendances.retain(|item| item.attending == Some(true));
	}
	if absent {
		attendances.retain(|item| item.attending == Some(false));
	}
	attendances
}

fn handle_member_list(args: MemberListArgs) -> anyhow::Result<()> {
	let key = profile_key(args.association.as_deref())?;
	let members = filter_members(api::list_members(&key)?, args.mail.as_deref());
	match args.format {
		OutputFormat::Json => output::print_generic_json(&members),
		OutputFormat::Table => output::print_members_table(&members),
	}
}

fn build_member_input(args: MemberWriteArgs) -> anyhow::Result<MemberInput> {
	let mut properties = Vec::new();
	for (ext, value) in args.prop_string {
		properties.push(MemberPropertyInput::new(ext, MemberPropertyValue::String(value)));
	}
	for (ext, value) in args.prop_number {
		let number: f64 = value.parse().with_context(|| format!("Invalid number for property '{ext}': '{value}'"))?;
		if !number.is_finite() {
			anyhow::bail!("Property '{ext}' requires a finite number");
		}
		properties.push(MemberPropertyInput::new(ext, MemberPropertyValue::Number(number)));
	}
	for (ext, value) in args.prop_date {
		let date = resolve_start_zoned(&value)?;
		properties.push(MemberPropertyInput::new(ext, MemberPropertyValue::Date(date)));
	}
	for (ext, value) in args.prop_bool {
		let boolean: bool = value.parse().with_context(|| format!("Invalid boolean for property '{ext}': '{value}' (expected true or false)"))?;
		properties.push(MemberPropertyInput::new(ext, MemberPropertyValue::Boolean(boolean)));
	}
	Ok(MemberInput {
		mail: args.mail,
		firstname: args.firstname,
		lastname: args.lastname,
		mobile_phone: args.mobile_phone,
		properties,
	})
}

fn validate_update_input(input: &MemberInput) -> anyhow::Result<()> {
	if input.firstname.is_none() && input.lastname.is_none() && input.mobile_phone.is_none() && input.properties.is_empty() {
		anyhow::bail!("nothing to update: provide a name, mobile phone, or data field");
	}
	Ok(())
}

fn handle_member_write(args: MemberWriteArgs, update: bool) -> anyhow::Result<()> {
	let dry_run = args.dry_run;
	let association = args.association.clone();
	let input = build_member_input(args)?;
	if update {
		validate_update_input(&input)?;
	}
	if dry_run {
		println!("{}", serde_json::to_string_pretty(&input).context("Failed to serialize request")?);
		return Ok(());
	}
	let key = profile_key(association.as_deref())?;
	if update {
		api::update_member(&key, &input)?;
	} else {
		api::add_member(&key, &input)?;
	}
	eprintln!("Member {}: {}", if update { "updated" } else { "added" }, input.mail);
	Ok(())
}

fn handle_reply_list(args: ReplyListArgs) -> anyhow::Result<()> {
	let key = profile_key(args.association.as_deref())?;
	let replies = filter_replies(api::list_replies(&key, args.app_id)?, args.reply.as_ref());
	match args.format {
		OutputFormat::Json => output::print_generic_json(&replies),
		OutputFormat::Table => output::print_replies_table(&replies),
	}
}

fn handle_attendance_list(args: AttendanceListArgs) -> anyhow::Result<()> {
	let key = profile_key(args.association.as_deref())?;
	let attendances = filter_attendances(api::list_attendances(&key, args.app_id)?, args.attending, args.absent);
	match args.format {
		OutputFormat::Json => output::print_generic_json(&attendances),
		OutputFormat::Table => output::print_attendances_table(&attendances),
	}
}

/// Resolve a start datetime to a zoned ISO 8601 string (UC-003 BR-001).
/// - Naive datetime → interpret as local machine timezone, convert to offset format.
/// - UTC or zoned datetime → pass through unchanged.
fn resolve_start_zoned(input: &str) -> anyhow::Result<String> {
	if !input.contains('T') {
		anyhow::bail!("--start requires a datetime, not just a date. Example: 2026-06-15T19:30:00");
	}

	// Already has timezone info → pass through
	if input.ends_with('Z') || input.contains('+') || input.rfind('-').is_some_and(|i| i > input.find('T').unwrap()) {
		return Ok(input.to_string());
	}

	// Naive datetime → interpret as local machine timezone
	let naive = chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S")
		.or_else(|_| chrono::NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M"))
		.with_context(|| format!("Invalid datetime format: '{input}'. Expected e.g. 2026-06-15T19:30:00"))?;

	let local = chrono::Local::now().timezone();
	let zoned = naive
		.and_local_timezone(local)
		.single()
		.ok_or_else(|| anyhow::anyhow!("Ambiguous or invalid local time: '{input}'. Use an explicit offset instead, e.g. {input}+02:00"))?;

	Ok(zoned.format("%Y-%m-%dT%H:%M:%S%:z").to_string())
}

/// Normalize a date input to ISO 8601 date-time (UC-002 BR-005).
/// - "2026-01-01"           → "2026-01-01T{suffix}"
/// - "2026-01-01T14:00:00"  → "2026-01-01T14:00:00Z"
/// - "2026-01-01T14:00:00Z" → unchanged
fn normalize_datetime(input: &str, day_suffix: &str) -> String {
	if !input.contains('T') {
		format!("{input}T{day_suffix}")
	} else if input.ends_with('Z') || input.contains('+') || input.rfind('-').is_some_and(|i| i > input.find('T').unwrap()) {
		input.to_string()
	} else {
		format!("{input}Z")
	}
}

fn build_filter(args: &ListArgs) -> AppointmentFilterInput {
	let has_date_filter = args.from.is_some() || args.to.is_some();

	let mut activation_status_list = Vec::new();
	if args.active {
		activation_status_list.push(ActivationStatus::Active);
	}
	if args.cancelled {
		activation_status_list.push(ActivationStatus::Cancelled);
	}

	let published_status = if args.published {
		Some(PublishedStatus::Published)
	} else if args.unpublished {
		Some(PublishedStatus::Unpublished)
	} else {
		None
	};

	let sort_mode = args.sort.as_ref().map(|s| match s {
		cli::SortMode::Startdate => SortModeApi::Startdate,
		cli::SortMode::Deadline => SortModeApi::Deadline,
	});

	let date_mode = if has_date_filter { Some(DateMode::FromDate) } else { Some(DateMode::Upcoming) };

	AppointmentFilterInput {
		filter_start: args.from.as_deref().map(|d| normalize_datetime(d, "00:00:00Z")),
		filter_end: args.to.as_deref().map(|d| normalize_datetime(d, "23:59:59Z")),
		type_ids: if args.type_ids.is_empty() { ALL_TYPE_IDS.to_vec() } else { args.type_ids.clone() },
		activation_status_list,
		published_status,
		tags: args.tag.clone(),
		sort_mode,
		date_mode,
		page: args.page,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_plain_date_from() {
		assert_eq!(normalize_datetime("2026-01-01", "00:00:00Z"), "2026-01-01T00:00:00Z");
	}

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_plain_date_to() {
		assert_eq!(normalize_datetime("2026-12-31", "23:59:59Z"), "2026-12-31T23:59:59Z");
	}

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_naive_datetime_appends_z() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00", "00:00:00Z"), "2026-01-01T14:00:00Z");
	}

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_utc_datetime_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00Z", "00:00:00Z"), "2026-01-01T14:00:00Z");
	}

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_positive_offset_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00+02:00", "00:00:00Z"), "2026-01-01T14:00:00+02:00");
	}

	/// UC-002 | BR-005: Date Input Normalization
	#[test]
	fn uc002_normalize_negative_offset_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00-05:00", "00:00:00Z"), "2026-01-01T14:00:00-05:00");
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_utc_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00Z").unwrap(), "2026-06-15T19:30:00Z");
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_positive_offset_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00+02:00").unwrap(), "2026-06-15T19:30:00+02:00");
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_negative_offset_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00-05:00").unwrap(), "2026-06-15T19:30:00-05:00");
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_naive_adds_local_offset() {
		let result = resolve_start_zoned("2026-06-15T19:30:00").unwrap();
		// Must start with the same date/time and have an offset
		assert!(result.starts_with("2026-06-15T19:30:00"));
		assert!(result.contains('+') || result.contains('-'));
		assert!(!result.ends_with('Z'));
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_naive_short_format() {
		let result = resolve_start_zoned("2026-06-15T19:30").unwrap();
		assert!(result.starts_with("2026-06-15T19:30:00"));
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_date_only_rejected() {
		let result = resolve_start_zoned("2026-06-15");
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("requires a datetime"));
	}

	/// UC-003 | BR-001: Naive Datetime Timezone Resolution
	#[test]
	fn uc003_resolve_start_invalid_format_rejected() {
		let result = resolve_start_zoned("2026-06-15Tnonsense");
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("Invalid datetime format"));
	}

	fn default_list_args() -> ListArgs {
		ListArgs {
			association: None,
			from: None,
			to: None,
			type_ids: vec![],
			active: false,
			cancelled: false,
			published: false,
			unpublished: false,
			sort: None,
			format: OutputFormat::Json,
			utc: false,
			page: None,
			tag: vec![],
		}
	}

	/// UC-002 | BR-001: Default Date Mode
	#[test]
	fn uc002_build_filter_defaults_to_upcoming() {
		let filter = build_filter(&default_list_args());
		let json = serde_json::to_string(&filter).unwrap();
		assert!(json.contains("\"dateMode\":\"UPCOMING\""));
		assert!(!json.contains("filterStart"));
		assert!(!json.contains("filterEnd"));
	}

	/// UC-002 | A1: Filter by Date Range
	/// Business Rules: BR-005
	#[test]
	fn uc002_build_filter_from_to_switches_to_from_date() {
		let args = ListArgs {
			from: Some("2026-01-01".to_string()),
			to: Some("2026-12-31".to_string()),
			..default_list_args()
		};
		let filter = build_filter(&args);
		let json = serde_json::to_string(&filter).unwrap();
		assert!(json.contains("\"dateMode\":\"FROM_DATE\""));
		assert!(json.contains("\"filterStart\":\"2026-01-01T00:00:00Z\""));
		assert!(json.contains("\"filterEnd\":\"2026-12-31T23:59:59Z\""));
	}

	/// UC-002 | A2: Filter by Type, Status, or Tags
	#[test]
	fn uc002_build_filter_all_filter_options() {
		let args = ListArgs {
			active: true,
			cancelled: true,
			published: true,
			type_ids: vec![3, 7],
			tag: vec!["Music".to_string(), "Jazz".to_string()],
			sort: Some(cli::SortMode::Startdate),
			..default_list_args()
		};
		let filter = build_filter(&args);
		let json = serde_json::to_string(&filter).unwrap();
		assert!(json.contains("\"activationStatusList\":[\"ACTIVE\",\"CANCELLED\"]"));
		assert!(json.contains("\"publishedStatus\":\"PUBLISHED\""));
		assert!(json.contains("\"typeIds\":[3,7]"));
		assert!(json.contains("\"tags\":[\"Music\",\"Jazz\"]"));
		assert!(json.contains("\"sortMode\":\"STARTDATE\""));
	}

	/// UC-002 | A11: Explicit Page Selection
	/// Business Rules: BR-003
	#[test]
	fn uc002_build_filter_explicit_page() {
		let args = ListArgs { page: Some(3), ..default_list_args() };
		let filter = build_filter(&args);
		let json = serde_json::to_string(&filter).unwrap();
		assert!(json.contains("\"page\":3"));
	}

	/// UC-002 | BR-002: Output Format Default
	#[test]
	fn uc002_output_format_defaults_to_json() {
		let cli = Cli::parse_from(["km", "appointment", "list"]);
		match cli.command {
			Commands::Appointment {
				action: AppointmentAction::List(args),
			} => assert_eq!(args.format, OutputFormat::Json),
			_ => panic!("Expected List command"),
		}
	}

	/// UC-003 | A9: Missing Required Flags
	#[test]
	fn uc003_missing_template_rejected() {
		let result = Cli::try_parse_from(["km", "appointment", "create", "--start", "2026-06-15T19:30:00"]);
		assert!(result.is_err());
	}

	/// UC-003 | A9: Missing Required Flags
	#[test]
	fn uc003_missing_start_rejected() {
		let result = Cli::try_parse_from(["km", "appointment", "create", "--template", "tmpl-1"]);
		assert!(result.is_err());
	}
	/// UC-002 | Required typeIds default
	#[test]
	fn uc002_default_type_ids() {
		assert_eq!(build_filter(&default_list_args()).type_ids, ALL_TYPE_IDS);
	}

	/// UC-002 | A2: Explicit appointment types replace the default
	#[test]
	fn uc002_explicit_type_ids_replace_default() {
		let args = ListArgs {
			type_ids: vec![2, 5],
			..default_list_args()
		};
		assert_eq!(build_filter(&args).type_ids, vec![2, 5]);
	}

	/// UC-002 / UC-003 | Legacy top-level commands are removed
	#[test]
	fn uc002_uc003_legacy_commands_rejected() {
		assert!(Cli::try_parse_from(["km", "list"]).is_err());
		assert!(Cli::try_parse_from(["km", "create", "--template", "tmpl-1", "--start", "2026-06-15T19:30:00"]).is_err());
		assert!(matches!(
			Cli::parse_from(["km", "appointment", "list"]).command,
			Commands::Appointment { action: AppointmentAction::List(_) }
		));
	}

	/// UC-005 | Main Success Scenario
	#[test]
	fn uc005_member_list_command_parses() {
		assert!(matches!(Cli::parse_from(["km", "member", "list"]).command, Commands::Member { action: MemberAction::List(_) }));
	}

	/// UC-005 | A1: Filter by Email
	#[test]
	fn uc005_mail_filter_ignores_case_and_can_return_empty() {
		let members: Vec<MemberDto> = serde_json::from_str(r#"[{"mail":"Alex@Example.com"},{"mail":"other@example.com"}]"#).unwrap();
		let matched = filter_members(members, Some("alex@example.com"));
		assert_eq!(matched.len(), 1);
		assert_eq!(filter_members(matched, Some("missing@example.com")).len(), 0);
	}

	/// UC-006 | Main Success Scenario
	#[test]
	fn uc006_member_add_command_parses() {
		assert!(matches!(
			Cli::parse_from(["km", "member", "add", "--mail", "a@example.com"]).command,
			Commands::Member { action: MemberAction::Add(_) }
		));
	}

	/// UC-006 | A2: Invalid Data Field
	#[test]
	fn uc006_invalid_properties_are_rejected() {
		for value in ["x=abc", "x=NaN"] {
			let cli = Cli::parse_from(["km", "member", "add", "--mail", "a@example.com", "--prop-number", value]);
			let Commands::Member { action: MemberAction::Add(args) } = cli.command else {
				panic!("Expected member add")
			};
			assert!(build_member_input(args).is_err());
		}
		let cli = Cli::parse_from(["km", "member", "add", "--mail", "a@example.com", "--prop-bool", "x=maybe"]);
		let Commands::Member { action: MemberAction::Add(args) } = cli.command else {
			panic!("Expected member add")
		};
		assert!(build_member_input(args).is_err());
		assert!(Cli::try_parse_from(["km", "member", "add", "--mail", "a@example.com", "--prop-string", "missing-equals"]).is_err());
	}

	/// UC-006 | Typed property parser
	#[test]
	fn uc006_member_property_parser() {
		let cli = Cli::parse_from(["km", "member", "add", "--mail", "a@example.com", "--prop-string", "section=brass", "--prop-bool", "active=false"]);
		let Commands::Member { action: MemberAction::Add(args) } = cli.command else {
			panic!("Expected member add");
		};
		let input = build_member_input(args).unwrap();
		assert_eq!(input.properties.len(), 2);
		assert_eq!(input.properties[0].property_ext_id, "section");
		assert_eq!(input.properties[1].value_boolean, Some(false));
	}
	/// UC-006 / UC-007 | Typed data fields and update command
	#[test]
	fn uc007_member_update_typed_properties() {
		let cli = Cli::parse_from([
			"km",
			"member",
			"update",
			"--mail",
			"a@example.com",
			"--prop-number",
			"score=1.5",
			"--prop-date",
			"joined=2026-06-15T19:30:00+02:00",
		]);
		let Commands::Member { action: MemberAction::Update(args) } = cli.command else {
			panic!("Expected member update");
		};
		let input = build_member_input(args).unwrap();
		assert_eq!(input.properties[0].value_number, Some(1.5));
		assert_eq!(input.properties[1].value_date.as_deref(), Some("2026-06-15T19:30:00+02:00"));
	}

	/// UC-007 | A2: No Change Provided
	#[test]
	fn uc007_update_requires_a_change() {
		let cli = Cli::parse_from(["km", "member", "update", "--mail", "a@example.com"]);
		let Commands::Member { action: MemberAction::Update(args) } = cli.command else {
			panic!("Expected member update")
		};
		let input = build_member_input(args).unwrap();
		assert!(validate_update_input(&input).is_err());
	}

	/// UC-008 | A1: Filter by Reply
	#[test]
	fn uc008_reply_filter() {
		let cli = Cli::parse_from(["km", "reply", "list", "123", "--reply", "maybe"]);
		let Commands::Reply { action: ReplyAction::List(args) } = cli.command else {
			panic!("Expected reply list")
		};
		assert_eq!(args.app_id, 123);
		let replies: Vec<ReplyDto> = serde_json::from_str(r#"[{"reply":"MAYBE"},{"reply":"POSITIVE"}]"#).unwrap();
		let matched = filter_replies(replies, args.reply.as_ref());
		assert_eq!(matched.len(), 1);
		assert!(filter_replies(matched, Some(&Reply::Negative)).is_empty());
	}

	/// UC-009 | A1: Filter by Attendance
	#[test]
	fn uc009_attendance_filter_and_exclusive_flags() {
		let cli = Cli::parse_from(["km", "attendance", "list", "123", "--absent"]);
		let Commands::Attendance { action: AttendanceAction::List(args) } = cli.command else {
			panic!("Expected attendance list")
		};
		assert_eq!(args.app_id, 123);
		let attendances: Vec<AttendanceDto> = serde_json::from_str(r#"[{"attending":true},{"attending":false},{"attending":null}]"#).unwrap();
		assert_eq!(filter_attendances(attendances, false, args.absent).len(), 1);
		let attendances: Vec<AttendanceDto> = serde_json::from_str(r#"[{"attending":true},{"attending":false},{"attending":null}]"#).unwrap();
		assert_eq!(filter_attendances(attendances, true, false).len(), 1);
		let attendances: Vec<AttendanceDto> = serde_json::from_str(r#"[{"attending":true},{"attending":false},{"attending":null}]"#).unwrap();
		assert!(filter_attendances(attendances, true, true).is_empty());
		assert!(Cli::try_parse_from(["km", "attendance", "list", "123", "--attending", "--absent"]).is_err());
	}
}
