# SisAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The unique identifier for the assignment. | [optional]
**course_id** | Option<**i32**> | The unique identifier for the course. | [optional]
**name** | Option<**String**> | the name of the assignment | [optional]
**created_at** | Option<**String**> | The time at which this assignment was originally created | [optional]
**due_at** | Option<**String**> | the due date for the assignment. returns null if not present. NOTE: If this assignment has assignment overrides, this field will be the due date as it applies to the user requesting information from the API. | [optional]
**unlock_at** | Option<**String**> | (Optional) Time at which this was/will be unlocked. | [optional]
**lock_at** | Option<**String**> | (Optional) Time at which this was/will be locked. | [optional]
**points_possible** | Option<**i32**> | The maximum points possible for the assignment | [optional]
**submission_types** | Option<**Vec<SubmissionTypes>**> | the types of submissions allowed for this assignment list containing one or more of the following: 'discussion_topic', 'online_quiz', 'on_paper', 'none', 'external_tool', 'online_text_entry', 'online_url', 'online_upload', 'media_recording', 'student_annotation' (enum: discussion_topic, online_quiz, on_paper, not_graded, none, external_tool, online_text_entry, online_url, online_upload, media_recording, student_annotation) | [optional]
**integration_id** | Option<**String**> | Third Party integration id for assignment | [optional]
**integration_data** | Option<**String**> | (optional, Third Party integration data for assignment) | [optional]
**include_in_final_grade** | Option<**bool**> | If false, the assignment will be omitted from the student's final grade | [optional]
**assignment_group** | Option<[**Vec<models::AssignmentGroupAttributes>**](AssignmentGroupAttributes.md)> | Includes attributes of a assignment_group for convenience. For more details see Assignments API. | [optional]
**sections** | Option<[**Vec<models::SectionAttributes>**](SectionAttributes.md)> | Includes attributes of a section for convenience. For more details see Sections API. | [optional]
**user_overrides** | Option<[**Vec<models::UserAssignmentOverrideAttributes>**](UserAssignmentOverrideAttributes.md)> | Includes attributes of a user assignment overrides. For more details see Assignments API. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


