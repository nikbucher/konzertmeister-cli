# Test–Use Case Traceability Matrix

This matrix maps each use case scenario and business rule to its covering test(s).
Tests follow the naming convention documented in [CONTRIBUTING.md](../../CONTRIBUTING.md#testuse-case-traceability).

## UC-001: Configure Association Profile

| Scenario / Business Rule            | Test(s)                                                                                                                            | Status                                           |
|-------------------------------------|------------------------------------------------------------------------------------------------------------------------------------|--------------------------------------------------|
| Main Success Scenario               | `config::uc001_config_round_trip`                                                                                                  | ✅ Unit                                           |
| A1: Values Provided via Flags       | —                                                                                                                                  | ⬜ Needs integration test (stdin/filesystem)      |
| A2: Profile Already Exists          | `config::uc001_profile_overwrite`                                                                                                  | ✅ Unit                                           |
| A3: Set Default Profile             | `config::uc001_resolve_profile_default`                                                                                            | ✅ Unit                                           |
| A4: Default Profile Does Not Exist  | `config::uc001_resolve_profile_not_found`                                                                                          | ✅ Unit                                           |
| A5: No API Key Entered              | —                                                                                                                                  | ⬜ Needs integration test (stdin)                 |
| A6: No Creator Email Entered        | —                                                                                                                                  | ⬜ Needs integration test (stdin)                 |
| A7: Config Directory Does Not Exist | —                                                                                                                                  | ⬜ Needs integration test (filesystem)            |
| A8: Edit Configuration in Editor    | —                                                                                                                                  | ⬜ Needs integration test (subprocess)            |
| BR-001: API Key Security            | —                                                                                                                                  | ⬜ Needs integration test (stdout/stderr capture) |
| BR-002: Profile Name Uniqueness     | `config::uc001_profile_overwrite`                                                                                                  | ✅ Unit                                           |
| BR-003: Configuration Directory     | `config::uc001_config_path_ends_with_config_toml`                                                                                  | ✅ Unit                                           |
| BR-004: Association Selection       | `config::uc001_resolve_profile_explicit`, `config::uc001_resolve_profile_missing`, `config::uc001_missing_profile_lists_available` | ✅ Unit                                           |

## UC-002: List Appointments

| Scenario / Business Rule            | Test(s)                                                                                                                                                                                                                                                                                      | Status                                    |
|-------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|-------------------------------------------|
| Main Success Scenario               | `model::uc002_filter_input_skips_empty_fields`, `model::uc002_appointment_dto_deserializes`                                                                                                                                                                                                  | ✅ Unit                                    |
| A1: Filter by Date Range            | `tests::uc002_build_filter_from_to_switches_to_from_date`                                                                                                                                                                                                                                    | ✅ Unit                                    |
| A2: Filter by Type, Status, or Tags | `tests::uc002_build_filter_all_filter_options`                                                                                                                                                                                                                                               | ✅ Unit                                    |
| A3: Sort Results                    | `tests::uc002_build_filter_all_filter_options`                                                                                                                                                                                                                                               | ✅ Unit                                    |
| A4: Table Output                    | —                                                                                                                                                                                                                                                                                            | ⬜ Needs integration test (stdout capture) |
| A5: UTC Output                      | `output::uc002_format_datetime_utc_returns_raw`                                                                                                                                                                                                                                              | ✅ Unit                                    |
| A6: No Appointments Found           | `output::uc002_format_datetime_none_returns_empty`                                                                                                                                                                                                                                           | ✅ Unit (partial)                          |
| A7: API Error                       | —                                                                                                                                                                                                                                                                                            | ⬜ Needs integration test (HTTP mock)      |
| A8: Association Not Found           | via UC-001 BR-004                                                                                                                                                                                                                                                                            | ✅ Unit                                    |
| A9: No Association Configured       | via UC-001 BR-004                                                                                                                                                                                                                                                                            | ✅ Unit                                    |
| A10: Multiple Pages of Results      | —                                                                                                                                                                                                                                                                                            | ⬜ Needs integration test (HTTP mock)      |
| A11: Explicit Page Selection        | `tests::uc002_build_filter_explicit_page`                                                                                                                                                                                                                                                    | ✅ Unit                                    |
| BR-001: Default Date Mode           | `tests::uc002_build_filter_defaults_to_upcoming`                                                                                                                                                                                                                                             | ✅ Unit                                    |
| BR-002: Output Format Default       | `tests::uc002_output_format_defaults_to_json`                                                                                                                                                                                                                                                | ✅ Unit                                    |
| BR-003: Auto-Pagination             | —                                                                                                                                                                                                                                                                                            | ⬜ Needs integration test (HTTP mock)      |
| BR-004: Timezone Display Default    | `output::uc002_convert_utc_to_zurich`, `output::uc002_convert_unknown_timezone_returns_none`, `output::uc002_convert_no_timezone_returns_none`                                                                                                                                               | ✅ Unit                                    |
| BR-005: Date Input Normalization    | `tests::uc002_normalize_plain_date_from`, `tests::uc002_normalize_plain_date_to`, `tests::uc002_normalize_naive_datetime_appends_z`, `tests::uc002_normalize_utc_datetime_unchanged`, `tests::uc002_normalize_positive_offset_unchanged`, `tests::uc002_normalize_negative_offset_unchanged` | ✅ Unit                                    |

## UC-003: Create Appointment

| Scenario / Business Rule          | Test(s)                                                                                                                                                                                                      | Status                                         |
|-----------------------------------|--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|------------------------------------------------|
| Main Success Scenario             | `model::uc003_create_input_serializes_camel_case`                                                                                                                                                            | ✅ Unit                                         |
| A1: Zoned Datetime Provided       | `tests::uc003_resolve_start_utc_passthrough`, `tests::uc003_resolve_start_positive_offset_passthrough`, `tests::uc003_resolve_start_negative_offset_passthrough`                                             | ✅ Unit                                         |
| A2: Override Name                 | `model::uc003_create_input_serializes_camel_case`                                                                                                                                                            | ✅ Unit                                         |
| A3: Override Description          | `model::uc003_create_input_with_description`                                                                                                                                                                 | ✅ Unit                                         |
| A4: Dry Run                       | —                                                                                                                                                                                                            | ⬜ Needs integration test (filesystem + stdout) |
| A5: API Error                     | —                                                                                                                                                                                                            | ⬜ Needs integration test (HTTP mock)           |
| A6: Creator Email Not Configured  | —                                                                                                                                                                                                            | ⬜ Needs integration test (filesystem)          |
| A7: Association Not Found         | via UC-001 BR-004                                                                                                                                                                                            | ✅ Unit                                         |
| A8: No Association Configured     | via UC-001 BR-004                                                                                                                                                                                            | ✅ Unit                                         |
| A9: Missing Required Flags        | `tests::uc003_missing_template_rejected`, `tests::uc003_missing_start_rejected`                                                                                                                              | ✅ Unit                                         |
| BR-001: Naive Datetime Resolution | `tests::uc003_resolve_start_naive_adds_local_offset`, `tests::uc003_resolve_start_naive_short_format`, `tests::uc003_resolve_start_date_only_rejected`, `tests::uc003_resolve_start_invalid_format_rejected` | ✅ Unit                                         |
| BR-002: Template External ID      | `model::uc003_create_input_serializes_camel_case`                                                                                                                                                            | ✅ Unit                                         |
| BR-003: Creator Email Source      | `model::uc003_create_input_serializes_camel_case`                                                                                                                                                            | ✅ Unit                                         |
| BR-004: Dry Run Output            | —                                                                                                                                                                                                            | ⬜ Needs integration test (filesystem + stdout) |

## UC-005: List Members

| Scenario / Business Rule | Test(s) | Status |
|--------------------------|---------|--------|
| Main Success Scenario | `model::uc005_member_dto_deserializes_properties`, `tests::uc005_member_list_command_parses` | ✅ Unit |
| A1: Select Association | via UC-001 BR-004 | ✅ Unit |
| A2: Filter by Email | `tests::uc005_mail_filter_ignores_case_and_can_return_empty` | ✅ Unit |
| A3: Table Output | — | ⬜ Needs output capture |
| A4: No Members Match | `tests::uc005_mail_filter_ignores_case_and_can_return_empty` | ✅ Unit |
| A5: Association Unavailable | via UC-001 BR-004 | ✅ Unit |
| A6: Service Error | — | ⬜ Needs HTTP mock |
| BR-001: Member List Source | — | ⬜ Needs HTTP mock |
| BR-002: Output Format | `model::uc005_member_dto_deserializes_properties` | ✅ Unit (JSON fields); table needs output capture |

## UC-006: Add Member

| Scenario / Business Rule | Test(s) | Status |
|--------------------------|---------|--------|
| Main Success Scenario | `model::uc006_member_input_serializes_typed_fields`, `tests::uc006_member_add_command_parses` | ✅ Unit (input); live response pending |
| A1: Person Has No Account | — | ⬜ Needs live API check |
| A2: Set Data Fields | `tests::uc006_member_property_parser` | ✅ Unit |
| A3: Preview Request | `cli::uc006_dry_run_prints_typed_json_without_config` | ✅ Integration |
| A4: Invalid Input | `tests::uc006_invalid_properties_are_rejected`, `cli::uc006_invalid_number_reports_error` | ✅ Unit and integration |
| A5: Association Unavailable | via UC-001 BR-004 | ✅ Unit |
| A6: Service Error | — | ⬜ Needs HTTP mock |
| BR-001: Required Email and Invitation | `tests::uc006_member_add_command_parses` | ✅ Unit (email); invitation needs live API check |
| BR-002: Data Fields | `model::uc006_member_input_serializes_typed_fields`, `tests::uc006_member_property_parser`, `tests::uc006_invalid_properties_are_rejected` | ✅ Unit |
| BR-003: Preview and Success Output | `cli::uc006_dry_run_prints_typed_json_without_config` | ✅ Integration (preview); live success pending |

## UC-007: Update Member

| Scenario / Business Rule | Test(s) | Status |
|--------------------------|---------|--------|
| Main Success Scenario | `tests::uc007_member_update_typed_properties` | ✅ Unit (input); live response pending |
| A1: Change Data Fields | `tests::uc007_member_update_typed_properties` | ✅ Unit |
| A2: Preview Request | — | ⬜ Needs stdout capture |
| A3: Nothing to Update | `tests::uc007_update_requires_a_change`, `cli::uc007_empty_update_reports_error` | ✅ Unit and integration |
| A4: Name Restricted by Account State | — | ⬜ Needs live API check |
| A5: Member or Association Unavailable | via UC-001 BR-004 (profile) | ⬜ Member lookup needs live API check |
| A6: Invalid Data Field or Service Error | `tests::uc006_invalid_properties_are_rejected` | ✅ Unit (input); service error needs HTTP mock |
| BR-001: Member Identity and Change Requirement | `tests::uc007_update_requires_a_change` | ✅ Unit |
| BR-002: Account Name Restriction | — | ⬜ Needs live API check |
| BR-003: Data Fields and Preview | `tests::uc007_member_update_typed_properties` | ✅ Unit (fields); output needs capture |

## UC-008: List Replies

| Scenario / Business Rule | Test(s) | Status |
|--------------------------|---------|--------|
| Main Success Scenario | `model::uc008_reply_dto_deserializes` | ✅ Unit (response); live request pending |
| A1: Filter by Reply | `tests::uc008_reply_filter` | ✅ Unit |
| A2: Table Output | — | ⬜ Needs output capture |
| A3: No Replies Match | `tests::uc008_reply_filter` | ✅ Unit (filter) |
| A4: Unknown Reply Value | `model::uc008_unknown_reply_deserializes` | ✅ Unit |
| A5: Appointment or Association Unavailable | via UC-001 BR-004 (profile) | ⬜ Appointment error needs HTTP mock |
| A6: Service Error | — | ⬜ Needs HTTP mock |
| BR-001: Reply Values and Source | `model::uc008_reply_dto_deserializes`, `model::uc008_unknown_reply_deserializes` | ✅ Unit (values); endpoint needs HTTP mock |
| BR-002: Filtering and Display | `tests::uc008_reply_filter` | ✅ Unit (filter); output needs capture |

## UC-009: List Attendances

| Scenario / Business Rule | Test(s) | Status |
|--------------------------|---------|--------|
| Main Success Scenario | `model::uc009_attendance_dto_deserializes` | ✅ Unit (response); live request pending |
| A1: Show Present Members | `tests::uc009_attendance_filter_and_exclusive_flags` | ✅ Unit |
| A2: Show Absent Members | `tests::uc009_attendance_filter_and_exclusive_flags` | ✅ Unit |
| A3: Table Output | — | ⬜ Needs output capture |
| A4: No Attendance Matches | `tests::uc009_attendance_filter_and_exclusive_flags` | ✅ Unit (filter) |
| A5: Conflicting Filters | `tests::uc009_attendance_filter_and_exclusive_flags` | ✅ Unit |
| A6: Appointment or Service Unavailable | — | ⬜ Needs HTTP mock |
| BR-001: Attendance Source | — | ⬜ Needs HTTP mock |
| BR-002: Filter Semantics | `tests::uc009_attendance_filter_and_exclusive_flags` | ✅ Unit |
