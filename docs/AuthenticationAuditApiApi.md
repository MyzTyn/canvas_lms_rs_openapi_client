# \AuthenticationAuditApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**authentication_audit_api_for_account**](AuthenticationAuditApiApi.md#authentication_audit_api_for_account) | **GET** /audit/authentication/accounts/{account_id} | Query by account.
[**authentication_audit_api_for_login**](AuthenticationAuditApiApi.md#authentication_audit_api_for_login) | **GET** /audit/authentication/logins/{login_id} | Query by login.
[**authentication_audit_api_for_user**](AuthenticationAuditApiApi.md#authentication_audit_api_for_user) | **GET** /audit/authentication/users/{user_id} | Query by user.



## authentication_audit_api_for_account

> authentication_audit_api_for_account(account_id, start_time, end_time, page, per_page)
Query by account.

List authentication events for a given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. Events are stored for one year. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authentication_audit_api_for_login

> authentication_audit_api_for_login(login_id, start_time, end_time, page, per_page)
Query by login.

List authentication events for a given login.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**login_id** | **String** | Scope response to login_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. Events are stored for one year. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## authentication_audit_api_for_user

> authentication_audit_api_for_user(user_id, start_time, end_time, page, per_page)
Query by user.

List authentication events for a given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**start_time** | Option<**String**> | The beginning of the time range from which you want events. Events are stored for one year. |  |
**end_time** | Option<**String**> | The end of the time range from which you want events. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

