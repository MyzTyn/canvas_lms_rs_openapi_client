# \SearchApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**search_all_courses**](SearchApi.md#search_all_courses) | **GET** /search/all_courses | List all courses
[**search_recipients_other**](SearchApi.md#search_recipients_other) | **GET** /conversations/find_recipients | Find recipients
[**search_recipients_other2**](SearchApi.md#search_recipients_other2) | **GET** /search/recipients | Find recipients



## search_all_courses

> search_all_courses(search, public_only, open_enrollment_only, page, per_page)
List all courses

A paginated list of all courses visible in the public index

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search terms used for matching users/courses/groups (e.g. \"bob smith\"). If multiple terms are given (separated via whitespace), only results matching all terms will be returned. |  |
**public_only** | Option<**bool**> | Only return courses with public content. Defaults to false. |  |
**open_enrollment_only** | Option<**bool**> | Only return courses that allow self enrollment. Defaults to false. |  |
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


## search_recipients_other

> search_recipients_other(search, context, exclude, r#type, user_id, from_conversation_id, permissions, page, per_page)
Find recipients

Find valid recipients (users, courses and groups) that the current user can send messages to. The /api/v1/search/recipients path is the preferred endpoint, /api/v1/conversations/find_recipients is deprecated.  Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search terms used for matching users/courses/groups (e.g. \"bob smith\"). If multiple terms are given (separated via whitespace), only results matching all terms will be returned. |  |
**context** | Option<**String**> | Limit the search to a particular course/group (e.g. \"course_3\" or \"group_4\"). |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String] Array of ids to exclude from the search. These may be user ids or course/group ids prefixed with \"course_\" or \"group_\" respectively, e.g. exclude[]=1&exclude[]=2&exclude[]=course_3 |  |
**r#type** | Option<**String**> | Limit the search just to users or contexts (groups/courses). |  |
**user_id** | Option<**i32**> | Search for a specific user id. This ignores the other above parameters, and will never return more than one result. |  |
**from_conversation_id** | Option<**i32**> | When searching by user_id, only users that could be normally messaged by this user will be returned. This parameter allows you to specify a conversation that will be referenced for a shared context -- if both the current user and the searched user are in the conversation, the user will be returned. This is used to start new side conversations. |  |
**permissions** | Option<[**Vec<String>**](String.md)> | [String] Array of permission strings to be checked for each matched context (e.g. \"send_messages\"). This argument determines which permissions may be returned in the response; it won't prevent contexts from being returned if they don't grant the permission(s). |  |
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


## search_recipients_other2

> search_recipients_other2(search, context, exclude, r#type, user_id, from_conversation_id, permissions, page, per_page)
Find recipients

Find valid recipients (users, courses and groups) that the current user can send messages to. The /api/v1/search/recipients path is the preferred endpoint, /api/v1/conversations/find_recipients is deprecated.  Pagination is supported.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> | Search terms used for matching users/courses/groups (e.g. \"bob smith\"). If multiple terms are given (separated via whitespace), only results matching all terms will be returned. |  |
**context** | Option<**String**> | Limit the search to a particular course/group (e.g. \"course_3\" or \"group_4\"). |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String] Array of ids to exclude from the search. These may be user ids or course/group ids prefixed with \"course_\" or \"group_\" respectively, e.g. exclude[]=1&exclude[]=2&exclude[]=course_3 |  |
**r#type** | Option<**String**> | Limit the search just to users or contexts (groups/courses). |  |
**user_id** | Option<**i32**> | Search for a specific user id. This ignores the other above parameters, and will never return more than one result. |  |
**from_conversation_id** | Option<**i32**> | When searching by user_id, only users that could be normally messaged by this user will be returned. This parameter allows you to specify a conversation that will be referenced for a shared context -- if both the current user and the searched user are in the conversation, the user will be returned. This is used to start new side conversations. |  |
**permissions** | Option<[**Vec<String>**](String.md)> | [String] Array of permission strings to be checked for each matched context (e.g. \"send_messages\"). This argument determines which permissions may be returned in the response; it won't prevent contexts from being returned if they don't grant the permission(s). |  |
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

