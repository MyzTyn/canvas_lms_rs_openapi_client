# \DeveloperKeyAccountBindingsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**developer_key_account_bindings_create_or_update**](DeveloperKeyAccountBindingsApi.md#developer_key_account_bindings_create_or_update) | **POST** /accounts/{account_id}/developer_keys/{developer_key_id}/developer_key_account_bindings | Create a Developer Key Account Binding



## developer_key_account_bindings_create_or_update

> String developer_key_account_bindings_create_or_update(account_id, developer_key_id, developer_key_account_bindings_create_or_update_request)
Create a Developer Key Account Binding

Create a new Developer Key Account Binding. The developer key specified in the request URL must be available in the requested account or the requested account's account chain. If the binding already exists for the specified account/key combination it will be updated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**developer_key_id** | **String** | Scope response to developer_key_id | [required] |
**developer_key_account_bindings_create_or_update_request** | Option<[**DeveloperKeyAccountBindingsCreateOrUpdateRequest**](DeveloperKeyAccountBindingsCreateOrUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

