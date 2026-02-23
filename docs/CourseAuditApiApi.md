# \CourseAuditApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**course_audit_api_for_account**](CourseAuditApiApi.md#course_audit_api_for_account) | **GET** /audit/course/accounts/{account_id} | Query by account.
[**course_audit_api_for_course**](CourseAuditApiApi.md#course_audit_api_for_course) | **GET** /audit/course/courses/{course_id} | Query by course.



## course_audit_api_for_account

> models::CourseEvent course_audit_api_for_account(account_id, start_time, end_time, page, per_page)
Query by account.

List course change events for a given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CourseEvent**](CourseEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_audit_api_for_course

> models::CourseEvent course_audit_api_for_course(course_id, start_time, end_time, page, per_page)
Query by course.

List course change events for a given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CourseEvent**](CourseEvent.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

