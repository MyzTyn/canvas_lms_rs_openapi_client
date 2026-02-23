# \GradebookHistoryApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gradebook_history_api_day_details**](GradebookHistoryApiApi.md#gradebook_history_api_day_details) | **GET** /courses/{course_id}/gradebook_history/{date} | Details for a given date in gradebook history for this course
[**gradebook_history_api_days**](GradebookHistoryApiApi.md#gradebook_history_api_days) | **GET** /courses/{course_id}/gradebook_history/days | Days in gradebook history for this course
[**gradebook_history_api_feed**](GradebookHistoryApiApi.md#gradebook_history_api_feed) | **GET** /courses/{course_id}/gradebook_history/feed | List uncollated submission versions
[**gradebook_history_api_submissions**](GradebookHistoryApiApi.md#gradebook_history_api_submissions) | **GET** /courses/{course_id}/gradebook_history/{date}/graders/{grader_id}/assignments/{assignment_id}/submissions | Lists submissions



## gradebook_history_api_day_details

> models::Grader gradebook_history_api_day_details(course_id, date)
Details for a given date in gradebook history for this course

Returns the graders who worked on this day, along with the assignments they worked on. More details can be obtained by selecting a grader and assignment and calling the 'submissions' api endpoint for a given date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the contextual course for this API call | [required] |
**date** | **String** | The date for which you would like to see detailed information | [required] |

### Return type

[**models::Grader**](Grader.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gradebook_history_api_days

> models::Day gradebook_history_api_days(course_id)
Days in gradebook history for this course

Returns a map of dates to grader/assignment groups

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the contextual course for this API call | [required] |

### Return type

[**models::Day**](Day.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gradebook_history_api_feed

> models::SubmissionVersion gradebook_history_api_feed(course_id, assignment_id, user_id, ascending, page, per_page)
List uncollated submission versions

Gives a paginated, uncollated list of submission versions for all matching submissions in the context. This SubmissionVersion objects will not include the +new_grade+ or +previous_grade+ keys, only the +grade+; same for +graded_at+ and +grader+.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the contextual course for this API call | [required] |
**assignment_id** | Option<**i32**> | The ID of the assignment for which you want to see submissions. If absent, versions of submissions from any assignment in the course are included. |  |
**user_id** | Option<**i32**> | The ID of the user for which you want to see submissions. If absent, versions of submissions from any user in the course are included. |  |
**ascending** | Option<**bool**> | Returns submission versions in ascending date order (oldest first). If absent, returns submission versions in descending date order (newest first). |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::SubmissionVersion**](SubmissionVersion.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gradebook_history_api_submissions

> models::SubmissionHistory gradebook_history_api_submissions(assignment_id, course_id, date, grader_id)
Lists submissions

Gives a nested list of submission versions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | The ID of the assignment for which you want to see submissions | [required] |
**course_id** | **String** | The id of the contextual course for this API call | [required] |
**date** | **String** | The date for which you would like to see submissions | [required] |
**grader_id** | **String** | The ID of the grader for which you want to see submissions | [required] |

### Return type

[**models::SubmissionHistory**](SubmissionHistory.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

