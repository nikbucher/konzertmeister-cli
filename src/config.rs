use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, bail};

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct KmConfig {
	pub default: Option<String>,
	#[serde(default)]
	pub profiles: BTreeMap<String, Profile>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct Profile {
	pub api_key: String,
	pub creator_mail: Option<String>,
}

pub fn config_path() -> anyhow::Result<PathBuf> {
	let proj = directories::ProjectDirs::from("", "", "km").context("Could not determine config directory")?;
	Ok(proj.config_dir().join("config.toml"))
}

pub fn load_config() -> anyhow::Result<KmConfig> {
	let path = config_path()?;
	if !path.exists() {
		return Ok(KmConfig::default());
	}
	let content = std::fs::read_to_string(&path).with_context(|| format!("Failed to read config at {}", path.display()))?;
	toml::from_str(&content).with_context(|| format!("Failed to parse config at {}", path.display()))
}

pub fn save_config(config: &KmConfig) -> anyhow::Result<()> {
	let path = config_path()?;
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent).with_context(|| format!("Failed to create config directory {}", parent.display()))?;
	}
	let content = toml::to_string_pretty(config).context("Failed to serialize config")?;
	std::fs::write(&path, &content).with_context(|| format!("Failed to write config to {}", path.display()))?;

	#[cfg(unix)]
	{
		use std::os::unix::fs::PermissionsExt;
		std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).with_context(|| "Failed to set config file permissions")?;
	}

	Ok(())
}

pub fn resolve_profile<'a>(config: &'a KmConfig, association: Option<&'a str>) -> anyhow::Result<(&'a str, &'a Profile)> {
	let name = match association {
		Some(name) => name,
		None => match &config.default {
			Some(default) => default.as_str(),
			None => {
				let available = format_available_profiles(config);
				bail!("No association specified. Use --association or set a default with 'km config default'.{}", available);
			}
		},
	};

	match config.profiles.get(name) {
		Some(profile) => Ok((name, profile)),
		None => {
			let available = format_available_profiles(config);
			bail!("Profile '{}' not found.{}", name, available);
		}
	}
}

fn format_available_profiles(config: &KmConfig) -> String {
	if config.profiles.is_empty() {
		return String::new();
	}
	let names: Vec<&str> = config.profiles.keys().map(|s| s.as_str()).collect();
	format!("\nAvailable profiles: {}", names.join(", "))
}

pub fn handle_set(name: &str, api_key: Option<&str>, creator_mail: Option<&str>) -> anyhow::Result<()> {
	let api_key = match api_key {
		Some(key) => key.to_string(),
		None => {
			let key = rpassword::prompt_password("API key: ").context("Failed to read API key")?;
			if key.is_empty() {
				bail!("API key must not be empty.");
			}
			key
		}
	};

	if api_key.is_empty() {
		bail!("API key must not be empty.");
	}

	let creator_mail = match creator_mail {
		Some(mail) => {
			if mail.is_empty() {
				eprintln!("Warning: Creator email not set. Appointment creation will not be possible with this profile.");
				None
			} else {
				Some(mail.to_string())
			}
		}
		None => {
			print!("Creator email: ");
			io::stdout().flush()?;
			let mut mail = String::new();
			io::stdin().read_line(&mut mail)?;
			let mail = mail.trim().to_string();
			if mail.is_empty() {
				eprintln!("Warning: Creator email not set. Appointment creation will not be possible with this profile.");
				None
			} else {
				Some(mail)
			}
		}
	};

	let mut config = load_config()?;
	config.profiles.insert(name.to_string(), Profile { api_key, creator_mail });
	save_config(&config)?;
	eprintln!("Profile '{}' saved.", name);
	Ok(())
}

