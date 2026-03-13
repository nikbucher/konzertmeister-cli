mod api;
mod cli;
mod config;
mod model;
mod output;

use anyhow::Context;
use clap::Parser;

use cli::{Cli, Commands, ConfigAction, CreateArgs, ListArgs, OutputFormat};
use model::{ActivationStatus, AppointmentFilterInput, CreateAppointmentInput, DateMode, PublishedStatus, SortModeApi};

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
		Commands::List(args) => handle_list(args),
		Commands::Create(args) => handle_create(args),
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
	let cfg = config::load_config().context("Failed to load configuration")?;
	let (_, profile) = config::resolve_profile(&cfg, args.association.as_deref())?;

	let filter = build_filter(&args);
	let appointments = api::list_appointments(&profile.api_key, &filter)?;

	match args.format {
		OutputFormat::Json => output::print_json(&appointments, args.utc),
		OutputFormat::Table => output::print_table(&appointments, args.utc),
	}
}

fn handle_create(args: CreateArgs) -> anyhow::Result<()> {
	let cfg = config::load_config().context("Failed to load configuration")?;
	let (profile_name, profile) = config::resolve_profile(&cfg, args.association.as_deref())?;

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

/// Resolve a start datetime to a zoned ISO 8601 string (BR-009).
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

/// Normalize a date input to ISO 8601 date-time (BR-008).
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
		type_ids: args.type_ids.clone(),
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

	#[test]
	fn normalize_plain_date_from() {
		assert_eq!(normalize_datetime("2026-01-01", "00:00:00Z"), "2026-01-01T00:00:00Z");
	}

	#[test]
	fn normalize_plain_date_to() {
		assert_eq!(normalize_datetime("2026-12-31", "23:59:59Z"), "2026-12-31T23:59:59Z");
	}

	#[test]
	fn normalize_naive_datetime_appends_z() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00", "00:00:00Z"), "2026-01-01T14:00:00Z");
	}

	#[test]
	fn normalize_utc_datetime_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00Z", "00:00:00Z"), "2026-01-01T14:00:00Z");
	}

	#[test]
	fn normalize_positive_offset_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00+02:00", "00:00:00Z"), "2026-01-01T14:00:00+02:00");
	}

	#[test]
	fn normalize_negative_offset_unchanged() {
		assert_eq!(normalize_datetime("2026-01-01T14:00:00-05:00", "00:00:00Z"), "2026-01-01T14:00:00-05:00");
	}

	#[test]
	fn resolve_start_utc_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00Z").unwrap(), "2026-06-15T19:30:00Z");
	}

	#[test]
	fn resolve_start_positive_offset_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00+02:00").unwrap(), "2026-06-15T19:30:00+02:00");
	}

	#[test]
	fn resolve_start_negative_offset_passthrough() {
		assert_eq!(resolve_start_zoned("2026-06-15T19:30:00-05:00").unwrap(), "2026-06-15T19:30:00-05:00");
	}

	#[test]
	fn resolve_start_naive_adds_local_offset() {
		let result = resolve_start_zoned("2026-06-15T19:30:00").unwrap();
		// Must start with the same date/time and have an offset
		assert!(result.starts_with("2026-06-15T19:30:00"));
		assert!(result.contains('+') || result.contains('-'));
		assert!(!result.ends_with('Z'));
	}

	#[test]
	fn resolve_start_naive_short_format() {
		let result = resolve_start_zoned("2026-06-15T19:30").unwrap();
		assert!(result.starts_with("2026-06-15T19:30:00"));
	}

	#[test]
	fn resolve_start_date_only_rejected() {
		let result = resolve_start_zoned("2026-06-15");
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("requires a datetime"));
	}

	#[test]
	fn resolve_start_invalid_format_rejected() {
		let result = resolve_start_zoned("2026-06-15Tnonsense");
		assert!(result.is_err());
		assert!(result.unwrap_err().to_string().contains("Invalid datetime format"));
	}
}
