use std::time::Instant;

use anyhow::{Context, bail};
use log::debug;
use ureq::Agent;
use ureq::config::Config;

use crate::model::{AppointmentDto, AppointmentFilterInput, CreateAppointmentInput};

const BASE_URL: &str = "https://rest.konzertmeister.app";
const APPOINTMENTS_PATH: &str = "/api/v4/org/m2m/appointments";
const CREATE_PATH: &str = "/api/v4/app/m2m/create";
const PAGE_SIZE: usize = 10;

fn agent() -> Agent {
	Config::builder().http_status_as_error(false).build().into()
}

pub fn list_appointments(api_key: &str, filter: &AppointmentFilterInput) -> anyhow::Result<Vec<AppointmentDto>> {
	if filter.page.is_some() {
		return fetch_page(&agent(), api_key, filter);
	}

	let agent = agent();
	let mut all = Vec::new();
	let mut page = 0i32;

	loop {
		let page_filter = AppointmentFilterInput {
			filter_start: filter.filter_start.clone(),
			filter_end: filter.filter_end.clone(),
			type_ids: filter.type_ids.clone(),
			activation_status_list: filter.activation_status_list.clone(),
			published_status: filter.published_status.clone(),
			tags: filter.tags.clone(),
			sort_mode: filter.sort_mode.clone(),
			date_mode: filter.date_mode.clone(),
			page: Some(page),
		};

		let results = fetch_page(&agent, api_key, &page_filter)?;
		let count = results.len();
		all.extend(results);

		if count < PAGE_SIZE {
			break;
		}
		page += 1;
	}

	Ok(all)
}

fn fetch_page(agent: &Agent, api_key: &str, filter: &AppointmentFilterInput) -> anyhow::Result<Vec<AppointmentDto>> {
	let url = format!("{}{}", BASE_URL, APPOINTMENTS_PATH);

	let body = serde_json::to_string(filter).context("Failed to serialize filter")?;
	debug!("POST {} — body: {}", url, body);

	let start = Instant::now();
	let mut response = agent
		.post(&url)
		.header("X-KM-ORG-API-KEY", api_key)
		.header("Content-Type", "application/json")
		.send(&body)
		.context("Failed to send request to Konzertmeister API")?;
	let elapsed = start.elapsed();

	let status = response.status().as_u16();
	debug!("Response: HTTP {} in {:.3}s", status, elapsed.as_secs_f64());

	if status != 200 {
		let body = response.body_mut().read_to_string().unwrap_or_else(|_| "<could not read response body>".to_string());
		bail!("API returned HTTP {} — {}", status, body);
	}

	let appointments: Vec<AppointmentDto> = response.body_mut().read_json().context("Failed to parse API response")?;
	debug!("Received {} appointments", appointments.len());
	Ok(appointments)
}

pub fn create_appointment(api_key: &str, input: &CreateAppointmentInput) -> anyhow::Result<AppointmentDto> {
	let url = format!("{}{}", BASE_URL, CREATE_PATH);

	let body = serde_json::to_string(input).context("Failed to serialize create input")?;
	debug!("POST {} — body: {}", url, body);

	let agent = agent();
	let start = Instant::now();
	let mut response = agent
		.post(&url)
		.header("X-KM-ORG-API-KEY", api_key)
		.header("Content-Type", "application/json")
		.send(&body)
		.context("Failed to send request to Konzertmeister API")?;
	let elapsed = start.elapsed();

	let status = response.status().as_u16();
	debug!("Response: HTTP {} in {:.3}s", status, elapsed.as_secs_f64());

	if status != 200 {
		let body = response.body_mut().read_to_string().unwrap_or_else(|_| "<could not read response body>".to_string());
		bail!("API returned HTTP {} — {}", status, body);
	}

	let appointment: AppointmentDto = response.body_mut().read_json().context("Failed to parse API response")?;
	debug!("Created appointment: id={:?}, name={:?}", appointment.id, appointment.name);
	Ok(appointment)
}
