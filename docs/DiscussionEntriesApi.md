# \DiscussionEntriesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**discussion_entries_destroy_for_courses**](DiscussionEntriesApi.md#discussion_entries_destroy_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id}/entries/{id} | Delete an entry
[**discussion_entries_destroy_for_groups**](DiscussionEntriesApi.md#discussion_entries_destroy_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id}/entries/{id} | Delete an entry
[**discussion_entries_update_for_courses**](DiscussionEntriesApi.md#discussion_entries_update_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/entries/{id} | Update an entry
[**discussion_entries_update_for_groups**](DiscussionEntriesApi.md#discussion_entries_update_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/entries/{id} | Update an entry



## discussion_entries_destroy_for_courses

> discussion_entries_destroy_for_courses(course_id, id, topic_id)
Delete an entry

Delete a discussion entry.  The entry must have been created by the current user, or the current user must have admin rights to the discussion. If the delete is not allowed, a 401 will be returned.  The discussion will be marked deleted, and the user_id and message will be cleared out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_entries_destroy_for_groups

> discussion_entries_destroy_for_groups(group_id, id, topic_id)
Delete an entry

Delete a discussion entry.  The entry must have been created by the current user, or the current user must have admin rights to the discussion. If the delete is not allowed, a 401 will be returned.  The discussion will be marked deleted, and the user_id and message will be cleared out.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_entries_update_for_courses

> discussion_entries_update_for_courses(course_id, id, topic_id, discussion_entries_update_for_courses_request)
Update an entry

Update an existing discussion entry.  The entry must have been created by the current user, or the current user must have admin rights to the discussion. If the edit is not allowed, a 401 will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_entries_update_for_courses_request** | Option<[**DiscussionEntriesUpdateForCoursesRequest**](DiscussionEntriesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_entries_update_for_groups

> discussion_entries_update_for_groups(group_id, id, topic_id, discussion_entries_update_for_courses_request)
Update an entry

Update an existing discussion entry.  The entry must have been created by the current user, or the current user must have admin rights to the discussion. If the edit is not allowed, a 401 will be returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_entries_update_for_courses_request** | Option<[**DiscussionEntriesUpdateForCoursesRequest**](DiscussionEntriesUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

