# Enrollment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of the enrollment. | [optional]
**course_id** | Option<**i32**> | The unique id of the course. | [optional]
**sis_course_id** | Option<**String**> | The SIS Course ID in which the enrollment is associated. Only displayed if present. This field is only included if the user has permission to view SIS information. | [optional]
**course_integration_id** | Option<**String**> | The Course Integration ID in which the enrollment is associated. This field is only included if the user has permission to view SIS information. | [optional]
**course_section_id** | Option<**i32**> | The unique id of the user's section. | [optional]
**section_integration_id** | Option<**String**> | The Section Integration ID in which the enrollment is associated. This field is only included if the user has permission to view SIS information. | [optional]
**sis_account_id** | Option<**String**> | The SIS Account ID in which the enrollment is associated. Only displayed if present. This field is only included if the user has permission to view SIS information. | [optional]
**sis_section_id** | Option<**String**> | The SIS Section ID in which the enrollment is associated. Only displayed if present. This field is only included if the user has permission to view SIS information. | [optional]
**sis_user_id** | Option<**String**> | The SIS User ID in which the enrollment is associated. Only displayed if present. This field is only included if the user has permission to view SIS information. | [optional]
**enrollment_state** | Option<**String**> | The state of the user's enrollment in the course. | [optional]
**limit_privileges_to_course_section** | Option<**bool**> | User can only access his or her own course section. | [optional]
**sis_import_id** | Option<**i32**> | The unique identifier for the SIS import. This field is only included if the user has permission to manage SIS information. | [optional]
**root_account_id** | Option<**i32**> | The unique id of the user's account. | [optional]
**r#type** | Option<**String**> | The enrollment type. One of 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment', 'ObserverEnrollment'. | [optional]
**user_id** | Option<**i32**> | The unique id of the user. | [optional]
**associated_user_id** | Option<**i32**> | The unique id of the associated user. Will be null unless type is ObserverEnrollment. | [optional]
**role** | Option<**String**> | The enrollment role, for course-level permissions. This field will match `type` if the enrollment role has not been customized. | [optional]
**role_id** | Option<**i32**> | The id of the enrollment role. | [optional]
**created_at** | Option<**String**> | The created time of the enrollment, in ISO8601 format. | [optional]
**updated_at** | Option<**String**> | The updated time of the enrollment, in ISO8601 format. | [optional]
**start_at** | Option<**String**> | The start time of the enrollment, in ISO8601 format. | [optional]
**end_at** | Option<**String**> | The end time of the enrollment, in ISO8601 format. | [optional]
**last_activity_at** | Option<**String**> | The last activity time of the user for the enrollment, in ISO8601 format. | [optional]
**last_attended_at** | Option<**String**> | The last attended date of the user for the enrollment in a course, in ISO8601 format. | [optional]
**total_activity_time** | Option<**i32**> | The total activity time of the user for the enrollment, in seconds. | [optional]
**html_url** | Option<**String**> | The URL to the Canvas web UI page for this course enrollment. | [optional]
**grades** | Option<[**models::Grade**](Grade.md)> |  | [optional]
**user** | Option<[**models::User**](User.md)> |  | [optional]
**override_grade** | Option<**String**> | The user's override grade for the course. | [optional]
**override_score** | Option<**f64**> | The user's override score for the course. | [optional]
**unposted_current_grade** | Option<**String**> | The user's current grade in the class including muted/unposted assignments. Only included if user has permissions to view this grade, typically teachers, TAs, and admins. | [optional]
**unposted_final_grade** | Option<**String**> | The user's final grade for the class including muted/unposted assignments. Only included if user has permissions to view this grade, typically teachers, TAs, and admins.. | [optional]
**unposted_current_score** | Option<**String**> | The user's current score in the class including muted/unposted assignments. Only included if user has permissions to view this score, typically teachers, TAs, and admins.. | [optional]
**unposted_final_score** | Option<**String**> | The user's final score for the class including muted/unposted assignments. Only included if user has permissions to view this score, typically teachers, TAs, and admins.. | [optional]
**has_grading_periods** | Option<**bool**> | optional: Indicates whether the course the enrollment belongs to has grading periods set up. (applies only to student enrollments, and only available in course endpoints) | [optional]
**totals_for_all_grading_periods_option** | Option<**bool**> | optional: Indicates whether the course the enrollment belongs to has the Display Totals for 'All Grading Periods' feature enabled. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_grading_period_title** | Option<**String**> | optional: The name of the currently active grading period, if one exists. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_grading_period_id** | Option<**i32**> | optional: The id of the currently active grading period, if one exists. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_period_override_grade** | Option<**String**> | The user's override grade for the current grading period. | [optional]
**current_period_override_score** | Option<**f64**> | The user's override score for the current grading period. | [optional]
**current_period_unposted_current_score** | Option<**f64**> | optional: The student's score in the course for the current grading period, including muted/unposted assignments. Only included if user has permission to view this score, typically teachers, TAs, and admins. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_period_unposted_final_score** | Option<**f64**> | optional: The student's score in the course for the current grading period, including muted/unposted assignments and including ungraded assignments with a score of 0. Only included if user has permission to view this score, typically teachers, TAs, and admins. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_period_unposted_current_grade** | Option<**String**> | optional: The letter grade equivalent of current_period_unposted_current_score, if available. Only included if user has permission to view this grade, typically teachers, TAs, and admins. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]
**current_period_unposted_final_grade** | Option<**String**> | optional: The letter grade equivalent of current_period_unposted_final_score, if available. Only included if user has permission to view this grade, typically teachers, TAs, and admins. If the course the enrollment belongs to does not have grading periods, or if no currently active grading period exists, the value will be null. (applies only to student enrollments, and only available in course endpoints) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


