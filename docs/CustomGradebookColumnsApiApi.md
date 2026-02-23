# \CustomGradebookColumnsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custom_gradebook_columns_api_create**](CustomGradebookColumnsApiApi.md#custom_gradebook_columns_api_create) | **POST** /courses/{course_id}/custom_gradebook_columns | Create a custom gradebook column
[**custom_gradebook_columns_api_destroy**](CustomGradebookColumnsApiApi.md#custom_gradebook_columns_api_destroy) | **DELETE** /courses/{course_id}/custom_gradebook_columns/{id} | Delete a custom gradebook column
[**custom_gradebook_columns_api_index**](CustomGradebookColumnsApiApi.md#custom_gradebook_columns_api_index) | **GET** /courses/{course_id}/custom_gradebook_columns | List custom gradebook columns
[**custom_gradebook_columns_api_reorder**](CustomGradebookColumnsApiApi.md#custom_gradebook_columns_api_reorder) | **POST** /courses/{course_id}/custom_gradebook_columns/reorder | Reorder custom columns
[**custom_gradebook_columns_api_update**](CustomGradebookColumnsApiApi.md#custom_gradebook_columns_api_update) | **PUT** /courses/{course_id}/custom_gradebook_columns/{id} | Update a custom gradebook column



## custom_gradebook_columns_api_create

> String custom_gradebook_columns_api_create(course_id, custom_gradebook_columns_api_create_request)
Create a custom gradebook column

Create a custom gradebook column

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**custom_gradebook_columns_api_create_request** | Option<[**CustomGradebookColumnsApiCreateRequest**](CustomGradebookColumnsApiCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_gradebook_columns_api_destroy

> String custom_gradebook_columns_api_destroy(course_id, id)
Delete a custom gradebook column

Permanently deletes a custom column and its associated data

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


## custom_gradebook_columns_api_index

> models::CustomColumn custom_gradebook_columns_api_index(course_id, include_hidden, page, per_page)
List custom gradebook columns

A paginated list of all custom gradebook columns for a course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**include_hidden** | Option<**bool**> | Include hidden parameters (defaults to false) |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CustomColumn**](CustomColumn.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_gradebook_columns_api_reorder

> custom_gradebook_columns_api_reorder(course_id, custom_gradebook_columns_api_reorder_request)
Reorder custom columns

Puts the given columns in the specified order  <b>200 OK</b> is returned if successful

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**custom_gradebook_columns_api_reorder_request** | Option<[**CustomGradebookColumnsApiReorderRequest**](CustomGradebookColumnsApiReorderRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_gradebook_columns_api_update

> String custom_gradebook_columns_api_update(course_id, id)
Update a custom gradebook column

Accepts the same parameters as custom gradebook column creation

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

