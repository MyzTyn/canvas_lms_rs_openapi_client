# EnrollmentsApiBulkEnrollmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**user_ids** | **Vec<String>** | [Required, Integer] The user IDs to enroll in the courses. | 
**course_ids** | **Vec<String>** | [Required, Integer] The course IDs to enroll each user in. | 
**enrollment_type** | Option<**EnrollmentType**> | Enroll each user as a student, teacher, TA, observer, or designer. If no value is given, the type will be 'StudentEnrollment'. (enum: StudentEnrollment, TeacherEnrollment, TaEnrollment, ObserverEnrollment, DesignerEnrollment) | [optional]
**enrollment_role_id** | Option<**i32**> | Optional custom course-level role id to apply to created enrollments. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


