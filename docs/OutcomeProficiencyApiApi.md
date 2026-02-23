# \OutcomeProficiencyApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outcome_proficiency_api_create_for_accounts**](OutcomeProficiencyApiApi.md#outcome_proficiency_api_create_for_accounts) | **POST** /accounts/{account_id}/outcome_proficiency | Create/update proficiency ratings
[**outcome_proficiency_api_create_for_courses**](OutcomeProficiencyApiApi.md#outcome_proficiency_api_create_for_courses) | **POST** /courses/{course_id}/outcome_proficiency | Create/update proficiency ratings
[**outcome_proficiency_api_show_for_accounts**](OutcomeProficiencyApiApi.md#outcome_proficiency_api_show_for_accounts) | **GET** /accounts/{account_id}/outcome_proficiency | Get proficiency ratings
[**outcome_proficiency_api_show_for_courses**](OutcomeProficiencyApiApi.md#outcome_proficiency_api_show_for_courses) | **GET** /courses/{course_id}/outcome_proficiency | Get proficiency ratings



## outcome_proficiency_api_create_for_accounts

> String outcome_proficiency_api_create_for_accounts(account_id, outcome_proficiency_api_create_for_accounts_request)
Create/update proficiency ratings

Create or update account-level proficiency ratings. These ratings will apply to all sub-accounts, unless they have their own account-level proficiency ratings defined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**outcome_proficiency_api_create_for_accounts_request** | Option<[**OutcomeProficiencyApiCreateForAccountsRequest**](OutcomeProficiencyApiCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_proficiency_api_create_for_courses

> String outcome_proficiency_api_create_for_courses(course_id, outcome_proficiency_api_create_for_accounts_request)
Create/update proficiency ratings

Create or update account-level proficiency ratings. These ratings will apply to all sub-accounts, unless they have their own account-level proficiency ratings defined.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**outcome_proficiency_api_create_for_accounts_request** | Option<[**OutcomeProficiencyApiCreateForAccountsRequest**](OutcomeProficiencyApiCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_proficiency_api_show_for_accounts

> String outcome_proficiency_api_show_for_accounts(account_id)
Get proficiency ratings

Get account-level proficiency ratings. If not defined for this account, it will return proficiency ratings for the nearest super-account with ratings defined. Will return 404 if none found.    Examples:     curl https://<canvas>/api/v1/accounts/<account_id>/outcome_proficiency \\         -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_proficiency_api_show_for_courses

> String outcome_proficiency_api_show_for_courses(course_id)
Get proficiency ratings

Get account-level proficiency ratings. If not defined for this account, it will return proficiency ratings for the nearest super-account with ratings defined. Will return 404 if none found.    Examples:     curl https://<canvas>/api/v1/accounts/<account_id>/outcome_proficiency \\         -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

