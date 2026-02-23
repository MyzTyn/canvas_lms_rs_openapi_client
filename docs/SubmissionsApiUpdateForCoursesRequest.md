# SubmissionsApiUpdateForCoursesRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**include** | Option<**Vec<String>**> | [String, \"submission_comments\"|\"visibility\"|\"sub_assignment_submissions\"|\"peer_review_submissions\"|\"provisional_grades\"|\"group\"] Associations to include with the submission. \"submission_comments\" is always included by default. - \"submission_comments\": Comments on the submission (always included) - \"visibility\": Whether the assignment is visible to the owner of the submission - \"sub_assignment_submissions\": Sub-assignment submissions for discussion checkpoints - \"peer_review_submissions\": Peer review submission data when peer review allocation and grading is enabled - \"provisional_grades\": Provisional grades (only available for moderated assignments) - \"group\": Group information (id and name) for group assignments | [optional]
**prefer_points_over_scheme** | Option<**bool**> | Treat posted_grade as points if the value matches a grading scheme value | [optional]
**comment** | Option<[**models::SubmissionsApiUpdateForCoursesRequestComment**](SubmissionsApiUpdateForCoursesRequestComment.md)> |  | [optional]
**submission** | Option<[**models::SubmissionsApiUpdateForCoursesRequestSubmission**](SubmissionsApiUpdateForCoursesRequestSubmission.md)> |  | [optional]
**rubric_assessment** | Option<[**models::SubmissionsApiUpdateForCoursesRequestRubricAssessment**](SubmissionsApiUpdateForCoursesRequestRubricAssessment.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


