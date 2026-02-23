# \SubmissionsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submissions_api_bulk_update_for_courses**](SubmissionsApiApi.md#submissions_api_bulk_update_for_courses) | **POST** /courses/{course_id}/submissions/update_grades | Grade or comment on multiple submissions
[**submissions_api_bulk_update_for_courses2**](SubmissionsApiApi.md#submissions_api_bulk_update_for_courses2) | **POST** /courses/{course_id}/assignments/{assignment_id}/submissions/update_grades | Grade or comment on multiple submissions
[**submissions_api_bulk_update_other**](SubmissionsApiApi.md#submissions_api_bulk_update_other) | **POST** /sections/{section_id}/submissions/update_grades | Grade or comment on multiple submissions
[**submissions_api_bulk_update_other2**](SubmissionsApiApi.md#submissions_api_bulk_update_other2) | **POST** /sections/{section_id}/assignments/{assignment_id}/submissions/update_grades | Grade or comment on multiple submissions
[**submissions_api_create_file_for_courses**](SubmissionsApiApi.md#submissions_api_create_file_for_courses) | **POST** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/files | Upload a file
[**submissions_api_create_file_other**](SubmissionsApiApi.md#submissions_api_create_file_other) | **POST** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/files | Upload a file
[**submissions_api_document_annotations_read_state_for_courses**](SubmissionsApiApi.md#submissions_api_document_annotations_read_state_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/document_annotations/read | Get document annotations read state
[**submissions_api_document_annotations_read_state_other**](SubmissionsApiApi.md#submissions_api_document_annotations_read_state_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/document_annotations/read | Get document annotations read state
[**submissions_api_for_students_for_courses**](SubmissionsApiApi.md#submissions_api_for_students_for_courses) | **GET** /courses/{course_id}/students/submissions | List submissions for multiple assignments
[**submissions_api_for_students_other**](SubmissionsApiApi.md#submissions_api_for_students_other) | **GET** /sections/{section_id}/students/submissions | List submissions for multiple assignments
[**submissions_api_gradeable_students**](SubmissionsApiApi.md#submissions_api_gradeable_students) | **GET** /courses/{course_id}/assignments/{assignment_id}/gradeable_students | List gradeable students
[**submissions_api_index_for_courses**](SubmissionsApiApi.md#submissions_api_index_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions | List assignment submissions
[**submissions_api_index_other**](SubmissionsApiApi.md#submissions_api_index_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions | List assignment submissions
[**submissions_api_mark_bulk_submissions_as_read_for_courses**](SubmissionsApiApi.md#submissions_api_mark_bulk_submissions_as_read_for_courses) | **PUT** /courses/{course_id}/submissions/bulk_mark_read | Mark bulk submissions as read
[**submissions_api_mark_bulk_submissions_as_read_other**](SubmissionsApiApi.md#submissions_api_mark_bulk_submissions_as_read_other) | **PUT** /sections/{section_id}/submissions/bulk_mark_read | Mark bulk submissions as read
[**submissions_api_mark_document_annotations_read_for_courses**](SubmissionsApiApi.md#submissions_api_mark_document_annotations_read_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/document_annotations/read | Mark document annotations as read
[**submissions_api_mark_document_annotations_read_other**](SubmissionsApiApi.md#submissions_api_mark_document_annotations_read_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/document_annotations/read | Mark document annotations as read
[**submissions_api_mark_rubric_assessments_read_for_courses**](SubmissionsApiApi.md#submissions_api_mark_rubric_assessments_read_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_comments/read | Mark rubric assessments as read
[**submissions_api_mark_rubric_assessments_read_for_courses2**](SubmissionsApiApi.md#submissions_api_mark_rubric_assessments_read_for_courses2) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_assessments/read | Mark rubric assessments as read
[**submissions_api_mark_rubric_assessments_read_other**](SubmissionsApiApi.md#submissions_api_mark_rubric_assessments_read_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_comments/read | Mark rubric assessments as read
[**submissions_api_mark_rubric_assessments_read_other2**](SubmissionsApiApi.md#submissions_api_mark_rubric_assessments_read_other2) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_assessments/read | Mark rubric assessments as read
[**submissions_api_mark_submission_item_read_for_courses**](SubmissionsApiApi.md#submissions_api_mark_submission_item_read_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/read/{item} | Mark submission item as read
[**submissions_api_mark_submission_item_read_other**](SubmissionsApiApi.md#submissions_api_mark_submission_item_read_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/read/{item} | Mark submission item as read
[**submissions_api_mark_submission_read_for_courses**](SubmissionsApiApi.md#submissions_api_mark_submission_read_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/read | Mark submission as read
[**submissions_api_mark_submission_read_other**](SubmissionsApiApi.md#submissions_api_mark_submission_read_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/read | Mark submission as read
[**submissions_api_mark_submission_unread_for_courses**](SubmissionsApiApi.md#submissions_api_mark_submission_unread_for_courses) | **DELETE** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/read | Mark submission as unread
[**submissions_api_mark_submission_unread_other**](SubmissionsApiApi.md#submissions_api_mark_submission_unread_other) | **DELETE** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/read | Mark submission as unread
[**submissions_api_multiple_gradeable_students**](SubmissionsApiApi.md#submissions_api_multiple_gradeable_students) | **GET** /courses/{course_id}/assignments/gradeable_students | List multiple assignments gradeable students
[**submissions_api_rubric_assessments_read_state_for_courses**](SubmissionsApiApi.md#submissions_api_rubric_assessments_read_state_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_comments/read | Get rubric assessments read state
[**submissions_api_rubric_assessments_read_state_for_courses2**](SubmissionsApiApi.md#submissions_api_rubric_assessments_read_state_for_courses2) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_assessments/read | Get rubric assessments read state
[**submissions_api_rubric_assessments_read_state_other**](SubmissionsApiApi.md#submissions_api_rubric_assessments_read_state_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_comments/read | Get rubric assessments read state
[**submissions_api_rubric_assessments_read_state_other2**](SubmissionsApiApi.md#submissions_api_rubric_assessments_read_state_other2) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id}/rubric_assessments/read | Get rubric assessments read state
[**submissions_api_show_anonymous_for_courses**](SubmissionsApiApi.md#submissions_api_show_anonymous_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/anonymous_submissions/{anonymous_id} | Get a single submission by anonymous id
[**submissions_api_show_anonymous_other**](SubmissionsApiApi.md#submissions_api_show_anonymous_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/anonymous_submissions/{anonymous_id} | Get a single submission by anonymous id
[**submissions_api_show_for_courses**](SubmissionsApiApi.md#submissions_api_show_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id} | Get a single submission
[**submissions_api_show_other**](SubmissionsApiApi.md#submissions_api_show_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id} | Get a single submission
[**submissions_api_submission_summary_for_courses**](SubmissionsApiApi.md#submissions_api_submission_summary_for_courses) | **GET** /courses/{course_id}/assignments/{assignment_id}/submission_summary | Submission Summary
[**submissions_api_submission_summary_other**](SubmissionsApiApi.md#submissions_api_submission_summary_other) | **GET** /sections/{section_id}/assignments/{assignment_id}/submission_summary | Submission Summary
[**submissions_api_submissions_clear_unread_for_courses**](SubmissionsApiApi.md#submissions_api_submissions_clear_unread_for_courses) | **PUT** /courses/{course_id}/submissions/{user_id}/clear_unread | Clear unread status for all submissions.
[**submissions_api_submissions_clear_unread_other**](SubmissionsApiApi.md#submissions_api_submissions_clear_unread_other) | **PUT** /sections/{section_id}/submissions/{user_id}/clear_unread | Clear unread status for all submissions.
[**submissions_api_update_anonymous_for_courses**](SubmissionsApiApi.md#submissions_api_update_anonymous_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/anonymous_submissions/{anonymous_id} | Grade or comment on a submission by anonymous id
[**submissions_api_update_anonymous_other**](SubmissionsApiApi.md#submissions_api_update_anonymous_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/anonymous_submissions/{anonymous_id} | Grade or comment on a submission by anonymous id
[**submissions_api_update_for_courses**](SubmissionsApiApi.md#submissions_api_update_for_courses) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id} | Grade or comment on a submission
[**submissions_api_update_other**](SubmissionsApiApi.md#submissions_api_update_other) | **PUT** /sections/{section_id}/assignments/{assignment_id}/submissions/{user_id} | Grade or comment on a submission



## submissions_api_bulk_update_for_courses

> String submissions_api_bulk_update_for_courses(course_id, submissions_api_bulk_update_for_courses_request)
Grade or comment on multiple submissions

Update the grading and comments on multiple student's assignment submissions in an asynchronous job.  The user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**submissions_api_bulk_update_for_courses_request** | Option<[**SubmissionsApiBulkUpdateForCoursesRequest**](SubmissionsApiBulkUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_bulk_update_for_courses2

> String submissions_api_bulk_update_for_courses2(assignment_id, course_id, submissions_api_bulk_update_for_courses_request)
Grade or comment on multiple submissions

Update the grading and comments on multiple student's assignment submissions in an asynchronous job.  The user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | ID of the assignment | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**submissions_api_bulk_update_for_courses_request** | Option<[**SubmissionsApiBulkUpdateForCoursesRequest**](SubmissionsApiBulkUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_bulk_update_other

> String submissions_api_bulk_update_other(section_id, submissions_api_bulk_update_for_courses_request)
Grade or comment on multiple submissions

Update the grading and comments on multiple student's assignment submissions in an asynchronous job.  The user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**submissions_api_bulk_update_for_courses_request** | Option<[**SubmissionsApiBulkUpdateForCoursesRequest**](SubmissionsApiBulkUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_bulk_update_other2

> String submissions_api_bulk_update_other2(assignment_id, section_id, submissions_api_bulk_update_for_courses_request)
Grade or comment on multiple submissions

Update the grading and comments on multiple student's assignment submissions in an asynchronous job.  The user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | ID of the assignment | [required] |
**section_id** | **String** | ID of the section | [required] |
**submissions_api_bulk_update_for_courses_request** | Option<[**SubmissionsApiBulkUpdateForCoursesRequest**](SubmissionsApiBulkUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_create_file_for_courses

> submissions_api_create_file_for_courses(assignment_id, course_id, user_id)
Upload a file

Upload a file to a submission.  This API endpoint is the first step in uploading a file to a submission as a student. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  The final step of the file upload workflow will return the attachment data, including the new file id. The caller can then POST to submit the +online_upload+ assignment with these file ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_create_file_other

> submissions_api_create_file_other(section_id, assignment_id, user_id)
Upload a file

Upload a file to a submission.  This API endpoint is the first step in uploading a file to a submission as a student. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  The final step of the file upload workflow will return the attachment data, including the new file id. The caller can then POST to submit the +online_upload+ assignment with these file ids.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_document_annotations_read_state_for_courses

> submissions_api_document_annotations_read_state_for_courses(assignment_id, course_id, user_id)
Get document annotations read state

Return whether annotations made on a submitted document have been read by the student

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_document_annotations_read_state_other

> submissions_api_document_annotations_read_state_other(section_id, assignment_id, user_id)
Get document annotations read state

Return whether annotations made on a submitted document have been read by the student

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_for_students_for_courses

> submissions_api_for_students_for_courses(course_id, student_ids, assignment_ids, grouped, post_to_sis, submitted_since, graded_since, grading_period_id, workflow_state, enrollment_state, state_based_on_date, order, order_direction, include, page, per_page)
List submissions for multiple assignments

A paginated list of all existing submissions for a given set of students and assignments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**student_ids** | Option<[**Vec<String>**](String.md)> | [String] List of student ids to return submissions for. If this argument is omitted, return submissions for the calling user. Students may only list their own submissions. Observers may only list those of associated students. The special id \"all\" will return submissions for all students in the course/section as appropriate. |  |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | [String] List of assignments to return submissions for. If none are given, submissions for all assignments are returned. |  |
**grouped** | Option<**bool**> | If this argument is present, the response will be grouped by student, rather than a flat array of submissions. |  |
**post_to_sis** | Option<**bool**> | If this argument is set to true, the response will only include submissions for assignments that have the post_to_sis flag set to true and user enrollments that were added through sis. |  |
**submitted_since** | Option<**String**> | If this argument is set, the response will only include submissions that were submitted after the specified date_time. This will exclude submissions that do not have a submitted_at which will exclude unsubmitted submissions. The value must be formatted as ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**graded_since** | Option<**String**> | If this argument is set, the response will only include submissions that were graded after the specified date_time. This will exclude submissions that have not been graded. The value must be formatted as ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**grading_period_id** | Option<**i32**> | The id of the grading period in which submissions are being requested (Requires grading periods to exist on the account) |  |
**workflow_state** | Option<**String**> | The current status of the submission |  |
**enrollment_state** | Option<**String**> | The current state of the enrollments. If omitted will include all enrollments that are not deleted. |  |
**state_based_on_date** | Option<**bool**> | If omitted it is set to true. When set to false it will ignore the effective state of the student enrollments and use the workflow_state for the enrollments. The argument is ignored unless enrollment_state argument is also passed. |  |
**order** | Option<**String**> | The order submissions will be returned in.  Defaults to \"id\".  Doesn't affect results for \"grouped\" mode. |  |
**order_direction** | Option<**String**> | Determines whether ordered results are returned in ascending or descending order.  Defaults to \"ascending\".  Doesn't affect results for \"grouped\" mode. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"assignment\"|\"total_scores\"|\"visibility\"|\"course\"|\"user\"|\"sub_assignment_submissions\"|\"peer_review_submissions\"|\"student_entered_score\"] Associations to include with the group. `total_scores` requires the `grouped` argument. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_for_students_other

> submissions_api_for_students_other(section_id, student_ids, assignment_ids, grouped, post_to_sis, submitted_since, graded_since, grading_period_id, workflow_state, enrollment_state, state_based_on_date, order, order_direction, include, page, per_page)
List submissions for multiple assignments

A paginated list of all existing submissions for a given set of students and assignments.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**student_ids** | Option<[**Vec<String>**](String.md)> | [String] List of student ids to return submissions for. If this argument is omitted, return submissions for the calling user. Students may only list their own submissions. Observers may only list those of associated students. The special id \"all\" will return submissions for all students in the course/section as appropriate. |  |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | [String] List of assignments to return submissions for. If none are given, submissions for all assignments are returned. |  |
**grouped** | Option<**bool**> | If this argument is present, the response will be grouped by student, rather than a flat array of submissions. |  |
**post_to_sis** | Option<**bool**> | If this argument is set to true, the response will only include submissions for assignments that have the post_to_sis flag set to true and user enrollments that were added through sis. |  |
**submitted_since** | Option<**String**> | If this argument is set, the response will only include submissions that were submitted after the specified date_time. This will exclude submissions that do not have a submitted_at which will exclude unsubmitted submissions. The value must be formatted as ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**graded_since** | Option<**String**> | If this argument is set, the response will only include submissions that were graded after the specified date_time. This will exclude submissions that have not been graded. The value must be formatted as ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**grading_period_id** | Option<**i32**> | The id of the grading period in which submissions are being requested (Requires grading periods to exist on the account) |  |
**workflow_state** | Option<**String**> | The current status of the submission |  |
**enrollment_state** | Option<**String**> | The current state of the enrollments. If omitted will include all enrollments that are not deleted. |  |
**state_based_on_date** | Option<**bool**> | If omitted it is set to true. When set to false it will ignore the effective state of the student enrollments and use the workflow_state for the enrollments. The argument is ignored unless enrollment_state argument is also passed. |  |
**order** | Option<**String**> | The order submissions will be returned in.  Defaults to \"id\".  Doesn't affect results for \"grouped\" mode. |  |
**order_direction** | Option<**String**> | Determines whether ordered results are returned in ascending or descending order.  Defaults to \"ascending\".  Doesn't affect results for \"grouped\" mode. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"assignment\"|\"total_scores\"|\"visibility\"|\"course\"|\"user\"|\"sub_assignment_submissions\"|\"peer_review_submissions\"|\"student_entered_score\"] Associations to include with the group. `total_scores` requires the `grouped` argument. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_gradeable_students

> models::AnonymousUserDisplay submissions_api_gradeable_students(assignment_id, course_id, sort, order, page, per_page)
List gradeable students

A paginated list of gradeable students for the assignment. The caller must have permission to view grades.  If anonymous grading is enabled for the current assignment and the allow_new_anonymous_id parameter is passed, the returned data will not include any values identifying the student, but will instead include an assignment-specific anonymous ID for each student.  Section-limited instructors will only see students in their own sections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**sort** | Option<**String**> | Sort results by this field. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::AnonymousUserDisplay**](AnonymousUserDisplay.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_index_for_courses

> serde_json::Value submissions_api_index_for_courses(assignment_id, course_id, include, grouped, page, per_page)
List assignment submissions

A paginated list of all existing submissions for an assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"assignment\"|\"visibility\"|\"course\"|\"user\"|\"group\"|\"read_status\"|\"student_entered_score\"] Associations to include with the group.  \"group\" will add group_id and group_name. |  |
**grouped** | Option<**bool**> | If this argument is true, the response will be grouped by student groups. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_index_other

> serde_json::Value submissions_api_index_other(section_id, assignment_id, include, grouped, page, per_page)
List assignment submissions

A paginated list of all existing submissions for an assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"assignment\"|\"visibility\"|\"course\"|\"user\"|\"group\"|\"read_status\"|\"student_entered_score\"] Associations to include with the group.  \"group\" will add group_id and group_name. |  |
**grouped** | Option<**bool**> | If this argument is true, the response will be grouped by student groups. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_bulk_submissions_as_read_for_courses

> submissions_api_mark_bulk_submissions_as_read_for_courses(course_id, submissions_api_mark_bulk_submissions_as_read_for_courses_request)
Mark bulk submissions as read

Accepts a string array of submission ids. Loops through and marks each submission as read  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**submissions_api_mark_bulk_submissions_as_read_for_courses_request** | Option<[**SubmissionsApiMarkBulkSubmissionsAsReadForCoursesRequest**](SubmissionsApiMarkBulkSubmissionsAsReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_bulk_submissions_as_read_other

> submissions_api_mark_bulk_submissions_as_read_other(section_id, submissions_api_mark_bulk_submissions_as_read_for_courses_request)
Mark bulk submissions as read

Accepts a string array of submission ids. Loops through and marks each submission as read  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**submissions_api_mark_bulk_submissions_as_read_for_courses_request** | Option<[**SubmissionsApiMarkBulkSubmissionsAsReadForCoursesRequest**](SubmissionsApiMarkBulkSubmissionsAsReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_document_annotations_read_for_courses

> submissions_api_mark_document_annotations_read_for_courses(assignment_id, course_id, user_id)
Mark document annotations as read

Indicate that annotations made on a submitted document have been read by the student. Only the student who owns the submission can use this endpoint.  NOTE: Document annotations will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_document_annotations_read_other

> submissions_api_mark_document_annotations_read_other(section_id, assignment_id, user_id)
Mark document annotations as read

Indicate that annotations made on a submitted document have been read by the student. Only the student who owns the submission can use this endpoint.  NOTE: Document annotations will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_rubric_assessments_read_for_courses

> submissions_api_mark_rubric_assessments_read_for_courses(assignment_id, course_id, user_id)
Mark rubric assessments as read

Indicate that rubric comments/grading made on a submission have been read by the student being assessed. Only the student who owns the submission can use this endpoint.  NOTE: Rubric assessments will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_rubric_assessments_read_for_courses2

> submissions_api_mark_rubric_assessments_read_for_courses2(assignment_id, course_id, user_id)
Mark rubric assessments as read

Indicate that rubric comments/grading made on a submission have been read by the student being assessed. Only the student who owns the submission can use this endpoint.  NOTE: Rubric assessments will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_rubric_assessments_read_other

> submissions_api_mark_rubric_assessments_read_other(section_id, assignment_id, user_id)
Mark rubric assessments as read

Indicate that rubric comments/grading made on a submission have been read by the student being assessed. Only the student who owns the submission can use this endpoint.  NOTE: Rubric assessments will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_rubric_assessments_read_other2

> submissions_api_mark_rubric_assessments_read_other2(section_id, assignment_id, user_id)
Mark rubric assessments as read

Indicate that rubric comments/grading made on a submission have been read by the student being assessed. Only the student who owns the submission can use this endpoint.  NOTE: Rubric assessments will be marked as read automatically when they are viewed in Canvas web.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_item_read_for_courses

> submissions_api_mark_submission_item_read_for_courses(assignment_id, course_id, item, user_id)
Mark submission item as read

No request fields are necessary.  A submission item can be \"grade\", \"comment\" or \"rubric\"  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**item** | **String** | Scope response to item | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_item_read_other

> submissions_api_mark_submission_item_read_other(section_id, assignment_id, item, user_id)
Mark submission item as read

No request fields are necessary.  A submission item can be \"grade\", \"comment\" or \"rubric\"  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**item** | **String** | Scope response to item | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_read_for_courses

> submissions_api_mark_submission_read_for_courses(assignment_id, course_id, user_id)
Mark submission as read

No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_read_other

> submissions_api_mark_submission_read_other(section_id, assignment_id, user_id)
Mark submission as read

No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_unread_for_courses

> submissions_api_mark_submission_unread_for_courses(assignment_id, course_id, user_id)
Mark submission as unread

No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_mark_submission_unread_other

> submissions_api_mark_submission_unread_other(section_id, assignment_id, user_id)
Mark submission as unread

No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_multiple_gradeable_students

> submissions_api_multiple_gradeable_students(course_id, assignment_ids, page, per_page)
List multiple assignments gradeable students

A paginated list of students eligible to submit a list of assignments. The caller must have permission to view grades for the requested course.  Section-limited instructors will only see students in their own sections.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | [String] Assignments being requested |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_rubric_assessments_read_state_for_courses

> submissions_api_rubric_assessments_read_state_for_courses(assignment_id, course_id, user_id)
Get rubric assessments read state

Return whether new rubric comments/grading made on a submission have been seen by the student being assessed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_rubric_assessments_read_state_for_courses2

> submissions_api_rubric_assessments_read_state_for_courses2(assignment_id, course_id, user_id)
Get rubric assessments read state

Return whether new rubric comments/grading made on a submission have been seen by the student being assessed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_rubric_assessments_read_state_other

> submissions_api_rubric_assessments_read_state_other(section_id, assignment_id, user_id)
Get rubric assessments read state

Return whether new rubric comments/grading made on a submission have been seen by the student being assessed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_rubric_assessments_read_state_other2

> submissions_api_rubric_assessments_read_state_other2(section_id, assignment_id, user_id)
Get rubric assessments read state

Return whether new rubric comments/grading made on a submission have been seen by the student being assessed.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_show_anonymous_for_courses

> submissions_api_show_anonymous_for_courses(anonymous_id, assignment_id, course_id, include)
Get a single submission by anonymous id

Get a single submission, based on the submission's anonymous id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anonymous_id** | **String** | Scope response to anonymous_id | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"rubric_assessment\"|\"full_rubric_assessment\"|\"visibility\"|\"course\"|\"user\"|\"read_status\"] Associations to include with the group. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_show_anonymous_other

> submissions_api_show_anonymous_other(section_id, anonymous_id, assignment_id, include)
Get a single submission by anonymous id

Get a single submission, based on the submission's anonymous id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**anonymous_id** | **String** | Scope response to anonymous_id | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"rubric_assessment\"|\"full_rubric_assessment\"|\"visibility\"|\"course\"|\"user\"|\"read_status\"] Associations to include with the group. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_show_for_courses

> submissions_api_show_for_courses(assignment_id, course_id, user_id, include)
Get a single submission

Get a single submission, based on user id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"full_rubric_assessment\"|\"visibility\"|\"course\"|\"user\"|\"read_status\"|\"student_entered_score\"] Associations to include with the group. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_show_other

> submissions_api_show_other(section_id, assignment_id, user_id, include)
Get a single submission

Get a single submission, based on user id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"submission_history\"|\"submission_comments\"|\"submission_html_comments\"|\"rubric_assessment\"|\"full_rubric_assessment\"|\"visibility\"|\"course\"|\"user\"|\"read_status\"|\"student_entered_score\"] Associations to include with the group. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_submission_summary_for_courses

> submissions_api_submission_summary_for_courses(assignment_id, course_id, grouped, include_deactivated)
Submission Summary

Returns the number of submissions for the given assignment based on gradeable students that fall into three categories: graded, ungraded, not submitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**grouped** | Option<**bool**> | If this argument is true, the response will take into account student groups. |  |
**include_deactivated** | Option<**bool**> | If this argument is true, the response will include deactivated students in the summary (defaults to false). |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_submission_summary_other

> submissions_api_submission_summary_other(section_id, assignment_id, grouped, include_deactivated)
Submission Summary

Returns the number of submissions for the given assignment based on gradeable students that fall into three categories: graded, ungraded, not submitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**grouped** | Option<**bool**> | If this argument is true, the response will take into account student groups. |  |
**include_deactivated** | Option<**bool**> | If this argument is true, the response will include deactivated students in the summary (defaults to false). |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_submissions_clear_unread_for_courses

> submissions_api_submissions_clear_unread_for_courses(course_id, user_id)
Clear unread status for all submissions.

Site-admin-only endpoint.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_submissions_clear_unread_other

> submissions_api_submissions_clear_unread_other(section_id, user_id)
Clear unread status for all submissions.

Site-admin-only endpoint.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_update_anonymous_for_courses

> submissions_api_update_anonymous_for_courses(anonymous_id, assignment_id, course_id, submissions_api_update_anonymous_for_courses_request)
Grade or comment on a submission by anonymous id

Comment on and/or update the grading for a student's assignment submission, fetching the submission by anonymous id (instead of user id). If any submission or rubric_assessment arguments are provided, the user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**anonymous_id** | **String** | Scope response to anonymous_id | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**submissions_api_update_anonymous_for_courses_request** | Option<[**SubmissionsApiUpdateAnonymousForCoursesRequest**](SubmissionsApiUpdateAnonymousForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_update_anonymous_other

> submissions_api_update_anonymous_other(section_id, anonymous_id, assignment_id, submissions_api_update_anonymous_for_courses_request)
Grade or comment on a submission by anonymous id

Comment on and/or update the grading for a student's assignment submission, fetching the submission by anonymous id (instead of user id). If any submission or rubric_assessment arguments are provided, the user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**anonymous_id** | **String** | Scope response to anonymous_id | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**submissions_api_update_anonymous_for_courses_request** | Option<[**SubmissionsApiUpdateAnonymousForCoursesRequest**](SubmissionsApiUpdateAnonymousForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_update_for_courses

> submissions_api_update_for_courses(assignment_id, course_id, user_id, submissions_api_update_for_courses_request)
Grade or comment on a submission

Comment on and/or update the grading for a student's assignment submission. If any submission or rubric_assessment arguments are provided, the user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**submissions_api_update_for_courses_request** | Option<[**SubmissionsApiUpdateForCoursesRequest**](SubmissionsApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submissions_api_update_other

> submissions_api_update_other(section_id, assignment_id, user_id, submissions_api_update_for_courses_request)
Grade or comment on a submission

Comment on and/or update the grading for a student's assignment submission. If any submission or rubric_assessment arguments are provided, the user must have permission to manage grades in the appropriate context (course or section).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**submissions_api_update_for_courses_request** | Option<[**SubmissionsApiUpdateForCoursesRequest**](SubmissionsApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

