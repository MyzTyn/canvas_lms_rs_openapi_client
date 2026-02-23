# \CourseReportsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**course_reports_create**](CourseReportsApi.md#course_reports_create) | **POST** /courses/{course_id}/reports/{report_type} | Start a Report
[**course_reports_last**](CourseReportsApi.md#course_reports_last) | **GET** /courses/{course_id}/reports/{report_type} | Status of last Report
[**course_reports_show**](CourseReportsApi.md#course_reports_show) | **GET** /courses/{course_id}/reports/{report_type}/{id} | Status of a Report



## course_reports_create

> String course_reports_create(course_id, report_type, course_reports_create_request)
Start a Report

Generates a report instance for the account. Note that \"report\" in the request must match one of the available report names.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course to report on. | [required] |
**report_type** | **String** | The type of report to generate. | [required] |
**course_reports_create_request** | Option<[**CourseReportsCreateRequest**](CourseReportsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_reports_last

> String course_reports_last(course_id, report_type)
Status of last Report

Returns the status of the last report initiated by the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**report_type** | **String** | Scope response to report_type | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_reports_show

> String course_reports_show(course_id, id, report_type)
Status of a Report

Returns the status of a report.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**report_type** | **String** | Scope response to report_type | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

