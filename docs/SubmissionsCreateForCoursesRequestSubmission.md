# SubmissionsCreateForCoursesRequestSubmission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**group_comment** | Option<**bool**> | [Boolean] Whether or not this comment should be sent to the entire group (defaults to false). Ignored if this is not a group assignment or if no text_comment is provided. | [optional]
**submission_type** | **SubmissionType** | [Required, String, \"online_text_entry\"|\"online_url\"|\"online_upload\"|\"media_recording\"|\"basic_lti_launch\"|\"student_annotation\"] The type of submission being made. The assignment submission_types must include this submission type as an allowed option, or the submission will be rejected with a 400 error.  The submission_type given determines which of the following parameters is used. For instance, to submit a URL, +submission[submission_type]+ must be set to \"online_url\", otherwise the +submission[url]+ parameter will be ignored.  \"basic_lti_launch\" requires the assignment submission_type \"online\" or \"external_tool\" (enum: online_text_entry, online_url, online_upload, media_recording, basic_lti_launch, student_annotation) | 
**body** | Option<**String**> | [String] Submit the assignment as an HTML document snippet. Note this HTML snippet will be sanitized using the same ruleset as a submission made from the Canvas web UI. The sanitized HTML will be returned in the response as the submission body. Requires a submission_type of \"online_text_entry\". | [optional]
**url** | Option<**String**> | [String] Submit the assignment as a URL. The URL scheme must be \"http\" or \"https\", no \"ftp\" or other URL schemes are allowed. If no scheme is given (e.g. \"www.example.com\") then \"http\" will be assumed. Requires a submission_type of \"online_url\" or \"basic_lti_launch\". | [optional]
**file_ids** | Option<**String**> | [] [Integer] Submit the assignment as a set of one or more previously uploaded files residing in the submitting user's files section (or the group's files section, for group assignments).  To upload a new file to submit, see the submissions {api:SubmissionsApiController#create_file Upload a file API}.  Requires a submission_type of \"online_upload\". | [optional]
**media_comment_id** | Option<**String**> | [String] The media comment id to submit. Media comment ids can be submitted via this API, however, note that there is not yet an API to generate or list existing media comments, so this functionality is currently of limited use.  Requires a submission_type of \"media_recording\". | [optional]
**media_comment_type** | Option<**MediaCommentType**> | [String, \"audio\"|\"video\"] The type of media comment being submitted. (enum: audio, video) | [optional]
**user_id** | Option<**i32**> | [Integer] Submit on behalf of the given user. Requires grading permission. | [optional]
**annotatable_attachment_id** | Option<**i32**> | [Integer] The Attachment ID of the document being annotated. This should match the annotatable_attachment_id on the assignment.  Requires a submission_type of \"student_annotation\". | [optional]
**submitted_at** | Option<**String**> | [DateTime] Choose the time the submission is listed as submitted at.  Requires grading permission. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


