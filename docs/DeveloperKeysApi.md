# \DeveloperKeysApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**developer_keys_create**](DeveloperKeysApi.md#developer_keys_create) | **POST** /accounts/{account_id}/developer_keys | Create a Developer Key
[**developer_keys_destroy**](DeveloperKeysApi.md#developer_keys_destroy) | **DELETE** /developer_keys/{id} | Delete a Developer Key
[**developer_keys_index**](DeveloperKeysApi.md#developer_keys_index) | **GET** /accounts/{account_id}/developer_keys | List Developer Keys
[**developer_keys_lookup_utids**](DeveloperKeysApi.md#developer_keys_lookup_utids) | **GET** /accounts/{account_id}/developer_keys/lookup_utids | Lookup LP API Registrations by Redirect URIs
[**developer_keys_update**](DeveloperKeysApi.md#developer_keys_update) | **PUT** /developer_keys/{id} | Update a Developer Key



## developer_keys_create

> String developer_keys_create(account_id, developer_keys_create_request)
Create a Developer Key

Create a new Canvas API key. Creating an LTI 1.3 registration is not supported here and should be done via the LTI Registration API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**developer_keys_create_request** | Option<[**DeveloperKeysCreateRequest**](DeveloperKeysCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## developer_keys_destroy

> String developer_keys_destroy(id)
Delete a Developer Key

Delete an existing Canvas API key. Deleting an LTI 1.3 registration should be done via the LTI Registration API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## developer_keys_index

> models::DeveloperKey developer_keys_index(account_id, inherited, page, per_page)
List Developer Keys

List all developer keys created in the current account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**inherited** | Option<**bool**> | Defaults to false. If true, lists keys inherited from Site Admin (and consortium parent account, if applicable). |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::DeveloperKey**](DeveloperKey.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## developer_keys_lookup_utids

> serde_json::Value developer_keys_lookup_utids(account_id, redirect_uris, sources)
Lookup LP API Registrations by Redirect URIs

Returns a list of matching Unified Tool IDs (UTIDs) from Learn Platform for the given redirect URIs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**redirect_uris** | [**Vec<String>**](String.md) | An array of redirect URI strings to match | [required] |
**sources** | Option<[**Vec<String>**](String.md)> | An array of sources to look up the redirect URIs from. Possible values are `partner_provided` and `manual` (defaults to both). |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## developer_keys_update

> String developer_keys_update(id, developer_keys_create_request)
Update a Developer Key

Update an existing Canvas API key. Updating an LTI 1.3 registration is not supported here and should be done via the LTI Registration API.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**developer_keys_create_request** | Option<[**DeveloperKeysCreateRequest**](DeveloperKeysCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

