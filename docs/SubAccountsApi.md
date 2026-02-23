# \SubAccountsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sub_accounts_create**](SubAccountsApi.md#sub_accounts_create) | **POST** /accounts/{account_id}/sub_accounts | Create a new sub-account
[**sub_accounts_destroy**](SubAccountsApi.md#sub_accounts_destroy) | **DELETE** /accounts/{account_id}/sub_accounts/{id} | Delete a sub-account



## sub_accounts_create

> String sub_accounts_create(account_id, sub_accounts_create_request)
Create a new sub-account

Add a new sub-account to a given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**sub_accounts_create_request** | Option<[**SubAccountsCreateRequest**](SubAccountsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sub_accounts_destroy

> String sub_accounts_destroy(account_id, id)
Delete a sub-account

Cannot delete an account with active courses or active sub_accounts. Cannot delete a root_account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

