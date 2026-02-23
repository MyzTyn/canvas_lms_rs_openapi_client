# OriginalityReport

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the OriginalityReport | [optional]
**file_id** | Option<**i32**> | The id of the file receiving the originality score | [optional]
**originality_score** | Option<**f64**> | A number between 0 and 100 representing the originality score | [optional]
**originality_report_file_id** | Option<**i32**> | The ID of the file within Canvas containing the originality report document (if provided) | [optional]
**originality_report_url** | Option<**String**> | A non-LTI launch URL where the originality score of the file may be found. | [optional]
**tool_setting** | Option<[**models::ToolSetting**](ToolSetting.md)> |  | [optional]
**error_report** | Option<**String**> | A message describing the error. If set, the workflow_state will become 'error.' | [optional]
**submission_time** | Option<**String**> | The submitted_at date time of the submission. | [optional]
**root_account_id** | Option<**i32**> | The id of the root Account associated with the OriginalityReport | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


