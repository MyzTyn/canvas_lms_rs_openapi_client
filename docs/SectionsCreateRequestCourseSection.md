# SectionsCreateRequestCourseSection

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] The name of the section | [optional]
**sis_section_id** | Option<**String**> | [String] The sis ID of the section. Must have manage_sis permission to set. This is ignored if caller does not have permission to set. | [optional]
**integration_id** | Option<**String**> | [String] The integration_id of the section. Must have manage_sis permission to set. This is ignored if caller does not have permission to set. | [optional]
**start_at** | Option<**String**> | [DateTime] Section start date in ISO8601 format, e.g. 2011-01-01T01:00Z | [optional]
**end_at** | Option<**String**> | [DateTime] Section end date in ISO8601 format. e.g. 2011-01-01T01:00Z | [optional]
**restrict_enrollments_to_section_dates** | Option<**bool**> | [Boolean] Set to true to restrict user enrollments to the start and end dates of the section. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


