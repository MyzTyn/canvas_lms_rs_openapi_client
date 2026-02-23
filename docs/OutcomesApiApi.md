# \OutcomesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outcomes_api_outcome_alignments**](OutcomesApiApi.md#outcomes_api_outcome_alignments) | **GET** /courses/{course_id}/outcome_alignments | Get outcome alignments for a student or assignment
[**outcomes_api_show**](OutcomesApiApi.md#outcomes_api_show) | **GET** /outcomes/{id} | Show an outcome
[**outcomes_api_update**](OutcomesApiApi.md#outcomes_api_update) | **PUT** /outcomes/{id} | Update an outcome



## outcomes_api_outcome_alignments

> models::OutcomeAlignment outcomes_api_outcome_alignments(course_id, student_id, assignment_id)
Get outcome alignments for a student or assignment

Returns outcome alignments for a student or assignment in a course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**student_id** | Option<**i32**> | The id of the student. Returns alignments filtered by student submissions. Can be combined with assignment_id to filter to a specific assignment. |  |
**assignment_id** | Option<**i32**> | The id of the assignment. When provided without student_id, returns all outcome alignments for the assignment (requires manage_grades or view_all_grades permission). When provided with student_id, filters to that student's submission. |  |

### Return type

[**models::OutcomeAlignment**](OutcomeAlignment.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcomes_api_show

> String outcomes_api_show(id, add_defaults)
Show an outcome

Returns the details of the outcome with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**add_defaults** | Option<**bool**> | If defaults are requested, then color and mastery level defaults will be added to outcome ratings in the result. This will only take effect if the Account Level Mastery Scales FF is DISABLED |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcomes_api_update

> String outcomes_api_update(id, outcomes_api_update_request)
Update an outcome

Modify an existing outcome. Fields not provided are left as is; unrecognized fields are ignored.  If any new ratings are provided, the combination of all new ratings provided completely replace any existing embedded rubric criterion; it is not possible to tweak the ratings of the embedded rubric criterion.  A new embedded rubric criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any new ratings lacking a description are given a default of \"No description\". Any new ratings lacking a point value are given a default of 0.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcomes_api_update_request** | Option<[**OutcomesApiUpdateRequest**](OutcomesApiUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/outcomes/1.json' \\      -X PUT \\      -F 'title=Outcome Title' \\      -F 'display_name=Title for reporting' \\      -F 'description=Outcome description' \\      -F 'vendor_guid=customid9001' \\      -F 'mastery_points=3' \\      -F 'calculation_method=decaying_average' \\      -F 'calculation_int=65' \\      -F 'ratings[][description]=Exceeds Expectations' \\      -F 'ratings[][points]=5' \\      -F 'ratings[][description]=Meets Expectations' \\      -F 'ratings[][points]=3' \\      -F 'ratings[][description]=Does Not Meet Expectations' \\      -F 'ratings[][points]=0' \\      -F 'ratings[][points]=0' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

