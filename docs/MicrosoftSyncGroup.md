# MicrosoftSyncGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the MicrosoftSync_Group | [optional]
**course_id** | Option<**i32**> | The id of the course related to the MicrosoftSync_Group | [optional]
**workflow_state** | Option<**String**> | The current state of the MicrosoftSync_Group | [optional]
**job_state** | Option<**String**> | Internal data about the last step run for a job in the 'retrying' state. Only returned for site admins | [optional]
**last_synced_at** | Option<**String**> | The time of the last successful sync | [optional]
**last_error** | Option<**String**> | The last error encountered during an attempted sync | [optional]
**last_error_report_id** | Option<**i32**> | The ErrorReport ID for the last_error. Only returned for site admins | [optional]
**root_account_id** | Option<**i32**> | The root account the MicrosoftSync_Group belongs to | [optional]
**created_at** | Option<**String**> | The time the MicrosoftSync_Group was created | [optional]
**updated_at** | Option<**String**> | The time the MicrosoftSync_Group was updated | [optional]
**debug_info** | Option<**Vec<serde_json::Value>**> | List of strings with debugging info (localized). Only returned for site admins. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


