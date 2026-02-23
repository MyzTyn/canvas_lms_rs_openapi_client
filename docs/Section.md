# Section

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the section. | [optional]
**name** | Option<**String**> | The name of the section. | [optional]
**sis_section_id** | Option<**String**> | The sis id of the section. This field is only included if the user has permission to view SIS information. | [optional]
**integration_id** | Option<**String**> | Optional: The integration ID of the section. This field is only included if the user has permission to view SIS information. | [optional]
**sis_import_id** | Option<**i32**> | The unique identifier for the SIS import if created through SIS. This field is only included if the user has permission to manage SIS information. | [optional]
**course_id** | Option<**i32**> | The unique Canvas identifier for the course in which the section belongs | [optional]
**sis_course_id** | Option<**String**> | The unique SIS identifier for the course in which the section belongs. This field is only included if the user has permission to view SIS information. | [optional]
**start_at** | Option<**String**> | the start date for the section, if applicable | [optional]
**end_at** | Option<**String**> | the end date for the section, if applicable | [optional]
**restrict_enrollments_to_section_dates** | Option<**bool**> | Restrict user enrollments to the start and end dates of the section | [optional]
**nonxlist_course_id** | Option<**i32**> | The unique identifier of the original course of a cross-listed section | [optional]
**total_students** | Option<**i32**> | optional: the total number of active and invited students in the section | [optional]
**students** | Option<[**Vec<models::User>**](User.md)> | optional: A list of students that are included in the section. Returned only if include[]=students. WARNING: this collection's size is capped (if there are an extremely large number of users in the section (thousands) not all of them will be returned). If you need to capture all the users in a section with certainty or experiencing slow response consider using the paginated /api/v1/sections/<section_id>/users endpoint. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


