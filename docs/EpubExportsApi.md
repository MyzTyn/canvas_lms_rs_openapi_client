# \EpubExportsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**epub_exports_create**](EpubExportsApi.md#epub_exports_create) | **POST** /courses/{course_id}/epub_exports | Create ePub Export
[**epub_exports_index**](EpubExportsApi.md#epub_exports_index) | **GET** /epub_exports | List courses with their latest ePub export
[**epub_exports_show**](EpubExportsApi.md#epub_exports_show) | **GET** /courses/{course_id}/epub_exports/{id} | Show ePub export



## epub_exports_create

> String epub_exports_create(course_id)
Create ePub Export

Begin an ePub export for a course.  You can use the {api:ProgressController#show Progress API} to track the progress of the export. The export's progress is linked to with the _progress_url_ value.  When the export completes, use the {api:EpubExportsController#show Show content export} endpoint to retrieve a download URL for the exported content.

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


## epub_exports_index

> models::CourseEpubExport epub_exports_index()
List courses with their latest ePub export

A paginated list of all courses a user is actively participating in, and the latest ePub export associated with the user & course.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::CourseEpubExport**](CourseEpubExport.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## epub_exports_show

> String epub_exports_show(course_id, id)
Show ePub export

Get information about a single ePub export.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

