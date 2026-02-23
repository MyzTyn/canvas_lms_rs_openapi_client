# \HistoryApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**history_index**](HistoryApi.md#history_index) | **GET** /users/{user_id}/history | List recent history for a user



## history_index

> models::HistoryEntry history_index(user_id, page, per_page)
List recent history for a user

Return a paginated list of the user's recent history. History entries are returned in descending order, newest to oldest. You may list history entries for yourself (use +self+ as the user_id), for a student you observe, or for a user you manage as an administrator. Note that the +per_page+ pagination argument is not supported and the number of history entries returned per page will vary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::HistoryEntry**](HistoryEntry.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

