# AssignmentOverride

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the assignment override | [optional]
**assignment_id** | Option<**i32**> | the ID of the assignment the override applies to (present if the override applies to an assignment) | [optional]
**quiz_id** | Option<**i32**> | the ID of the quiz the override applies to (present if the override applies to a quiz) | [optional]
**context_module_id** | Option<**i32**> | the ID of the module the override applies to (present if the override applies to a module) | [optional]
**discussion_topic_id** | Option<**i32**> | the ID of the discussion the override applies to (present if the override applies to an ungraded discussion) | [optional]
**wiki_page_id** | Option<**i32**> | the ID of the page the override applies to (present if the override applies to a page) | [optional]
**attachment_id** | Option<**i32**> | the ID of the file the override applies to (present if the override applies to a file) | [optional]
**student_ids** | Option<**Vec<i32>**> | the IDs of the override's target students (present if the override targets an ad-hoc set of students) | [optional]
**group_id** | Option<**i32**> | the ID of the override's target group (present if the override targets a group and the assignment is a group assignment) | [optional]
**course_section_id** | Option<**i32**> | the ID of the overrides's target section (present if the override targets a section) | [optional]
**title** | Option<**String**> | the title of the override | [optional]
**due_at** | Option<**String**> | the overridden due at (present if due_at is overridden) | [optional]
**all_day** | Option<**bool**> | the overridden all day flag (present if due_at is overridden) | [optional]
**all_day_date** | Option<**String**> | the overridden all day date (present if due_at is overridden) | [optional]
**unlock_at** | Option<**String**> | the overridden unlock at (present if unlock_at is overridden) | [optional]
**lock_at** | Option<**String**> | the overridden lock at, if any (present if lock_at is overridden) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


