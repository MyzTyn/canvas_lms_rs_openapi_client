# Conference

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the conference | [optional]
**conference_type** | Option<**String**> | The type of conference | [optional]
**conference_key** | Option<**String**> | The 3rd party's ID for the conference | [optional]
**description** | Option<**String**> | The description for the conference | [optional]
**duration** | Option<**i32**> | The expected duration the conference is supposed to last | [optional]
**ended_at** | Option<**String**> | The date that the conference ended at, null if it hasn't ended | [optional]
**started_at** | Option<**String**> | The date the conference started at, null if it hasn't started | [optional]
**title** | Option<**String**> | The title of the conference | [optional]
**users** | Option<**Vec<i32>**> | Array of user ids that are participants in the conference | [optional]
**invitees** | Option<**Vec<i32>**> | Array of user ids that are invitees in the conference | [optional]
**attendees** | Option<**Vec<i32>**> | Array of user ids that are attendees in the conference | [optional]
**has_advanced_settings** | Option<**bool**> | True if the conference type has advanced settings. | [optional]
**long_running** | Option<**bool**> | If true the conference is long running and has no expected end time | [optional]
**user_settings** | Option<**serde_json::Value**> | A collection of settings specific to the conference type | [optional]
**recordings** | Option<[**Vec<models::ConferenceRecording>**](ConferenceRecording.md)> | A List of recordings for the conference | [optional]
**url** | Option<**String**> | URL for the conference, may be null if the conference type doesn't set it | [optional]
**join_url** | Option<**String**> | URL to join the conference, may be null if the conference type doesn't set it | [optional]
**context_type** | Option<**String**> | The type of this conference's context, typically 'Course' or 'Group'. | [optional]
**context_id** | Option<**i32**> | The ID of this conference's context. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


