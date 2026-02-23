# \AccountCalendarsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_calendars_api_all_calendars**](AccountCalendarsApiApi.md#account_calendars_api_all_calendars) | **GET** /accounts/{account_id}/account_calendars | List all account calendars
[**account_calendars_api_bulk_update**](AccountCalendarsApiApi.md#account_calendars_api_bulk_update) | **PUT** /accounts/{account_id}/account_calendars | Update several calendars
[**account_calendars_api_index**](AccountCalendarsApiApi.md#account_calendars_api_index) | **GET** /account_calendars | List available account calendars
[**account_calendars_api_show**](AccountCalendarsApiApi.md#account_calendars_api_show) | **GET** /account_calendars/{account_id} | Get a single account calendar
[**account_calendars_api_update**](AccountCalendarsApiApi.md#account_calendars_api_update) | **PUT** /account_calendars/{account_id} | Update a calendar
[**account_calendars_api_visible_calendars_count**](AccountCalendarsApiApi.md#account_calendars_api_visible_calendars_count) | **GET** /accounts/{account_id}/visible_calendars_count | Count of all visible account calendars



## account_calendars_api_all_calendars

> models::AccountCalendar account_calendars_api_all_calendars(account_id, search_term, filter, page, per_page)
List all account calendars

Returns a paginated list of account calendars for the provided account and its first level of sub-accounts. Includes hidden calendars in the response. Requires the `manage_account_calendar_visibility` permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**search_term** | Option<**String**> | When included, searches all descendent accounts of provided account for the term. Returns matching results. Term must be at least 2 characters. Can be combined with a filter value. |  |
**filter** | Option<**String**> | When included, only returns calendars that are either visible or hidden. Can be combined with a search term. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::AccountCalendar**](AccountCalendar.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_calendars_api_bulk_update

> account_calendars_api_bulk_update(account_id)
Update several calendars

Set visibility and/or auto_subscribe on many calendars simultaneously. Requires the `manage_account_calendar_visibility` permission on the account.  Accepts a JSON array of objects containing 2-3 keys each: `id` (the account's id, required), `visible` (a boolean indicating whether the account calendar is visible), and `auto_subscribe` (a boolean indicating whether users should see these events in their calendar without manually subscribing).  Returns the count of updated accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_calendars_api_index

> serde_json::Value account_calendars_api_index(search_term, page, per_page)
List available account calendars

Returns a paginated list of account calendars available to the current user. Includes visible account calendars where the user has an account association.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search_term** | Option<**String**> | When included, searches available account calendars for the term. Returns matching results. Term must be at least 2 characters. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_calendars_api_show

> String account_calendars_api_show(account_id)
Get a single account calendar

Get details about a specific account calendar.

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


## account_calendars_api_update

> String account_calendars_api_update(account_id, account_calendars_api_update_request)
Update a calendar

Set an account calendar's visibility and auto_subscribe values. Requires the `manage_account_calendar_visibility` permission on the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**account_calendars_api_update_request** | Option<[**AccountCalendarsApiUpdateRequest**](AccountCalendarsApiUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/account_calendars/204 \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'visible=false' \\   -d 'auto_subscribe=false' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_calendars_api_visible_calendars_count

> serde_json::Value account_calendars_api_visible_calendars_count(account_id)
Count of all visible account calendars

Returns the number of visible account calendars.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

