# \ProvisionalGradesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**provisional_grades_bulk_select**](ProvisionalGradesApi.md#provisional_grades_bulk_select) | **PUT** /courses/{course_id}/assignments/{assignment_id}/provisional_grades/bulk_select | Bulk select provisional grades
[**provisional_grades_publish**](ProvisionalGradesApi.md#provisional_grades_publish) | **POST** /courses/{course_id}/assignments/{assignment_id}/provisional_grades/publish | Publish provisional grades for an assignment
[**provisional_grades_select**](ProvisionalGradesApi.md#provisional_grades_select) | **PUT** /courses/{course_id}/assignments/{assignment_id}/provisional_grades/{provisional_grade_id}/select | Select provisional grade
[**provisional_grades_status**](ProvisionalGradesApi.md#provisional_grades_status) | **GET** /courses/{course_id}/assignments/{assignment_id}/provisional_grades/status | Show provisional grade status for a student



## provisional_grades_bulk_select

> provisional_grades_bulk_select(assignment_id, course_id)
Bulk select provisional grades

Choose which provisional grades will be received by associated students for an assignment. The caller must be the final grader for the assignment or an admin with :select_final_grade rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provisional_grades_publish

> provisional_grades_publish(assignment_id, course_id)
Publish provisional grades for an assignment

Publish the selected provisional grade for all submissions to an assignment. Use the \"Select provisional grade\" endpoint to choose which provisional grade to publish for a particular submission.  Students not in the moderation set will have their one and only provisional grade published.  WARNING: This is irreversible. This will overwrite existing grades in the gradebook.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provisional_grades_select

> provisional_grades_select(assignment_id, course_id, provisional_grade_id)
Select provisional grade

Choose which provisional grade the student should receive for a submission. The caller must be the final grader for the assignment or an admin with :select_final_grade rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**provisional_grade_id** | **String** | Scope response to provisional_grade_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## provisional_grades_status

> provisional_grades_status(assignment_id, course_id, student_id)
Show provisional grade status for a student

Tell whether the student's submission needs one or more provisional grades.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**student_id** | Option<**i32**> | The id of the student to show the status for |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

