use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
    pub status: reqwest::StatusCode,
    pub content: String,
    pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
    ResponseError(ResponseContent<T>),
}

impl <T> fmt::Display for Error<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (module, e) = match self {
            Error::Reqwest(e) => ("reqwest", e.to_string()),
            Error::Serde(e) => ("serde", e.to_string()),
            Error::Io(e) => ("IO", e.to_string()),
            Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
        };
        write!(f, "error in {}: {}", module, e)
    }
}

impl <T: fmt::Debug> error::Error for Error<T> {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(match self {
            Error::Reqwest(e) => e,
            Error::Serde(e) => e,
            Error::Io(e) => e,
            Error::ResponseError(_) => return None,
        })
    }
}

impl <T> From<reqwest::Error> for Error<T> {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl <T> From<serde_json::Error> for Error<T> {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl <T> From<std::io::Error> for Error<T> {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
    ::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
    if let serde_json::Value::Object(object) = value {
        let mut params = vec![];

        for (key, value) in object {
            match value {
                serde_json::Value::Object(_) => params.append(&mut parse_deep_object(
                    &format!("{}[{}]", prefix, key),
                    value,
                )),
                serde_json::Value::Array(array) => {
                    for (i, value) in array.iter().enumerate() {
                        params.append(&mut parse_deep_object(
                            &format!("{}[{}][{}]", prefix, key, i),
                            value,
                        ));
                    }
                },
                serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
                _ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
            }
        }

        return params;
    }

    unimplemented!("Only objects are supported with style=deepObject")
}

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
    Json,
    Text,
    Unsupported(String)
}

impl From<&str> for ContentType {
    fn from(content_type: &str) -> Self {
        if content_type.starts_with("application") && content_type.contains("json") {
            return Self::Json;
        } else if content_type.starts_with("text/plain") {
            return Self::Text;
        } else {
            return Self::Unsupported(content_type.to_string());
        }
    }
}

pub mod account_calendars_api_api;
pub mod account_notifications_api;
pub mod account_reports_api;
pub mod accounts_api;
pub mod admins_api;
pub mod ai_conversations_api;
pub mod ai_experiences_api;
pub mod announcements_api_api;
pub mod anonymous_provisional_grades_api;
pub mod appointment_groups_api;
pub mod assessment_question_banks_api;
pub mod assignment_extensions_api;
pub mod assignment_groups_api;
pub mod assignment_groups_api_api;
pub mod assignment_overrides_api;
pub mod assignments_api;
pub mod assignments_api_api;
pub mod authentication_audit_api_api;
pub mod authentication_providers_api;
pub mod bad_permission_setting_error_api;
pub mod blackout_dates_api;
pub mod block_editor_templates_api_api;
pub mod brand_configs_api_api;
pub mod calendar_events_api_api;
pub mod canvadoc_sessions_api;
pub mod career_experience_api;
pub mod comm_messages_api_api;
pub mod communication_channels_api;
pub mod conferences_api;
pub mod content_exports_api_api;
pub mod content_imports_api;
pub mod content_migrations_api;
pub mod content_shares_api;
pub mod context_module_items_api_api;
pub mod context_modules_api_api;
pub mod conversations_api;
pub mod course_audit_api_api;
pub mod course_nicknames_api;
pub mod course_paces_api;
pub mod course_reports_api;
pub mod courses_api;
pub mod csp_settings_api;
pub mod custom_data_api;
pub mod custom_gradebook_column_data_api_api;
pub mod custom_gradebook_columns_api_api;
pub mod developer_key_account_bindings_api;
pub mod developer_keys_api;
pub mod disable_post_to_sis_api_api;
pub mod discovery_pages_api_api;
pub mod discussion_entries_api;
pub mod discussion_topics_api;
pub mod discussion_topics_api_api;
pub mod enrollments_api_api;
pub mod eportfolios_api_api;
pub mod epub_exports_api;
pub mod errors_api;
pub mod external_feeds_api;
pub mod favorites_api;
pub mod feature_flags_api;
pub mod folders_api;
pub mod grade_change_audit_api_api;
pub mod gradebook_history_api_api;
pub mod gradebooks_api;
pub mod grading_period_sets_api;
pub mod grading_periods_api;
pub mod grading_standards_api_api;
pub mod group_categories_api;
pub mod group_memberships_api;
pub mod groups_api;
pub mod history_api;
pub mod inst_access_tokens_api;
pub mod invalid_content_type_api;
pub mod jwts_api;
pub mod learning_mastery_gradebook_settings_api_api;
pub mod learning_object_dates_api;
pub mod lmgb_user_details_api;
pub mod media_objects_api;
pub mod media_tracks_api;
pub mod metadata_sax_doc_api;
pub mod migration_issues_api;
pub mod moderation_set_api;
pub mod module_assignment_overrides_api;
pub mod no_compatible_tool_api;
pub mod notification_preferences_api;
pub mod observer_pairing_codes_api_api;
pub mod outcome_groups_api_api;
pub mod outcome_proficiency_api_api;
pub mod outcome_results_api;
pub mod outcomes_api_api;
pub mod page_views_api;
pub mod peer_reviews_api_api;
pub mod permissions_help_api;
pub mod planner_api;
pub mod planner_notes_api;
pub mod planner_overrides_api;
pub mod profile_api;
pub mod progress_api;
pub mod provisional_grades_api;
pub mod pseudonyms_api;
pub mod record_already_exists_api;
pub mod rich_content_api_api;
pub mod rubric_assessments_api;
pub mod rubric_associations_api;
pub mod rubrics_api;
pub mod rubrics_api_api;
pub mod scopes_api_api;
pub mod search_api;
pub mod sections_api;
pub mod service_credentials_api;
pub mod services_api_api;
pub mod shared_brand_configs_api;
pub mod sis_api_api;
pub mod sis_import_errors_api_api;
pub mod sis_imports_api_api;
pub mod smart_search_api;
pub mod sub_accounts_api;
pub mod submission_comments_api_api;
pub mod submissions_api;
pub mod submissions_api_api;
pub mod syllabus_api_api;
pub mod tabs_api;
pub mod temporary_enrollment_pairings_api_api;
pub mod terms_api;
pub mod terms_api_api;
pub mod timing_meta_api;
pub mod tokens_api;
pub mod usage_rights_api;
pub mod user_observees_api;
pub mod web_zip_exports_api;
pub mod what_if_grades_api_api;
pub mod wiki_pages_api_api;

pub mod configuration;
