# \CspSettingsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**csp_settings_add_domain**](CspSettingsApi.md#csp_settings_add_domain) | **POST** /accounts/{account_id}/csp_settings/domains | Add an allowed domain to account
[**csp_settings_add_multiple_domains**](CspSettingsApi.md#csp_settings_add_multiple_domains) | **POST** /accounts/{account_id}/csp_settings/domains/batch_create | Add multiple allowed domains to an account
[**csp_settings_get_csp_settings_for_accounts**](CspSettingsApi.md#csp_settings_get_csp_settings_for_accounts) | **GET** /accounts/{account_id}/csp_settings | Get current settings for account or course
[**csp_settings_get_csp_settings_for_courses**](CspSettingsApi.md#csp_settings_get_csp_settings_for_courses) | **GET** /courses/{course_id}/csp_settings | Get current settings for account or course
[**csp_settings_remove_domain**](CspSettingsApi.md#csp_settings_remove_domain) | **DELETE** /accounts/{account_id}/csp_settings/domains | Remove a domain from account
[**csp_settings_set_csp_lock**](CspSettingsApi.md#csp_settings_set_csp_lock) | **PUT** /accounts/{account_id}/csp_settings/lock | Lock or unlock current CSP settings for sub-accounts and courses
[**csp_settings_set_csp_setting_for_accounts**](CspSettingsApi.md#csp_settings_set_csp_setting_for_accounts) | **PUT** /accounts/{account_id}/csp_settings | Enable, disable, or clear explicit CSP setting
[**csp_settings_set_csp_setting_for_courses**](CspSettingsApi.md#csp_settings_set_csp_setting_for_courses) | **PUT** /courses/{course_id}/csp_settings | Enable, disable, or clear explicit CSP setting



## csp_settings_add_domain

> csp_settings_add_domain(account_id, csp_settings_add_domain_request)
Add an allowed domain to account

Adds an allowed domain for the current account. Note: this will not take effect unless CSP is explicitly enabled on this account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**csp_settings_add_domain_request** | Option<[**CspSettingsAddDomainRequest**](CspSettingsAddDomainRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_add_multiple_domains

> csp_settings_add_multiple_domains(account_id, csp_settings_add_multiple_domains_request)
Add multiple allowed domains to an account

Adds multiple allowed domains for the current account. Note: this will not take effect unless CSP is explicitly enabled on this account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**csp_settings_add_multiple_domains_request** | Option<[**CspSettingsAddMultipleDomainsRequest**](CspSettingsAddMultipleDomainsRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_get_csp_settings_for_accounts

> csp_settings_get_csp_settings_for_accounts(account_id)
Get current settings for account or course

Update multiple modules in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_get_csp_settings_for_courses

> csp_settings_get_csp_settings_for_courses(course_id)
Get current settings for account or course

Update multiple modules in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_remove_domain

> csp_settings_remove_domain(account_id, domain)
Remove a domain from account

Removes an allowed domain from the current account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**domain** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_set_csp_lock

> csp_settings_set_csp_lock(account_id, csp_settings_set_csp_lock_request)
Lock or unlock current CSP settings for sub-accounts and courses

Can only be set if CSP is explicitly enabled or disabled on this account (i.e. \"inherited\" is false).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**csp_settings_set_csp_lock_request** | Option<[**CspSettingsSetCspLockRequest**](CspSettingsSetCspLockRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_set_csp_setting_for_accounts

> csp_settings_set_csp_setting_for_accounts(account_id, csp_settings_set_csp_setting_for_courses_request)
Enable, disable, or clear explicit CSP setting

Either explicitly sets CSP to be on or off for courses and sub-accounts, or clear the explicit settings to default to those set by a parent account  Note: If \"inherited\" and \"settings_locked\" are both true for this account or course, then the CSP setting cannot be modified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**csp_settings_set_csp_setting_for_courses_request** | Option<[**CspSettingsSetCspSettingForCoursesRequest**](CspSettingsSetCspSettingForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## csp_settings_set_csp_setting_for_courses

> csp_settings_set_csp_setting_for_courses(course_id, csp_settings_set_csp_setting_for_courses_request)
Enable, disable, or clear explicit CSP setting

Either explicitly sets CSP to be on or off for courses and sub-accounts, or clear the explicit settings to default to those set by a parent account  Note: If \"inherited\" and \"settings_locked\" are both true for this account or course, then the CSP setting cannot be modified.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**csp_settings_set_csp_setting_for_courses_request** | Option<[**CspSettingsSetCspSettingForCoursesRequest**](CspSettingsSetCspSettingForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

