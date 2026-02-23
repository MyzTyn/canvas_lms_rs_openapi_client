# \ConferencesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conferences_for_user**](ConferencesApi.md#conferences_for_user) | **GET** /conferences | List conferences for the current user
[**conferences_index_for_courses**](ConferencesApi.md#conferences_index_for_courses) | **GET** /courses/{course_id}/conferences | List conferences
[**conferences_index_for_groups**](ConferencesApi.md#conferences_index_for_groups) | **GET** /groups/{group_id}/conferences | List conferences



## conferences_for_user

> models::Conference conferences_for_user(state, page, per_page)
List conferences for the current user

Retrieve the paginated list of conferences for all courses and groups the current user belongs to  This API returns a JSON object containing the list of conferences. The key for the list of conferences is \"conferences\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**state** | Option<**String**> | If set to \"live\", returns only conferences that are live (i.e., have started and not finished yet). If omitted, returns all conferences for this user's groups and courses. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Conference**](Conference.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conferences_index_for_courses

> serde_json::Value conferences_index_for_courses(course_id, page, per_page)
List conferences

Retrieve the paginated list of conferences for this context  This API returns a JSON object containing the list of conferences, the key for the list of conferences is \"conferences\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
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


## conferences_index_for_groups

> serde_json::Value conferences_index_for_groups(group_id, page, per_page)
List conferences

Retrieve the paginated list of conferences for this context  This API returns a JSON object containing the list of conferences, the key for the list of conferences is \"conferences\"

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
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

