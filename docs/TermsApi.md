# \TermsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**terms_create**](TermsApi.md#terms_create) | **POST** /accounts/{account_id}/terms | Create enrollment term
[**terms_destroy**](TermsApi.md#terms_destroy) | **DELETE** /accounts/{account_id}/terms/{id} | Delete enrollment term
[**terms_update**](TermsApi.md#terms_update) | **PUT** /accounts/{account_id}/terms/{id} | Update enrollment term



## terms_create

> String terms_create(account_id, terms_create_request)
Create enrollment term

Create a new enrollment term for the specified account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**terms_create_request** | Option<[**TermsCreateRequest**](TermsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terms_destroy

> String terms_destroy(account_id, id)
Delete enrollment term

Delete the specified enrollment term.

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


## terms_update

> String terms_update(account_id, id, terms_update_request)
Update enrollment term

Update an existing enrollment term for the specified account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**terms_update_request** | Option<[**TermsUpdateRequest**](TermsUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

