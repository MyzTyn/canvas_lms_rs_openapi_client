# MigrationIssue

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the unique identifier for the issue | [optional]
**content_migration_url** | Option<**String**> | API url to the content migration | [optional]
**description** | Option<**String**> | Description of the issue for the end-user | [optional]
**workflow_state** | Option<**WorkflowState**> | Current state of the issue: active, resolved (enum: active, resolved) | [optional]
**fix_issue_html_url** | Option<**String**> | HTML Url to the Canvas page to investigate the issue | [optional]
**issue_type** | Option<**IssueType**> | Severity of the issue: todo, warning, error (enum: todo, warning, error) | [optional]
**error_report_html_url** | Option<**String**> | Link to a Canvas error report if present (If the requesting user has permissions) | [optional]
**error_message** | Option<**String**> | Site administrator error message (If the requesting user has permissions) | [optional]
**created_at** | Option<**String**> | timestamp | [optional]
**updated_at** | Option<**String**> | timestamp | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


