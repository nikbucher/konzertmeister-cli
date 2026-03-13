use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "km", about = "Konzertmeister CLI — manage appointments for music associations")]
pub struct Cli {
	#[command(subcommand)]
	pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
	/// Manage association profiles
	Config {
		#[command(subcommand)]
		action: ConfigAction,
	},
	/// List appointments
	List(ListArgs),
	/// Create an appointment from a template
	Create(CreateArgs),
}

#[derive(Subcommand)]
pub enum ConfigAction {
	/// Create or update an association profile
	Set {
		/// Profile name
		name: String,
		/// API key (omit to be prompted)
		#[arg(long)]
		api_key: Option<String>,
		/// Creator email (omit to be prompted)
		#[arg(long)]
		creator_mail: Option<String>,
	},
	/// Set the default profile
	Default {
		/// Profile name to set as default
		name: String,
	},
	/// Open config file in $EDITOR
	Edit,
	/// Print the config file path
	Path,
}

#[derive(clap::Args)]
pub struct ListArgs {
	/// Association profile to use (overrides default)
	#[arg(long)]
	pub association: Option<String>,

	/// Filter start date (ISO 8601, e.g. 2026-01-01)
	#[arg(long)]
	pub from: Option<String>,

	/// Filter end date (ISO 8601, e.g. 2026-12-31)
	#[arg(long)]
	pub to: Option<String>,

	/// Filter by appointment type ID (repeatable)
	#[arg(long = "type", action = clap::ArgAction::Append)]
	pub type_ids: Vec<i32>,

	/// Show only active appointments
	#[arg(long)]
	pub active: bool,

	/// Show only cancelled appointments
	#[arg(long)]
	pub cancelled: bool,

	/// Show only published appointments
	#[arg(long)]
	pub published: bool,

	/// Show only unpublished appointments
	#[arg(long)]
	pub unpublished: bool,

	/// Sort mode
	#[arg(long, value_enum)]
	pub sort: Option<SortMode>,

	/// Output format
	#[arg(long, value_enum, default_value = "json")]
	pub format: OutputFormat,

	/// Display times in UTC instead of local timezone
	#[arg(long)]
	pub utc: bool,

	/// Fetch only this page (disables auto-pagination)
	#[arg(long)]
	pub page: Option<i32>,

	/// Filter by tag (repeatable)
	#[arg(long, action = clap::ArgAction::Append)]
	pub tag: Vec<String>,
}

#[derive(clap::Args)]
pub struct CreateArgs {
	/// Association profile to use (overrides default)
	#[arg(long)]
	pub association: Option<String>,

	/// Appointment template external ID (from Konzertmeister web UI)
	#[arg(long)]
	pub template: String,

	/// Start datetime (naive → local TZ, or provide offset/Z for explicit TZ)
	#[arg(long)]
	pub start: String,

	/// Override the appointment name from the template
	#[arg(long)]
	pub name: Option<String>,

	/// Override the appointment description from the template
	#[arg(long)]
	pub description: Option<String>,

	/// Show the request payload without sending it
	#[arg(long)]
	pub dry_run: bool,
}

#[derive(ValueEnum, Clone)]
pub enum SortMode {
	Startdate,
	Deadline,
}

#[derive(ValueEnum, Clone, PartialEq)]
pub enum OutputFormat {
	Json,
	Table,
}
