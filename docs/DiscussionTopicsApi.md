# \DiscussionTopicsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**discussion_topics_create_for_courses**](DiscussionTopicsApi.md#discussion_topics_create_for_courses) | **POST** /courses/{course_id}/discussion_topics | Create a new discussion topic
[**discussion_topics_create_for_groups**](DiscussionTopicsApi.md#discussion_topics_create_for_groups) | **POST** /groups/{group_id}/discussion_topics | Create a new discussion topic
[**discussion_topics_destroy_for_courses**](DiscussionTopicsApi.md#discussion_topics_destroy_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id} | Delete a topic
[**discussion_topics_destroy_for_groups**](DiscussionTopicsApi.md#discussion_topics_destroy_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id} | Delete a topic
[**discussion_topics_index_for_courses**](DiscussionTopicsApi.md#discussion_topics_index_for_courses) | **GET** /courses/{course_id}/discussion_topics | List discussion topics
[**discussion_topics_index_for_groups**](DiscussionTopicsApi.md#discussion_topics_index_for_groups) | **GET** /groups/{group_id}/discussion_topics | List discussion topics
[**discussion_topics_reorder_for_courses**](DiscussionTopicsApi.md#discussion_topics_reorder_for_courses) | **POST** /courses/{course_id}/discussion_topics/reorder | Reorder pinned topics
[**discussion_topics_reorder_for_groups**](DiscussionTopicsApi.md#discussion_topics_reorder_for_groups) | **POST** /groups/{group_id}/discussion_topics/reorder | Reorder pinned topics
[**discussion_topics_update_for_courses**](DiscussionTopicsApi.md#discussion_topics_update_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id} | Update a topic
[**discussion_topics_update_for_groups**](DiscussionTopicsApi.md#discussion_topics_update_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id} | Update a topic



## discussion_topics_create_for_courses

> discussion_topics_create_for_courses(course_id, discussion_topics_create_for_courses_request)
Create a new discussion topic

Create an new discussion topic for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**discussion_topics_create_for_courses_request** | Option<[**DiscussionTopicsCreateForCoursesRequest**](DiscussionTopicsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_create_for_groups

> discussion_topics_create_for_groups(group_id, discussion_topics_create_for_courses_request)
Create a new discussion topic

Create an new discussion topic for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**discussion_topics_create_for_courses_request** | Option<[**DiscussionTopicsCreateForCoursesRequest**](DiscussionTopicsCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_destroy_for_courses

> discussion_topics_destroy_for_courses(course_id, topic_id)
Delete a topic

Deletes the discussion topic. This will also delete the assignment, if it's an assignment discussion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_destroy_for_groups

> discussion_topics_destroy_for_groups(group_id, topic_id)
Delete a topic

Deletes the discussion topic. This will also delete the assignment, if it's an assignment discussion.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_index_for_courses

> serde_json::Value discussion_topics_index_for_courses(course_id, include, order_by, scope, only_announcements, filter_by, search_term, exclude_context_module_locked_topics, page, per_page)
List discussion topics

Returns the paginated list of discussion topics for this course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"all_dates\", \"sections\", \"sections_user_count\", \"overrides\"] If \"all_dates\" is passed, all dates associated with graded discussions' assignments will be included. if \"sections\" is passed, includes the course sections that are associated with the topic, if the topic is specific to certain sections of the course. If \"sections_user_count\" is passed, then:   (a) If sections were asked for *and* the topic is specific to certain       course sections, includes the number of users in each       section. (as part of the section json asked for above)   (b) Else, includes at the root level the total number of users in the       topic's context (group or course) that the topic applies to. If \"overrides\" is passed, the overrides for the assignment will be included |  |
**order_by** | Option<**String**> | Determines the order of the discussion topic list. Defaults to \"position\". |  |
**scope** | Option<**String**> | Only return discussion topics in the given state(s). Defaults to including all topics. Filtering is done after pagination, so pages may be smaller than requested if topics are filtered. Can pass multiple states as comma separated string. |  |
**only_announcements** | Option<**bool**> | Return announcements instead of discussion topics. Defaults to false |  |
**filter_by** | Option<**String**> | The state of the discussion topic to return. Currently only supports unread state. |  |
**search_term** | Option<**String**> | The partial title of the discussion topics to match and return. |  |
**exclude_context_module_locked_topics** | Option<**bool**> | For students, exclude topics that are locked by module progression. Defaults to false. |  |
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


## discussion_topics_index_for_groups

> serde_json::Value discussion_topics_index_for_groups(group_id, include, order_by, scope, only_announcements, filter_by, search_term, exclude_context_module_locked_topics, page, per_page)
List discussion topics

Returns the paginated list of discussion topics for this course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"all_dates\", \"sections\", \"sections_user_count\", \"overrides\"] If \"all_dates\" is passed, all dates associated with graded discussions' assignments will be included. if \"sections\" is passed, includes the course sections that are associated with the topic, if the topic is specific to certain sections of the course. If \"sections_user_count\" is passed, then:   (a) If sections were asked for *and* the topic is specific to certain       course sections, includes the number of users in each       section. (as part of the section json asked for above)   (b) Else, includes at the root level the total number of users in the       topic's context (group or course) that the topic applies to. If \"overrides\" is passed, the overrides for the assignment will be included |  |
**order_by** | Option<**String**> | Determines the order of the discussion topic list. Defaults to \"position\". |  |
**scope** | Option<**String**> | Only return discussion topics in the given state(s). Defaults to including all topics. Filtering is done after pagination, so pages may be smaller than requested if topics are filtered. Can pass multiple states as comma separated string. |  |
**only_announcements** | Option<**bool**> | Return announcements instead of discussion topics. Defaults to false |  |
**filter_by** | Option<**String**> | The state of the discussion topic to return. Currently only supports unread state. |  |
**search_term** | Option<**String**> | The partial title of the discussion topics to match and return. |  |
**exclude_context_module_locked_topics** | Option<**bool**> | For students, exclude topics that are locked by module progression. Defaults to false. |  |
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


## discussion_topics_reorder_for_courses

> discussion_topics_reorder_for_courses(course_id, discussion_topics_reorder_for_courses_request)
Reorder pinned topics

Puts the pinned discussion topics in the specified order. All pinned topics should be included.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**discussion_topics_reorder_for_courses_request** | Option<[**DiscussionTopicsReorderForCoursesRequest**](DiscussionTopicsReorderForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_reorder_for_groups

> discussion_topics_reorder_for_groups(group_id, discussion_topics_reorder_for_courses_request)
Reorder pinned topics

Puts the pinned discussion topics in the specified order. All pinned topics should be included.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**discussion_topics_reorder_for_courses_request** | Option<[**DiscussionTopicsReorderForCoursesRequest**](DiscussionTopicsReorderForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_update_for_courses

> discussion_topics_update_for_courses(course_id, topic_id, discussion_topics_update_for_courses_request)
Update a topic

Update an existing discussion topic for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_update_for_courses_request** | Option<[**DiscussionTopicsUpdateForCoursesRequest**](DiscussionTopicsUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_update_for_groups

> discussion_topics_update_for_groups(group_id, topic_id, discussion_topics_update_for_courses_request)
Update a topic

Update an existing discussion topic for the course or group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_update_for_courses_request** | Option<[**DiscussionTopicsUpdateForCoursesRequest**](DiscussionTopicsUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

