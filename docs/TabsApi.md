# \TabsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tabs_index_for_accounts**](TabsApi.md#tabs_index_for_accounts) | **GET** /accounts/{account_id}/tabs | List available tabs for a course or group
[**tabs_index_for_courses**](TabsApi.md#tabs_index_for_courses) | **GET** /courses/{course_id}/tabs | List available tabs for a course or group
[**tabs_index_for_groups**](TabsApi.md#tabs_index_for_groups) | **GET** /groups/{group_id}/tabs | List available tabs for a course or group
[**tabs_index_for_users**](TabsApi.md#tabs_index_for_users) | **GET** /users/{user_id}/tabs | List available tabs for a course or group
[**tabs_update**](TabsApi.md#tabs_update) | **PUT** /courses/{course_id}/tabs/{tab_id} | Update a tab for a course



## tabs_index_for_accounts

> tabs_index_for_accounts(account_id, include)
List available tabs for a course or group

Returns a paginated list of navigation tabs available in the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"course_subject_tabs\"] - \"course_subject_tabs\": Optional flag to return the tabs associated with a canvas_for_elementary subject course's   home page instead of the typical sidebar navigation. Only takes effect if this request is for a course context   in a canvas_for_elementary-enabled account or sub-account. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tabs_index_for_courses

> tabs_index_for_courses(course_id, include)
List available tabs for a course or group

Returns a paginated list of navigation tabs available in the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"course_subject_tabs\"] - \"course_subject_tabs\": Optional flag to return the tabs associated with a canvas_for_elementary subject course's   home page instead of the typical sidebar navigation. Only takes effect if this request is for a course context   in a canvas_for_elementary-enabled account or sub-account. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tabs_index_for_groups

> tabs_index_for_groups(group_id, include)
List available tabs for a course or group

Returns a paginated list of navigation tabs available in the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"course_subject_tabs\"] - \"course_subject_tabs\": Optional flag to return the tabs associated with a canvas_for_elementary subject course's   home page instead of the typical sidebar navigation. Only takes effect if this request is for a course context   in a canvas_for_elementary-enabled account or sub-account. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tabs_index_for_users

> tabs_index_for_users(user_id, include)
List available tabs for a course or group

Returns a paginated list of navigation tabs available in the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"course_subject_tabs\"] - \"course_subject_tabs\": Optional flag to return the tabs associated with a canvas_for_elementary subject course's   home page instead of the typical sidebar navigation. Only takes effect if this request is for a course context   in a canvas_for_elementary-enabled account or sub-account. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tabs_update

> String tabs_update(course_id, tab_id, tabs_update_request)
Update a tab for a course

Home and Settings tabs are not manageable, and can't be hidden or moved  Returns a tab object

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**tab_id** | **String** | Scope response to tab_id | [required] |
**tabs_update_request** | Option<[**TabsUpdateRequest**](TabsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/tabs/tab_id \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'hidden=true' \\   -d 'position=2' // 1 based ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

