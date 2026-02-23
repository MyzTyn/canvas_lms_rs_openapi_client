# CoursesCreateRequestCourse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] The name of the course. If omitted, the course will be named \"Unnamed Course.\" | [optional]
**course_code** | Option<**String**> | [String] The course code for the course. | [optional]
**start_at** | Option<**String**> | [DateTime] Course start date in ISO8601 format, e.g. 2011-01-01T01:00Z This value is ignored unless 'restrict_enrollments_to_course_dates' is set to true. | [optional]
**end_at** | Option<**String**> | [DateTime] Course end date in ISO8601 format. e.g. 2011-01-01T01:00Z This value is ignored unless 'restrict_enrollments_to_course_dates' is set to true. | [optional]
**license** | Option<**String**> | [String] The name of the licensing. Should be one of the following abbreviations (a descriptive name is included in parenthesis for reference): - 'private' (Private Copyrighted) - 'cc_by_nc_nd' (CC Attribution Non-Commercial No Derivatives) - 'cc_by_nc_sa' (CC Attribution Non-Commercial Share Alike) - 'cc_by_nc' (CC Attribution Non-Commercial) - 'cc_by_nd' (CC Attribution No Derivatives) - 'cc_by_sa' (CC Attribution Share Alike) - 'cc_by' (CC Attribution) - 'public_domain' (Public Domain). | [optional]
**is_public** | Option<**bool**> | [Boolean] Set to true if course is public to both authenticated and unauthenticated users. | [optional]
**is_public_to_auth_users** | Option<**bool**> | [Boolean] Set to true if course is public only to authenticated users. | [optional]
**public_syllabus** | Option<**bool**> | [Boolean] Set to true to make the course syllabus public. | [optional]
**public_syllabus_to_auth** | Option<**bool**> | [Boolean] Set to true to make the course syllabus public for authenticated users. | [optional]
**public_description** | Option<**String**> | [String] A publicly visible description of the course. | [optional]
**allow_student_wiki_edits** | Option<**bool**> | [Boolean] If true, students will be able to modify the course wiki. | [optional]
**allow_wiki_comments** | Option<**bool**> | [Boolean] If true, course members will be able to comment on wiki pages. | [optional]
**allow_student_forum_attachments** | Option<**bool**> | [Boolean] If true, students can attach files to forum posts. | [optional]
**open_enrollment** | Option<**bool**> | [Boolean] Set to true if the course is open enrollment. | [optional]
**self_enrollment** | Option<**bool**> | [Boolean] Set to true if the course is self enrollment. | [optional]
**restrict_enrollments_to_course_dates** | Option<**bool**> | [Boolean] Set to true to restrict user enrollments to the start and end dates of the course. This value must be set to true in order to specify a course start date and/or end date. | [optional]
**term_id** | Option<**String**> | [String] The unique ID of the term to create to course in. | [optional]
**sis_course_id** | Option<**String**> | [String] The unique SIS identifier. | [optional]
**integration_id** | Option<**String**> | [String] The unique Integration identifier. | [optional]
**hide_final_grades** | Option<**bool**> | [Boolean] If this option is set to true, the totals in student grades summary will be hidden. | [optional]
**apply_assignment_group_weights** | Option<**bool**> | [Boolean] Set to true to weight final grade based on assignment groups percentages. | [optional]
**time_zone** | Option<**String**> | [String] The time zone for the course. Allowed time zones are {http://www.iana.org/time-zones IANA time zones} or friendlier {http://api.rubyonrails.org/classes/ActiveSupport/TimeZone.html Ruby on Rails time zones}. | [optional]
**default_view** | Option<**DefaultView**> | [String, \"feed\"|\"wiki\"|\"modules\"|\"syllabus\"|\"assignments\"] The type of page that users will see when they first visit the course * 'feed' Recent Activity Dashboard * 'modules' Course Modules/Sections Page * 'assignments' Course Assignments List * 'syllabus' Course Syllabus Page other types may be added in the future (enum: feed, wiki, modules, syllabus, assignments) | [optional]
**syllabus_body** | Option<**String**> | [String] The syllabus body for the course | [optional]
**grading_standard_id** | Option<**i32**> | [Integer] The grading standard id to set for the course.  If no value is provided for this argument the current grading_standard will be un-set from this course. | [optional]
**grade_passback_setting** | Option<**String**> | [String] Optional. The grade_passback_setting for the course. Only 'nightly_sync', 'disabled', and '' are allowed | [optional]
**course_format** | Option<**String**> | [String] Optional. Specifies the format of the course. (Should be 'on_campus', 'online', or 'blended') | [optional]
**post_manually** | Option<**bool**> | [Boolean] Default is false. When true, all grades in the course must be posted manually, and will not be automatically posted. When false, all grades in the course will be automatically posted. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


