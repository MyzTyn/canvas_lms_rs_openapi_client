# EnrollmentsApiCreateForCoursesRequestEnrollment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**start_at** | Option<**String**> | [DateTime] The start time of the enrollment, in ISO8601 format. e.g. 2012-04-18T23:08:51Z | [optional]
**end_at** | Option<**String**> | [DateTime] The end time of the enrollment, in ISO8601 format. e.g. 2012-04-18T23:08:51Z | [optional]
**user_id** | **String** | [Required, String] The ID of the user to be enrolled in the course. | 
**r#type** | **Type** | [Required, String, \"StudentEnrollment\"|\"TeacherEnrollment\"|\"TaEnrollment\"|\"ObserverEnrollment\"|\"DesignerEnrollment\"] Enroll the user as a student, teacher, TA, observer, or designer. If no value is given, the type will be inferred by +enrollment[role]+ if supplied, otherwise 'StudentEnrollment' will be used. (enum: StudentEnrollment, TeacherEnrollment, TaEnrollment, ObserverEnrollment, DesignerEnrollment) | 
**role** | Option<**String**> | [Deprecated, String] Assigns a custom course-level role to the user. | [optional]
**role_id** | Option<**i32**> | [Integer] Assigns a custom course-level role to the user. | [optional]
**enrollment_state** | **EnrollmentState** | [String, \"active\"|\"invited\"|\"inactive\"] If set to 'active,' student will be immediately enrolled in the course. Otherwise they will be required to accept a course invitation. Default is 'invited.'.  If set to 'inactive', student will be listed in the course roster for teachers, but will not be able to participate in the course until their enrollment is activated. (enum: active, invited, inactive) | 
**course_section_id** | Option<**i32**> | [Integer] The ID of the course section to enroll the student in. If the section-specific URL is used, this argument is redundant and will be ignored. | [optional]
**limit_privileges_to_course_section** | Option<**bool**> | [Boolean] If set, the enrollment will only allow the user to see and interact with users enrolled in the section given by course_section_id. * For teachers and TAs, this includes grading privileges. * Section-limited students will not see any users (including teachers   and TAs) not enrolled in their sections. * Users may have other enrollments that grant privileges to   multiple sections in the same course. | [optional]
**notify** | Option<**bool**> | [Boolean] If true, a notification will be sent to the enrolled user. Notifications are not sent by default. | [optional]
**self_enrollment_code** | Option<**String**> | [String] If the current user is not allowed to manage enrollments in this course, but the course allows self-enrollment, the user can self- enroll as a student in the default section by passing in a valid code. When self-enrolling, the user_id must be 'self'. The enrollment_state will be set to 'active' and all other arguments will be ignored. | [optional]
**self_enrolled** | Option<**bool**> | [Boolean] If true, marks the enrollment as a self-enrollment, which gives students the ability to drop the course if desired. Defaults to false. | [optional]
**associated_user_id** | Option<**i32**> | [Integer] For an observer enrollment, the ID of a student to observe. This is a one-off operation; to automatically observe all a student's enrollments (for example, as a parent), please use the {api:UserObserveesController#create User Observees API}. | [optional]
**sis_user_id** | **String** | [String] Required if the user is being enrolled from another trusted account. The unique identifier for the user (sis_user_id) must also be accompanied by the root_account parameter. The user_id will be ignored. | 
**integration_id** | **String** | [String] Required if the user is being enrolled from another trusted account. The unique identifier for the user (integration_id) must also be accompanied by the root_account parameter. The user_id will be ignored. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


