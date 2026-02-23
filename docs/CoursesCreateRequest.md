# CoursesCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**offer** | Option<**bool**> | If this option is set to true, the course will be available to students immediately. | [optional]
**enroll_me** | Option<**bool**> | Set to true to enroll the current user as the teacher. | [optional]
**skip_course_template** | Option<**bool**> | If this option is set to true, the template of the account will not be applied to this course It means copy_from_course_template will not be executed. This option is thought for a course copy. | [optional]
**enable_sis_reactivation** | Option<**bool**> | When true, will first try to re-activate a deleted course with matching sis_course_id if possible. | [optional]
**course** | Option<[**models::CoursesCreateRequestCourse**](CoursesCreateRequestCourse.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


