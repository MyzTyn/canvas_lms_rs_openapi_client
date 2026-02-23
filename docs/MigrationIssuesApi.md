# \MigrationIssuesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**migration_issues_index_for_accounts**](MigrationIssuesApi.md#migration_issues_index_for_accounts) | **GET** /accounts/{account_id}/content_migrations/{content_migration_id}/migration_issues | List migration issues
[**migration_issues_index_for_courses**](MigrationIssuesApi.md#migration_issues_index_for_courses) | **GET** /courses/{course_id}/content_migrations/{content_migration_id}/migration_issues | List migration issues
[**migration_issues_index_for_groups**](MigrationIssuesApi.md#migration_issues_index_for_groups) | **GET** /groups/{group_id}/content_migrations/{content_migration_id}/migration_issues | List migration issues
[**migration_issues_index_for_users**](MigrationIssuesApi.md#migration_issues_index_for_users) | **GET** /users/{user_id}/content_migrations/{content_migration_id}/migration_issues | List migration issues
[**migration_issues_show_for_accounts**](MigrationIssuesApi.md#migration_issues_show_for_accounts) | **GET** /accounts/{account_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Get a migration issue
[**migration_issues_show_for_courses**](MigrationIssuesApi.md#migration_issues_show_for_courses) | **GET** /courses/{course_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Get a migration issue
[**migration_issues_show_for_groups**](MigrationIssuesApi.md#migration_issues_show_for_groups) | **GET** /groups/{group_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Get a migration issue
[**migration_issues_show_for_users**](MigrationIssuesApi.md#migration_issues_show_for_users) | **GET** /users/{user_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Get a migration issue
[**migration_issues_update_for_accounts**](MigrationIssuesApi.md#migration_issues_update_for_accounts) | **PUT** /accounts/{account_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Update a migration issue
[**migration_issues_update_for_courses**](MigrationIssuesApi.md#migration_issues_update_for_courses) | **PUT** /courses/{course_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Update a migration issue
[**migration_issues_update_for_groups**](MigrationIssuesApi.md#migration_issues_update_for_groups) | **PUT** /groups/{group_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Update a migration issue
[**migration_issues_update_for_users**](MigrationIssuesApi.md#migration_issues_update_for_users) | **PUT** /users/{user_id}/content_migrations/{content_migration_id}/migration_issues/{id} | Update a migration issue



## migration_issues_index_for_accounts

> serde_json::Value migration_issues_index_for_accounts(account_id, content_migration_id, page, per_page)
List migration issues

Returns paginated migration issues

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
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


## migration_issues_index_for_courses

> serde_json::Value migration_issues_index_for_courses(course_id, content_migration_id, page, per_page)
List migration issues

Returns paginated migration issues

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
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


## migration_issues_index_for_groups

> serde_json::Value migration_issues_index_for_groups(group_id, content_migration_id, page, per_page)
List migration issues

Returns paginated migration issues

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
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


## migration_issues_index_for_users

> serde_json::Value migration_issues_index_for_users(user_id, content_migration_id, page, per_page)
List migration issues

Returns paginated migration issues

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
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


## migration_issues_show_for_accounts

> String migration_issues_show_for_accounts(account_id, content_migration_id, id)
Get a migration issue

Returns data on an individual migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_show_for_courses

> String migration_issues_show_for_courses(course_id, content_migration_id, id)
Get a migration issue

Returns data on an individual migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_show_for_groups

> String migration_issues_show_for_groups(group_id, content_migration_id, id)
Get a migration issue

Returns data on an individual migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_show_for_users

> String migration_issues_show_for_users(user_id, content_migration_id, id)
Get a migration issue

Returns data on an individual migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_update_for_accounts

> String migration_issues_update_for_accounts(account_id, content_migration_id, id, migration_issues_update_for_accounts_request)
Update a migration issue

Update the workflow_state of a migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |
**migration_issues_update_for_accounts_request** | Option<[**MigrationIssuesUpdateForAccountsRequest**](MigrationIssuesUpdateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_update_for_courses

> String migration_issues_update_for_courses(course_id, content_migration_id, id, migration_issues_update_for_accounts_request)
Update a migration issue

Update the workflow_state of a migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |
**migration_issues_update_for_accounts_request** | Option<[**MigrationIssuesUpdateForAccountsRequest**](MigrationIssuesUpdateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_update_for_groups

> String migration_issues_update_for_groups(group_id, content_migration_id, id, migration_issues_update_for_accounts_request)
Update a migration issue

Update the workflow_state of a migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |
**migration_issues_update_for_accounts_request** | Option<[**MigrationIssuesUpdateForAccountsRequest**](MigrationIssuesUpdateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## migration_issues_update_for_users

> String migration_issues_update_for_users(user_id, content_migration_id, id, migration_issues_update_for_accounts_request)
Update a migration issue

Update the workflow_state of a migration issue

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_migration_id** | **String** | Scope response to content_migration_id | [required] |
**id** | **String** | Scope response to id | [required] |
**migration_issues_update_for_accounts_request** | Option<[**MigrationIssuesUpdateForAccountsRequest**](MigrationIssuesUpdateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