pub fn handle_default(name: &str) -> anyhow::Result<()> {
	let mut config = load_config()?;
	if !config.profiles.contains_key(name) {
		let available = format_available_profiles(&config);
		bail!("Profile '{}' not found.{}", name, available);
	}
	config.default = Some(name.to_string());
	save_config(&config)?;
	eprintln!("Default profile set to '{}'.", name);
	Ok(())
}

pub fn handle_edit() -> anyhow::Result<()> {
	let path = config_path()?;

	if !path.exists() {
		if let Some(parent) = path.parent() {
			std::fs::create_dir_all(parent)?;
		}
		let template = r#"# km configuration
# default = "my-association"
#
# [profiles.my-association]
# api_key = "your-api-key"
# creator_mail = "admin@example.com"
"#;
		std::fs::write(&path, template)?;

		#[cfg(unix)]
		{
			use std::os::unix::fs::PermissionsExt;
			std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
		}
	}

	let editor = std::env::var("EDITOR").or_else(|_| std::env::var("VISUAL")).unwrap_or_else(|_| "vim".to_string());

	let status = std::process::Command::new(&editor).arg(&path).status().with_context(|| format!("Failed to open editor '{}'", editor))?;

	if !status.success() {
		bail!("Editor exited with non-zero status");
	}
	Ok(())
}

pub fn handle_path() -> anyhow::Result<()> {
	println!("{}", config_path()?.display());
	Ok(())
}

#[cfg(test)]
mod tests {
	use super::*;

	/// UC-001 | Main Success Scenario
	#[test]
	fn uc001_config_round_trip() {
		let config = KmConfig {
			default: Some("test".to_string()),
			profiles: BTreeMap::from([(
				"test".to_string(),
				Profile {
					api_key: "secret-key".to_string(),
					creator_mail: Some("test@example.com".to_string()),
				},
			)]),
		};

		let serialized = toml::to_string_pretty(&config).unwrap();
		let deserialized: KmConfig = toml::from_str(&serialized).unwrap();

		assert_eq!(deserialized.default.as_deref(), Some("test"));
		assert_eq!(deserialized.profiles["test"].api_key, "secret-key");
		assert_eq!(deserialized.profiles["test"].creator_mail.as_deref(), Some("test@example.com"));
	}

	/// UC-001 | BR-004: Association Selection
	#[test]
	fn uc001_resolve_profile_explicit() {
		let config = KmConfig {
			default: None,
			profiles: BTreeMap::from([(
				"myband".to_string(),
				Profile {
					api_key: "key".to_string(),
					creator_mail: None,
				},
			)]),
		};

		let (name, profile) = resolve_profile(&config, Some("myband")).unwrap();
		assert_eq!(name, "myband");
		assert_eq!(profile.api_key, "key");
	}

	/// UC-001 | A3: Set Default Profile
	#[test]
	fn uc001_resolve_profile_default() {
		let config = KmConfig {
			default: Some("myband".to_string()),
			profiles: BTreeMap::from([(
				"myband".to_string(),
				Profile {
					api_key: "key".to_string(),
					creator_mail: None,
				},
			)]),
		};

		let (name, _) = resolve_profile(&config, None).unwrap();
		assert_eq!(name, "myband");
	}

	/// UC-001 | BR-004: Association Selection
	#[test]
	fn uc001_resolve_profile_missing() {
		let config = KmConfig {
			default: None,
			profiles: BTreeMap::new(),
		};

		let err = resolve_profile(&config, None).unwrap_err();
		assert!(err.to_string().contains("No association specified"));
	}

	/// UC-001 | A4: Default Profile Does Not Exist
	#[test]
	fn uc001_resolve_profile_not_found() {
		let config = KmConfig {
			default: None,
			profiles: BTreeMap::from([(
				"other".to_string(),
				Profile {
					api_key: "key".to_string(),
					creator_mail: None,
				},
			)]),
		};

		let err = resolve_profile(&config, Some("missing")).unwrap_err();
		assert!(err.to_string().contains("not found"));
		assert!(err.to_string().contains("other"));
	}
}
