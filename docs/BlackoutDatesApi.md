# \BlackoutDatesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**blackout_dates_bulk_update**](BlackoutDatesApi.md#blackout_dates_bulk_update) | **PUT** /courses/{course_id}/blackout_dates | Update a list of Blackout Dates
[**blackout_dates_create_for_accounts**](BlackoutDatesApi.md#blackout_dates_create_for_accounts) | **POST** /accounts/{account_id}/blackout_dates | Create Blackout Date
[**blackout_dates_create_for_courses**](BlackoutDatesApi.md#blackout_dates_create_for_courses) | **POST** /courses/{course_id}/blackout_dates | Create Blackout Date
[**blackout_dates_destroy_for_accounts**](BlackoutDatesApi.md#blackout_dates_destroy_for_accounts) | **DELETE** /accounts/{account_id}/blackout_dates/{id} | Delete Blackout Date
[**blackout_dates_destroy_for_courses**](BlackoutDatesApi.md#blackout_dates_destroy_for_courses) | **DELETE** /courses/{course_id}/blackout_dates/{id} | Delete Blackout Date
[**blackout_dates_index_for_accounts**](BlackoutDatesApi.md#blackout_dates_index_for_accounts) | **GET** /accounts/{account_id}/blackout_dates | List blackout dates
[**blackout_dates_index_for_courses**](BlackoutDatesApi.md#blackout_dates_index_for_courses) | **GET** /courses/{course_id}/blackout_dates | List blackout dates
[**blackout_dates_new_for_accounts**](BlackoutDatesApi.md#blackout_dates_new_for_accounts) | **GET** /accounts/{account_id}/blackout_dates/new | New Blackout Date
[**blackout_dates_new_for_courses**](BlackoutDatesApi.md#blackout_dates_new_for_courses) | **GET** /courses/{course_id}/blackout_dates/new | New Blackout Date
[**blackout_dates_show_for_accounts**](BlackoutDatesApi.md#blackout_dates_show_for_accounts) | **GET** /accounts/{account_id}/blackout_dates/{id} | Get a single blackout date
[**blackout_dates_show_for_courses**](BlackoutDatesApi.md#blackout_dates_show_for_courses) | **GET** /courses/{course_id}/blackout_dates/{id} | Get a single blackout date
[**blackout_dates_update_for_accounts**](BlackoutDatesApi.md#blackout_dates_update_for_accounts) | **PUT** /accounts/{account_id}/blackout_dates/{id} | Update Blackout Date
[**blackout_dates_update_for_courses**](BlackoutDatesApi.md#blackout_dates_update_for_courses) | **PUT** /courses/{course_id}/blackout_dates/{id} | Update Blackout Date



## blackout_dates_bulk_update

> String blackout_dates_bulk_update(course_id, blackout_dates_bulk_update_request)
Update a list of Blackout Dates

Create, update, and delete blackout dates to sync the db with the incoming data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**blackout_dates_bulk_update_request** | Option<[**BlackoutDatesBulkUpdateRequest**](BlackoutDatesBulkUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_create_for_accounts

> String blackout_dates_create_for_accounts(account_id, blackout_dates_create_for_courses_request)
Create Blackout Date

Create a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**blackout_dates_create_for_courses_request** | Option<[**BlackoutDatesCreateForCoursesRequest**](BlackoutDatesCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_create_for_courses

> String blackout_dates_create_for_courses(course_id, blackout_dates_create_for_courses_request)
Create Blackout Date

Create a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**blackout_dates_create_for_courses_request** | Option<[**BlackoutDatesCreateForCoursesRequest**](BlackoutDatesCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_destroy_for_accounts

> String blackout_dates_destroy_for_accounts(account_id, id)
Delete Blackout Date

Delete a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_destroy_for_courses

> String blackout_dates_destroy_for_courses(course_id, id)
Delete Blackout Date

Delete a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_index_for_accounts

> serde_json::Value blackout_dates_index_for_accounts(account_id)
List blackout dates

Returns the list of blackout dates for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_index_for_courses

> serde_json::Value blackout_dates_index_for_courses(course_id)
List blackout dates

Returns the list of blackout dates for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_new_for_accounts

> String blackout_dates_new_for_accounts(account_id)
New Blackout Date

Initialize an unsaved Blackout Date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_new_for_courses

> String blackout_dates_new_for_courses(course_id)
New Blackout Date

Initialize an unsaved Blackout Date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_show_for_accounts

> String blackout_dates_show_for_accounts(account_id, id)
Get a single blackout date

Returns the blackout date with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_show_for_courses

> String blackout_dates_show_for_courses(course_id, id)
Get a single blackout date

Returns the blackout date with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_update_for_accounts

> String blackout_dates_update_for_accounts(account_id, id, blackout_dates_create_for_courses_request)
Update Blackout Date

Update a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**blackout_dates_create_for_courses_request** | Option<[**BlackoutDatesCreateForCoursesRequest**](BlackoutDatesCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## blackout_dates_update_for_courses

> String blackout_dates_update_for_courses(course_id, id, blackout_dates_create_for_courses_request)
Update Blackout Date

Update a blackout date for the given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**blackout_dates_create_for_courses_request** | Option<[**BlackoutDatesCreateForCoursesRequest**](BlackoutDatesCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

