# \ScopesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**scopes_api_index**](ScopesApiApi.md#scopes_api_index) | **GET** /accounts/{account_id}/scopes | List scopes



## scopes_api_index

> models::Scope scopes_api_index(account_id, group_by)
List scopes

A list of scopes that can be applied to developer keys and access tokens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**group_by** | Option<**String**> | The attribute to group the scopes by. By default no grouping is done. |  |

### Return type

[**models::Scope**](Scope.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

