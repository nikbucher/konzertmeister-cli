use std::time::Instant;

use anyhow::{Context, bail};
use log::debug;
use serde::{Serialize, de::DeserializeOwned};
use ureq::{Agent, config::Config};

use crate::model::{AppointmentDto, AppointmentFilterInput, CreateAppointmentInput, MemberDto, MemberInput, ReplyDto};

const BASE_URL: &str = "https://rest.konzertmeister.app";
const APPOINTMENTS_PATH: &str = "/api/v4/org/m2m/appointments";
const CREATE_PATH: &str = "/api/v4/app/m2m/create";
const PAGE_SIZE: usize = 10;

fn agent() -> Agent {
	Config::builder().http_status_as_error(false).build().into()
}

fn check_response(response: &mut ureq::http::Response<ureq::Body>, start: Instant) -> anyhow::Result<()> {
	let status = response.status().as_u16();
	debug!("Response: HTTP {} in {:.3}s", status, start.elapsed().as_secs_f64());
	if status != 200 {
		let body = response.body_mut().read_to_string().unwrap_or_else(|_| "<could not read response body>".to_string());
		bail!("API returned HTTP {} — {}", status, body);
	}
	Ok(())
}

fn get_json<T: DeserializeOwned>(agent: &Agent, api_key: &str, path: &str) -> anyhow::Result<T> {
	let url = format!("{BASE_URL}{path}");
	debug!("GET {url}");
	let start = Instant::now();
	let mut response = agent.get(&url).header("X-KM-ORG-API-KEY", api_key).call().context("Failed to send request to Konzertmeister API")?;
	check_response(&mut response, start)?;
	response.body_mut().read_json().context("Failed to parse API response")
}

fn post_response<B: Serialize>(agent: &Agent, api_key: &str, path: &str, input: &B) -> anyhow::Result<ureq::http::Response<ureq::Body>> {
	let url = format!("{BASE_URL}{path}");
	let body = serde_json::to_string(input).context("Failed to serialize request")?;
	debug!("POST {} — body: {}", url, body);
	let start = Instant::now();
	let mut response = agent
		.post(&url)
		.header("X-KM-ORG-API-KEY", api_key)
		.header("Content-Type", "application/json")
		.send(&body)
		.context("Failed to send request to Konzertmeister API")?;
	check_response(&mut response, start)?;
	Ok(response)
}

fn post_json<B: Serialize, T: DeserializeOwned>(agent: &Agent, api_key: &str, path: &str, input: &B) -> anyhow::Result<T> {
	let mut response = post_response(agent, api_key, path, input)?;
	response.body_mut().read_json().context("Failed to parse API response")
}

fn post_no_content<B: Serialize>(api_key: &str, path: &str, input: &B) -> anyhow::Result<()> {
	post_response(&agent(), api_key, path, input)?;
	Ok(())
}

pub fn list_appointments(api_key: &str, filter: &AppointmentFilterInput) -> anyhow::Result<Vec<AppointmentDto>> {
	let agent = agent();
	if filter.page.is_some() {
		return post_json(&agent, api_key, APPOINTMENTS_PATH, filter);
	}
	let mut all = Vec::new();
	let mut page = 0i32;
	loop {
		let page_filter = AppointmentFilterInput { page: Some(page), ..filter.clone() };
		let results: Vec<AppointmentDto> = post_json(&agent, api_key, APPOINTMENTS_PATH, &page_filter)?;
		let count = results.len();
		all.extend(results);
		if count < PAGE_SIZE {
			break;
		}
		page += 1;
	}
	Ok(all)
}

pub fn create_appointment(api_key: &str, input: &CreateAppointmentInput) -> anyhow::Result<AppointmentDto> {
	post_json(&agent(), api_key, CREATE_PATH, input)
}

pub fn list_members(api_key: &str) -> anyhow::Result<Vec<MemberDto>> {
	get_json(&agent(), api_key, "/api/v4/org/m2m/members")
}

pub fn add_member(api_key: &str, input: &MemberInput) -> anyhow::Result<()> {
	post_no_content(api_key, "/api/v4/org/m2m/addmember", input)
}

pub fn update_member(api_key: &str, input: &MemberInput) -> anyhow::Result<()> {
	post_no_content(api_key, "/api/v4/org/m2m/updatemember", input)
}

pub fn list_replies(api_key: &str, app_id: i64) -> anyhow::Result<Vec<ReplyDto>> {
	get_json(&agent(), api_key, &format!("/api/v4/att/m2m/{app_id}"))
}
