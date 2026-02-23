# \WebZipExportsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**web_zip_exports_index**](WebZipExportsApi.md#web_zip_exports_index) | **GET** /courses/{course_id}/web_zip_exports | List all web zip exports in a course
[**web_zip_exports_show**](WebZipExportsApi.md#web_zip_exports_show) | **GET** /courses/{course_id}/web_zip_exports/{id} | Show WebZipExport



## web_zip_exports_index

> models::WebZipExport web_zip_exports_index(course_id, page, per_page)
List all web zip exports in a course

A paginated list of all web zip exports in a course for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::WebZipExport**](WebZipExport.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## web_zip_exports_show

> String web_zip_exports_show(course_id, id)
Show WebZipExport

Get information about a single WebZipExport.

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

