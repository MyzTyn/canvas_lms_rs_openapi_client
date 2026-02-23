# AssignmentsApiUpdateRequestAssignment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | Option<**String**> | [String] The assignment name. | [optional]
**position** | Option<**i32**> | [Integer] The position of this assignment in the group when displaying assignment lists. | [optional]
**submission_types** | Option<**String**> | [] [String, \"online_quiz\"|\"none\"|\"on_paper\"|\"discussion_topic\"|\"external_tool\"|\"online_upload\"|\"online_text_entry\"|\"online_url\"|\"media_recording\"|\"student_annotation\"] Only applies if the assignment doesn't have student submissions.  List of supported submission types for the assignment. Unless the assignment is allowing online submissions, the array should only have one element.  If not allowing online submissions, your options are:   \"online_quiz\"   \"none\"   \"on_paper\"   \"discussion_topic\"   \"external_tool\"  If you are allowing online submissions, you can have one or many allowed submission types:    \"online_upload\"   \"online_text_entry\"   \"online_url\"   \"media_recording\" (Only valid when the Kaltura plugin is enabled)   \"student_annotation\" | [optional]
**allowed_extensions** | Option<**String**> | [] [String] Allowed extensions if submission_types includes \"online_upload\"  Example:   allowed_extensions: [\"docx\",\"ppt\"] | [optional]
**turnitin_enabled** | Option<**bool**> | [Boolean] Only applies when the Turnitin plugin is enabled for a course and the submission_types array includes \"online_upload\". Toggles Turnitin submissions for the assignment. Will be ignored if Turnitin is not available for the course. | [optional]
**vericite_enabled** | Option<**bool**> | [Boolean] Only applies when the VeriCite plugin is enabled for a course and the submission_types array includes \"online_upload\". Toggles VeriCite submissions for the assignment. Will be ignored if VeriCite is not available for the course. | [optional]
**turnitin_settings** | Option<**String**> | Settings to send along to turnitin. See Assignment object definition for format. | [optional]
**sis_assignment_id** | Option<**String**> | The sis id of the Assignment | [optional]
**integration_data** | **String** | Data used for SIS integrations. Requires admin-level token with the \"Manage SIS\" permission. JSON string required. | 
**integration_id** | Option<**String**> | Unique ID from third party integrations | [optional]
**peer_reviews** | Option<**bool**> | [Boolean] If submission_types does not include external_tool,discussion_topic, online_quiz, or on_paper, determines whether or not peer reviews will be turned on for the assignment. | [optional]
**automatic_peer_reviews** | Option<**bool**> | [Boolean] Whether peer reviews will be assigned automatically by Canvas or if teachers must manually assign peer reviews. Does not apply if peer reviews are not enabled. | [optional]
**notify_of_update** | Option<**bool**> | [Boolean] If true, Canvas will send a notification to students in the class notifying them that the content has changed. | [optional]
**group_category_id** | Option<**i32**> | [Integer] If present, the assignment will become a group assignment assigned to the group. | [optional]
**grade_group_students_individually** | Option<**i32**> | [Integer] If this is a group assignment, teachers have the options to grade students individually. If false, Canvas will apply the assignment's score to each member of the group. If true, the teacher can manually assign scores to each member of the group. | [optional]
**external_tool_tag_attributes** | Option<**String**> | Hash of external tool parameters if submission_types is [\"external_tool\"]. See Assignment object definition for format. | [optional]
**points_possible** | Option<**f64**> | [Float] The maximum points possible on the assignment. | [optional]
**grading_type** | Option<**String**> | [\"pass_fail\"|\"percent\"|\"letter_grade\"|\"gpa_scale\"|\"points\"|\"not_graded\"] The strategy used for grading the assignment. The assignment defaults to \"points\" if this field is omitted. | [optional]
**due_at** | Option<**String**> | [DateTime] The day/time the assignment is due. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. | [optional]
**lock_at** | Option<**String**> | [DateTime] The day/time the assignment is locked after. Must be after the due date if there is a due date. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. | [optional]
**unlock_at** | Option<**String**> | [DateTime] The day/time the assignment is unlocked. Must be before the due date if there is a due date. Accepts times in ISO 8601 format, e.g. 2014-10-21T18:48:00Z. | [optional]
**description** | Option<**String**> | [String] The assignment's description, supports HTML. | [optional]
**assignment_group_id** | Option<**i32**> | [Integer] The assignment group id to put the assignment in. Defaults to the top assignment group in the course. | [optional]
**assignment_overrides** | Option<**String**> | [] [AssignmentOverride] List of overrides for the assignment. If the +assignment[assignment_overrides]+ key is absent, any existing overrides are kept as is. If the +assignment[assignment_overrides]+ key is present, existing overrides are updated or deleted (and new ones created, as necessary) to match the provided list. | [optional]
**only_visible_to_overrides** | Option<**bool**> | [Boolean] Whether this assignment is only visible to overrides (Only useful if 'differentiated assignments' account setting is on) | [optional]
**published** | Option<**bool**> | [Boolean] Whether this assignment is published. (Only useful if 'draft state' account setting is on) Unpublished assignments are not visible to students. | [optional]
**grading_standard_id** | Option<**i32**> | [Integer] The grading standard id to set for the course.  If no value is provided for this argument the current grading_standard will be un-set from this course. This will update the grading_type for the course to 'letter_grade' unless it is already 'gpa_scale'. | [optional]
**omit_from_final_grade** | Option<**bool**> | [Boolean] Whether this assignment is counted towards a student's final grade. | [optional]
**hide_in_gradebook** | Option<**bool**> | [Boolean] Whether this assignment is shown in the gradebook. | [optional]
**moderated_grading** | Option<**bool**> | [Boolean] Whether this assignment is moderated. | [optional]
**grader_count** | Option<**i32**> | [Integer] The maximum number of provisional graders who may issue grades for this assignment. Only relevant for moderated assignments. Must be a positive value, and must be set to 1 if the course has fewer than two active instructors. Otherwise, the maximum value is the number of active instructors in the course minus one, or 10 if the course has more than 11 active instructors. | [optional]
**final_grader_id** | Option<**i32**> | [Integer] The user ID of the grader responsible for choosing final grades for this assignment. Only relevant for moderated assignments. | [optional]
**grader_comments_visible_to_graders** | Option<**bool**> | [Boolean] Boolean indicating if provisional graders' comments are visible to other provisional graders. Only relevant for moderated assignments. | [optional]
**graders_anonymous_to_graders** | Option<**bool**> | [Boolean] Boolean indicating if provisional graders' identities are hidden from other provisional graders. Only relevant for moderated assignments. | [optional]
**graders_names_visible_to_final_grader** | Option<**bool**> | [Boolean] Boolean indicating if provisional grader identities are visible to the the final grader. Only relevant for moderated assignments. | [optional]
**anonymous_grading** | Option<**bool**> | [Boolean] Boolean indicating if the assignment is graded anonymously. If true, graders cannot see student identities. | [optional]
**allowed_attempts** | Option<**i32**> | [Integer] The number of submission attempts allowed for this assignment. Set to -1 or null for unlimited attempts. | [optional]
**annotatable_attachment_id** | Option<**i32**> | [Integer] The Attachment ID of the document being annotated.  Only applies when submission_types includes \"student_annotation\". | [optional]
**asset_processors** | Option<**String**> | [] [Array] Document processors for this assignment. New document processors can only be added via the interactive LTI Deep Linking flow (in a browser), not via API token or JWT authentication. Deletion of document processors (passing an empty array) is allowed via API. | [optional]
**force_updated_at** | Option<**bool**> | [Boolean] If true, updated_at will be set even if no changes were made. | [optional]
**peer_review** | Option<**String**> | [peer_review_overrides][] [AssignmentOverride] List of overrides for the peer reviews. When updating overrides: - Include \"id\" to update an existing override - Omit \"id\" to create a new override - Omit an override from the list to delete it | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


