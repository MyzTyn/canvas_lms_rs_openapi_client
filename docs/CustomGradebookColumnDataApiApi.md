# \CustomGradebookColumnDataApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**custom_gradebook_column_data_api_bulk_update**](CustomGradebookColumnDataApiApi.md#custom_gradebook_column_data_api_bulk_update) | **PUT** /courses/{course_id}/custom_gradebook_column_data | Bulk update column data
[**custom_gradebook_column_data_api_index**](CustomGradebookColumnDataApiApi.md#custom_gradebook_column_data_api_index) | **GET** /courses/{course_id}/custom_gradebook_columns/{id}/data | List entries for a column
[**custom_gradebook_column_data_api_update**](CustomGradebookColumnDataApiApi.md#custom_gradebook_column_data_api_update) | **PUT** /courses/{course_id}/custom_gradebook_columns/{id}/data/{user_id} | Update column data



## custom_gradebook_column_data_api_bulk_update

> String custom_gradebook_column_data_api_bulk_update(course_id, custom_gradebook_column_data_api_bulk_update_request)
Bulk update column data

Set the content of custom columns  {   \"column_data\": [     {       \"column_id\": example_column_id,       \"user_id\": example_student_id,       \"content\": example_content       },       {       \"column_id\": example_column_id,       \"user_id\": example_student_id,       \"content: example_content     }   ] }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**custom_gradebook_column_data_api_bulk_update_request** | Option<[**CustomGradebookColumnDataApiBulkUpdateRequest**](CustomGradebookColumnDataApiBulkUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_gradebook_column_data_api_index

> models::ColumnDatum custom_gradebook_column_data_api_index(course_id, id, include_hidden, page, per_page)
List entries for a column

This does not list entries for students without associated data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**include_hidden** | Option<**bool**> | If true, hidden columns will be included in the result. If false or absent, only visible columns will be returned. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::ColumnDatum**](ColumnDatum.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## custom_gradebook_column_data_api_update

> String custom_gradebook_column_data_api_update(course_id, id, user_id, custom_gradebook_column_data_api_update_request)
Update column data

Set the content of a custom column

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**custom_gradebook_column_data_api_update_request** | Option<[**CustomGradebookColumnDataApiUpdateRequest**](CustomGradebookColumnDataApiUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

