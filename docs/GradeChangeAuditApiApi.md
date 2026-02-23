# \GradeChangeAuditApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grade_change_audit_api_for_assignment**](GradeChangeAuditApiApi.md#grade_change_audit_api_for_assignment) | **GET** /audit/grade_change/assignments/{assignment_id} | Query by assignment
[**grade_change_audit_api_for_course**](GradeChangeAuditApiApi.md#grade_change_audit_api_for_course) | **GET** /audit/grade_change/courses/{course_id} | Query by course
[**grade_change_audit_api_for_grader**](GradeChangeAuditApiApi.md#grade_change_audit_api_for_grader) | **GET** /audit/grade_change/graders/{grader_id} | Query by grader
[**grade_change_audit_api_for_student**](GradeChangeAuditApiApi.md#grade_change_audit_api_for_student) | **GET** /audit/grade_change/students/{student_id} | Query by student
[**grade_change_audit_api_query**](GradeChangeAuditApiApi.md#grade_change_audit_api_query) | **GET** /audit/grade_change | Advanced query



## grade_change_audit_api_for_assignment

> models::GradeChangeEvent grade_change_audit_api_for_assignment(assignment_id, start_time, end_time, page, per_page)
Query by assignment

List grade change events for a given assignment.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GradeChangeEvent**](GradeChangeEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grade_change_audit_api_for_course

> models::GradeChangeEvent grade_change_audit_api_for_course(course_id, start_time, end_time, page, per_page)
Query by course

List grade change events for a given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GradeChangeEvent**](GradeChangeEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grade_change_audit_api_for_grader

> models::GradeChangeEvent grade_change_audit_api_for_grader(grader_id, start_time, end_time, page, per_page)
Query by grader

List grade change events for a given grader.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grader_id** | **String** | Scope response to grader_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GradeChangeEvent**](GradeChangeEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grade_change_audit_api_for_student

> models::GradeChangeEvent grade_change_audit_api_for_student(student_id, start_time, end_time, page, per_page)
Query by student

List grade change events for a given student.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**student_id** | **String** | Scope response to student_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GradeChangeEvent**](GradeChangeEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grade_change_audit_api_query

> models::GradeChangeEvent grade_change_audit_api_query(course_id, assignment_id, student_id, grader_id, start_time, end_time, page, per_page)
Advanced query

List grade change events satisfying all given parameters. Teachers may query for events in courses they teach. Queries without +course_id+ require account administrator rights.  At least one of +course_id+, +assignment_id+, +student_id+, or +grader_id+ must be specified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | Option<**i32**> | Restrict query to events in the specified course. |  |
**assignment_id** | Option<**i32**> | Restrict query to the given assignment. If \"override\" is given, query the course final grade override instead. |  |
**student_id** | Option<**i32**> | User id of a student to search grading events for. |  |
**grader_id** | Option<**i32**> | User id of a grader to search grading events for. |  |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GradeChangeEvent**](GradeChangeEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

