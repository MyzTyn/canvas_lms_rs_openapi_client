# \TermsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**terms_api_index**](TermsApiApi.md#terms_api_index) | **GET** /accounts/{account_id}/terms | List enrollment terms
[**terms_api_show**](TermsApiApi.md#terms_api_show) | **GET** /accounts/{account_id}/terms/{id} | Retrieve enrollment term



## terms_api_index

> String terms_api_index(account_id, workflow_state, include, term_name, page, per_page)
List enrollment terms

An object with a paginated list of all of the terms in the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**workflow_state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"deleted\"|\"all\"] If set, only returns terms that are in the given state. Defaults to 'active'. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"overrides\"] Array of additional information to include.  \"overrides\":: term start/end dates overridden for different enrollment types \"course_count\":: the number of courses in each term |  |
**term_name** | Option<**String**> | If set, only returns terms that match the given search keyword. Search keyword is matched against term name. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## terms_api_show

> String terms_api_show(account_id, id)
Retrieve enrollment term

Retrieves the details for an enrollment term in the account. Includes overrides by default.

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

