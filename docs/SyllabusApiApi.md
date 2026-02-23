# \SyllabusApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**syllabus_api_accessibility_queue_scan**](SyllabusApiApi.md#syllabus_api_accessibility_queue_scan) | **POST** /courses/{course_id}/syllabus/accessibility/queue_scan | Queue syllabus accessibility scan
[**syllabus_api_accessibility_scan**](SyllabusApiApi.md#syllabus_api_accessibility_scan) | **POST** /courses/{course_id}/syllabus/accessibility/scan | Scan syllabus for accessibility issues



## syllabus_api_accessibility_queue_scan

> String syllabus_api_accessibility_queue_scan(course_id)
Queue syllabus accessibility scan

Queues a course syllabus for accessibility scanning.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## syllabus_api_accessibility_scan

> String syllabus_api_accessibility_scan(course_id)
Scan syllabus for accessibility issues

Scans a course syllabus for accessibility issues and returns the results.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

