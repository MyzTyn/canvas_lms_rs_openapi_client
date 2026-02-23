# CoursesUpdateSettingsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_final_grade_override** | Option<**bool**> | Let student final grades for a grading period or the total grades for the course be overridden | [optional]
**allow_student_discussion_topics** | Option<**bool**> | Let students create discussion topics | [optional]
**allow_student_forum_attachments** | Option<**bool**> | Let students attach files to discussions | [optional]
**allow_student_discussion_editing** | Option<**bool**> | Let students edit or delete their own discussion replies | [optional]
**allow_student_organized_groups** | Option<**bool**> | Let students organize their own groups | [optional]
**allow_student_discussion_reporting** | Option<**bool**> | Let students report offensive discussion content | [optional]
**allow_student_anonymous_discussion_topics** | Option<**bool**> | Let students create anonymous discussion topics | [optional]
**filter_speed_grader_by_student_group** | Option<**bool**> | Filter SpeedGrader to only the selected student group | [optional]
**hide_final_grades** | Option<**bool**> | Hide totals in student grades summary | [optional]
**hide_distribution_graphs** | Option<**bool**> | Hide grade distribution graphs from students | [optional]
**hide_sections_on_course_users_page** | Option<**bool**> | Disallow students from viewing students in sections they do not belong to | [optional]
**lock_all_announcements** | Option<**bool**> | Disable comments on announcements | [optional]
**usage_rights_required** | Option<**bool**> | Copyright and license information must be provided for files before they are published. | [optional]
**restrict_student_past_view** | Option<**bool**> | Restrict students from viewing courses after end date | [optional]
**restrict_student_future_view** | Option<**bool**> | Restrict students from viewing courses before start date | [optional]
**show_announcements_on_home_page** | Option<**bool**> | Show the most recent announcements on the Course home page (if a Wiki, defaults to five announcements, configurable via home_page_announcement_limit). Canvas for Elementary subjects ignore this setting. | [optional]
**home_page_announcement_limit** | Option<**i32**> | Limit the number of announcements on the home page if enabled via show_announcements_on_home_page | [optional]
**syllabus_course_summary** | Option<**bool**> | Show the course summary (list of assignments and calendar events) on the syllabus page. Default is true. | [optional]
**default_due_time** | Option<**String**> | Set the default due time for assignments. This is the time that will be pre-selected in the Canvas user interface when setting a due date for an assignment. It does not change when any existing assignment is due. It should be given in 24-hour HH:MM:SS format. The default is \"23:59:59\". Use \"inherit\" to inherit the account setting. | [optional]
**conditional_release** | Option<**bool**> | Enable or disable individual learning paths for students based on assessment | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


