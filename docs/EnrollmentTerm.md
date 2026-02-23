# EnrollmentTerm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the enrollment term. | [optional]
**sis_term_id** | Option<**String**> | The SIS id of the term. Only included if the user has permission to view SIS information. | [optional]
**sis_import_id** | Option<**i32**> | the unique identifier for the SIS import. This field is only included if the user has permission to manage SIS information. | [optional]
**name** | Option<**String**> | The name of the term. | [optional]
**start_at** | Option<**String**> | The datetime of the start of the term. | [optional]
**end_at** | Option<**String**> | The datetime of the end of the term. | [optional]
**workflow_state** | Option<**String**> | The state of the term. Can be 'active' or 'deleted'. | [optional]
**overrides** | Option<**serde_json::Value**> | Term date overrides for specific enrollment types | [optional]
**course_count** | Option<**i32**> | The number of courses in the term (available via include) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


