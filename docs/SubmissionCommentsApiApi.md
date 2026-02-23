# \SubmissionCommentsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**submission_comments_api_create_file**](SubmissionCommentsApiApi.md#submission_comments_api_create_file) | **POST** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/comments/files | Upload a file
[**submission_comments_api_destroy**](SubmissionCommentsApiApi.md#submission_comments_api_destroy) | **DELETE** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/comments/{id} | Delete a submission comment
[**submission_comments_api_update**](SubmissionCommentsApiApi.md#submission_comments_api_update) | **PUT** /courses/{course_id}/assignments/{assignment_id}/submissions/{user_id}/comments/{id} | Edit a submission comment



## submission_comments_api_create_file

> submission_comments_api_create_file(assignment_id, course_id, user_id)
Upload a file

Upload a file to attach to a submission comment  See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  The final step of the file upload workflow will return the attachment data, including the new file id. The caller can then PUT the file_id to the submission API to attach it to a comment

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


## submission_comments_api_destroy

> String submission_comments_api_destroy(assignment_id, course_id, id, user_id)
Delete a submission comment

Delete the given submission comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## submission_comments_api_update

> String submission_comments_api_update(assignment_id, course_id, id, user_id, submission_comments_api_update_request)
Edit a submission comment

Edit the given submission comment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**submission_comments_api_update_request** | Option<[**SubmissionCommentsApiUpdateRequest**](SubmissionCommentsApiUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

