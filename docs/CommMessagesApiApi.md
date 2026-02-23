# \CommMessagesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**comm_messages_api_index**](CommMessagesApiApi.md#comm_messages_api_index) | **GET** /comm_messages | List of CommMessages for a user



## comm_messages_api_index

> models::CommMessage comm_messages_api_index(user_id, start_time, end_time, page, per_page)
List of CommMessages for a user

Retrieve a paginated list of messages sent to a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | The user id for whom you want to retrieve CommMessages | [required] |
**start_time** | Option<**String**> | The beginning of the time range you want to retrieve message from. Up to a year prior to the current date is available. |  |
**end_time** | Option<**String**> | The end of the time range you want to retrieve messages for. Up to a year prior to the current date is available. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CommMessage**](CommMessage.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

