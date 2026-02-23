# \LmgbUserDetailsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**lmgb_user_details_show**](LmgbUserDetailsApi.md#lmgb_user_details_show) | **GET** /courses/{course_id}/users/{id}/lmgb_user_details | Get LMGB user details



## lmgb_user_details_show

> String lmgb_user_details_show(course_id, id)
Get LMGB user details

Returns details about a user in the context of a course for LMGB

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | The ID of the user to retrieve details for | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

