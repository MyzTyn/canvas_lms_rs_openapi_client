# TermsCreateRequestEnrollmentTerm

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] The name of the term. | [optional]
**start_at** | Option<**String**> | [DateTime] The day/time the term starts. Accepts times in ISO 8601 format, e.g. 2015-01-10T18:48:00Z. | [optional]
**end_at** | Option<**String**> | [DateTime] The day/time the term ends. Accepts times in ISO 8601 format, e.g. 2015-01-10T18:48:00Z. | [optional]
**sis_term_id** | Option<**String**> | [String] The unique SIS identifier for the term. | [optional]
**overrides** | Option<**String**> | [enrollment_type][end_at] [DateTime] The day/time the term ends, overridden for the given enrollment type. *enrollment_type* can be one of StudentEnrollment, TeacherEnrollment, TaEnrollment, or DesignerEnrollment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


