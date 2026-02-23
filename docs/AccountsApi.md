# \AccountsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_course_accounts**](AccountsApi.md#accounts_course_accounts) | **GET** /course_accounts | List accounts for course admins
[**accounts_course_creation_accounts**](AccountsApi.md#accounts_course_creation_accounts) | **GET** /course_creation_accounts | Get accounts that users can create courses in
[**accounts_courses_api**](AccountsApi.md#accounts_courses_api) | **GET** /accounts/{account_id}/courses | List active courses in an account
[**accounts_environment**](AccountsApi.md#accounts_environment) | **GET** /settings/environment | List environment settings
[**accounts_help_links**](AccountsApi.md#accounts_help_links) | **GET** /accounts/{account_id}/help_links | Get help links
[**accounts_index**](AccountsApi.md#accounts_index) | **GET** /accounts | List accounts
[**accounts_manageable_accounts**](AccountsApi.md#accounts_manageable_accounts) | **GET** /manageable_accounts | Get accounts that admins can manage
[**accounts_manually_created_courses_account**](AccountsApi.md#accounts_manually_created_courses_account) | **GET** /manually_created_courses_account | Get the manually-created courses sub-account for the domain root account
[**accounts_permissions**](AccountsApi.md#accounts_permissions) | **GET** /accounts/{account_id}/permissions | Permissions
[**accounts_remove_user**](AccountsApi.md#accounts_remove_user) | **DELETE** /accounts/{account_id}/users/{user_id} | Delete a user from the root account
[**accounts_remove_users**](AccountsApi.md#accounts_remove_users) | **DELETE** /accounts/{account_id}/users | Delete multiple users from the root account
[**accounts_restore_user**](AccountsApi.md#accounts_restore_user) | **PUT** /accounts/{account_id}/users/{user_id}/restore | Restore a deleted user from a root account
[**accounts_show**](AccountsApi.md#accounts_show) | **GET** /accounts/{id} | Get a single account
[**accounts_show_settings**](AccountsApi.md#accounts_show_settings) | **GET** /accounts/{account_id}/settings | Settings
[**accounts_sub_accounts**](AccountsApi.md#accounts_sub_accounts) | **GET** /accounts/{account_id}/sub_accounts | Get the sub-accounts of an account
[**accounts_terms_of_service**](AccountsApi.md#accounts_terms_of_service) | **GET** /accounts/{account_id}/terms_of_service | Get the Terms of Service
[**accounts_update**](AccountsApi.md#accounts_update) | **PUT** /accounts/{id} | Update an account
[**accounts_update_users**](AccountsApi.md#accounts_update_users) | **PUT** /accounts/{account_id}/users/bulk_update | Update multiple users



## accounts_course_accounts

> models::Account accounts_course_accounts(page, per_page)
List accounts for course admins

A paginated list of accounts that the current user can view through their admin course enrollments. (Teacher, TA, or designer enrollments). Only returns \"id\", \"name\", \"workflow_state\", \"root_account_id\" and \"parent_account_id\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_course_creation_accounts

> models::Account accounts_course_creation_accounts(page, per_page)
Get accounts that users can create courses in

A paginated list of accounts where the current user has permission to create courses.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_courses_api

> models::Course accounts_courses_api(account_id, with_enrollments, enrollment_type, enrollment_workflow_state, published, completed, blueprint, blueprint_associated, public, by_teachers, by_subaccounts, hide_enrollmentless_courses, state, enrollment_term_id, search_term, include, sort, order, search_by, starts_before, ends_after, homeroom, page, per_page)
List active courses in an account

Retrieve a paginated list of courses in this account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**with_enrollments** | Option<**bool**> | If true, include only courses with at least one enrollment.  If false, include only courses with no enrollments.  If not present, do not filter on course enrollment status. |  |
**enrollment_type** | Option<[**Vec<String>**](String.md)> | [String, \"teacher\"|\"student\"|\"ta\"|\"observer\"|\"designer\"] If set, only return courses that have at least one user enrolled in in the course with one of the specified enrollment types. |  |
**enrollment_workflow_state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"completed\"|\"deleted\"|\"invited\"|\"pending\"|\"creation_pending\"|\"rejected\"|\"inactive\"] If set, only return courses that have at least one user enrolled in in the course with one of the specified enrollment workflow states. |  |
**published** | Option<**bool**> | If true, include only published courses.  If false, exclude published courses.  If not present, do not filter on published status. |  |
**completed** | Option<**bool**> | If true, include only completed courses (these may be in state 'completed', or their enrollment term may have ended).  If false, exclude completed courses.  If not present, do not filter on completed status. |  |
**blueprint** | Option<**bool**> | If true, include only blueprint courses. If false, exclude them. If not present, do not filter on this basis. |  |
**blueprint_associated** | Option<**bool**> | If true, include only courses that inherit content from a blueprint course. If false, exclude them. If not present, do not filter on this basis. |  |
**public** | Option<**bool**> | If true, include only public courses. If false, exclude them. If not present, do not filter on this basis. |  |
**by_teachers** | Option<[**Vec<String>**](String.md)> | [Integer] List of User IDs of teachers; if supplied, include only courses taught by one of the referenced users. |  |
**by_subaccounts** | Option<[**Vec<String>**](String.md)> | [Integer] List of Account IDs; if supplied, include only courses associated with one of the referenced subaccounts. |  |
**hide_enrollmentless_courses** | Option<**bool**> | If present, only return courses that have at least one enrollment. Equivalent to 'with_enrollments=true'; retained for compatibility. |  |
**state** | Option<[**Vec<String>**](String.md)> | [\"created\"|\"claimed\"|\"available\"|\"completed\"|\"deleted\"|\"all\"] If set, only return courses that are in the given state(s). By default, all states but \"deleted\" are returned. |  |
**enrollment_term_id** | Option<**i32**> | If set, only includes courses from the specified term. |  |
**search_term** | Option<**String**> | The partial course name, code, or full ID to match and return in the results list. Must be at least 3 characters. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"syllabus_body\"|\"term\"|\"course_progress\"|\"storage_quota_used_mb\"|\"total_students\"|\"teachers\"|\"account_name\"|\"concluded\"|\"post_manually\"] - All explanations can be seen in the {api:CoursesController#index Course API index documentation} - \"sections\", \"needs_grading_count\" and \"total_scores\" are not valid options at the account level |  |
**sort** | Option<**String**> | The column to sort results by. |  |
**order** | Option<**String**> | The order to sort the given column by. |  |
**search_by** | Option<**String**> | The filter to search by. \"course\" searches for course names, course codes, and SIS IDs. \"teacher\" searches for teacher names |  |
**starts_before** | Option<**String**> | If set, only return courses that start before the value (inclusive) or their enrollment term starts before the value (inclusive) or both the course's start_at and the enrollment term's start_at are set to null. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**ends_after** | Option<**String**> | If set, only return courses that end after the value (inclusive) or their enrollment term ends after the value (inclusive) or both the course's end_at and the enrollment term's end_at are set to null. The value should be formatted as: yyyy-mm-dd or ISO 8601 YYYY-MM-DDTHH:MM:SSZ. |  |
**homeroom** | Option<**bool**> | If set, only return homeroom courses. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Course**](Course.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_environment

> accounts_environment()
List environment settings

Return a hash of global settings for the root account This is the same information supplied to the web interface as +ENV.SETTINGS+.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_help_links

> String accounts_help_links(account_id)
Get help links

Returns the help links for that account

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


## accounts_index

> models::Account accounts_index(include, page, per_page)
List accounts

A paginated list of accounts that the current user can view or manage. Typically, students and even teachers will get an empty list in response, only account admins can view the accounts that they are in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | [String, \"lti_guid\"|\"registration_settings\"|\"services\"|\"course_count\"|\"sub_account_count\"] Array of additional information to include.  \"lti_guid\":: the 'tool_consumer_instance_guid' that will be sent for this account on LTI launches \"registration_settings\":: returns info about the privacy policy and terms of use \"services\":: returns services and whether they are enabled (requires account management permissions) \"course_count\":: returns the number of courses directly under each account \"sub_account_count\":: returns the number of sub-accounts directly under each account |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_manageable_accounts

> models::Account accounts_manageable_accounts(page, per_page)
Get accounts that admins can manage

A paginated list of accounts where the current user has permission to create or manage courses. List will be empty for students and teachers as only admins can view which accounts they are in.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_manually_created_courses_account

> String accounts_manually_created_courses_account()
Get the manually-created courses sub-account for the domain root account

Returns the sub-account that contains manually created courses for the domain root account.

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_permissions

> accounts_permissions(account_id, permissions)
Permissions

Returns permission information for the calling user and the given account. You may use `self` as the account id to check permissions against the domain root account. The caller must have an account role or admin (teacher/TA/designer) enrollment in a course in the account.  See also the {api:CoursesController#permissions Course} and {api:GroupsController#permissions Group} counterparts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**permissions** | Option<[**Vec<String>**](String.md)> | [String] List of permissions to check against the authenticated user. Permission names are documented in the {api:RoleOverridesController#manageable_permissions List assignable permissions} endpoint. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_remove_user

> String accounts_remove_user(account_id, user_id)
Delete a user from the root account

Delete a user record from a Canvas root account. If a user is associated with multiple root accounts (in a multi-tenant instance of Canvas), this action will NOT remove them from the other accounts.  WARNING: This API will allow a user to remove themselves from the account. If they do this, they won't be able to make API calls or log into Canvas at that account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_remove_users

> String accounts_remove_users(account_id)
Delete multiple users from the root account

Delete multiple users from a Canvas root account. If a user is associated with multiple root accounts (in a multi-tenant instance of Canvas), this action will NOT remove them from the other accounts.  WARNING: This API will allow a user to remove themselves from the account. If they do this, they won't be able to make API calls or log into Canvas at that account.

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


## accounts_restore_user

> String accounts_restore_user(account_id, user_id)
Restore a deleted user from a root account

Restore a user record along with the most recently deleted pseudonym from a Canvas root account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_show

> String accounts_show(id)
Get a single account

Retrieve information on an individual account, given by id or sis sis_account_id.

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


## accounts_show_settings

> accounts_show_settings(account_id)
Settings

Returns a JSON object containing a subset of settings for the specified account. It's possible an empty set will be returned if no settings are applicable. The caller must be an Account admin with the manage_account_settings permission.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_sub_accounts

> models::Account accounts_sub_accounts(account_id, recursive, order, include, page, per_page)
Get the sub-accounts of an account

List accounts that are sub-accounts of the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**recursive** | Option<**bool**> | If true, the entire account tree underneath this account will be returned (though still paginated). If false, only direct sub-accounts of this account will be returned. Defaults to false. |  |
**order** | Option<**String**> | Sorts the accounts by id or name. Only applies when recursive is false. Defaults to id. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"course_count\"|\"sub_account_count\"] Array of additional information to include.  \"course_count\":: returns the number of courses directly under each account \"sub_account_count\":: returns the number of sub-accounts directly under each account |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Account**](Account.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_terms_of_service

> String accounts_terms_of_service(account_id)
Get the Terms of Service

Returns the terms of service for that account

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


## accounts_update

> String accounts_update(id, accounts_update_request)
Update an account

Update an existing account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**accounts_update_request** | Option<[**AccountsUpdateRequest**](AccountsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/<account_id> \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'account[name]=New account name' \\   -d 'account[default_time_zone]=Mountain Time (US & Canada)' \\   -d 'account[default_storage_quota_mb]=450' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_update_users

> String accounts_update_users(account_id, accounts_update_users_request)
Update multiple users

Updates multiple users in bulk.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**accounts_update_users_request** | Option<[**AccountsUpdateUsersRequest**](AccountsUpdateUsersRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/3/users/bulk_update \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'user_ids[]=1' \\   -d 'user_ids[]=2' \\   -d 'user[event]=suspend' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

