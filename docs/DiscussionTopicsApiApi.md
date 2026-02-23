# \DiscussionTopicsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**discussion_topics_api_add_entry_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_add_entry_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/entries | Post an entry
[**discussion_topics_api_add_entry_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_add_entry_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/entries | Post an entry
[**discussion_topics_api_add_reply_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_add_reply_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/entries/{entry_id}/replies | Post a reply
[**discussion_topics_api_add_reply_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_add_reply_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/entries/{entry_id}/replies | Post a reply
[**discussion_topics_api_disable_summary_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_disable_summary_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/summaries/disable | Disable summary
[**discussion_topics_api_disable_summary_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_disable_summary_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/summaries/disable | Disable summary
[**discussion_topics_api_duplicate_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_duplicate_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/duplicate | Duplicate discussion topic
[**discussion_topics_api_duplicate_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_duplicate_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/duplicate | Duplicate discussion topic
[**discussion_topics_api_entries_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_entries_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id}/entries | List topic entries
[**discussion_topics_api_entries_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_entries_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id}/entries | List topic entries
[**discussion_topics_api_entry_list_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_entry_list_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id}/entry_list | List entries
[**discussion_topics_api_entry_list_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_entry_list_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id}/entry_list | List entries
[**discussion_topics_api_find_or_create_summary_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_find_or_create_summary_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/summaries | Find or Create Summary
[**discussion_topics_api_find_or_create_summary_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_find_or_create_summary_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/summaries | Find or Create Summary
[**discussion_topics_api_find_summary_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_find_summary_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id}/summaries | Find Last Summary
[**discussion_topics_api_find_summary_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_find_summary_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id}/summaries | Find Last Summary
[**discussion_topics_api_mark_all_read_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_read_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/read_all | Mark all entries as read
[**discussion_topics_api_mark_all_read_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_read_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/read_all | Mark all entries as read
[**discussion_topics_api_mark_all_topic_read_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_topic_read_for_courses) | **PUT** /courses/{course_id}/discussion_topics/read_all | Mark all topic as read
[**discussion_topics_api_mark_all_topic_read_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_topic_read_for_groups) | **PUT** /groups/{group_id}/discussion_topics/read_all | Mark all topic as read
[**discussion_topics_api_mark_all_unread_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_unread_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id}/read_all | Mark all entries as unread
[**discussion_topics_api_mark_all_unread_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_all_unread_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id}/read_all | Mark all entries as unread
[**discussion_topics_api_mark_entry_read_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_entry_read_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/entries/{entry_id}/read | Mark entry as read
[**discussion_topics_api_mark_entry_read_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_entry_read_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/entries/{entry_id}/read | Mark entry as read
[**discussion_topics_api_mark_entry_unread_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_entry_unread_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id}/entries/{entry_id}/read | Mark entry as unread
[**discussion_topics_api_mark_entry_unread_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_entry_unread_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id}/entries/{entry_id}/read | Mark entry as unread
[**discussion_topics_api_mark_topic_read_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_topic_read_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/read | Mark topic as read
[**discussion_topics_api_mark_topic_read_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_topic_read_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/read | Mark topic as read
[**discussion_topics_api_mark_topic_unread_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_topic_unread_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id}/read | Mark topic as unread
[**discussion_topics_api_mark_topic_unread_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_mark_topic_unread_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id}/read | Mark topic as unread
[**discussion_topics_api_rate_entry_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_rate_entry_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/entries/{entry_id}/rating | Rate entry
[**discussion_topics_api_rate_entry_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_rate_entry_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/entries/{entry_id}/rating | Rate entry
[**discussion_topics_api_replies_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_replies_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id}/entries/{entry_id}/replies | List entry replies
[**discussion_topics_api_replies_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_replies_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id}/entries/{entry_id}/replies | List entry replies
[**discussion_topics_api_show_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_show_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id} | Get a single topic
[**discussion_topics_api_show_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_show_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id} | Get a single topic
[**discussion_topics_api_subscribe_topic_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_subscribe_topic_for_courses) | **PUT** /courses/{course_id}/discussion_topics/{topic_id}/subscribed | Subscribe to a topic
[**discussion_topics_api_subscribe_topic_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_subscribe_topic_for_groups) | **PUT** /groups/{group_id}/discussion_topics/{topic_id}/subscribed | Subscribe to a topic
[**discussion_topics_api_summary_feedback_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_summary_feedback_for_courses) | **POST** /courses/{course_id}/discussion_topics/{topic_id}/summaries/{summary_id}/feedback | Summary Feedback
[**discussion_topics_api_summary_feedback_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_summary_feedback_for_groups) | **POST** /groups/{group_id}/discussion_topics/{topic_id}/summaries/{summary_id}/feedback | Summary Feedback
[**discussion_topics_api_unsubscribe_topic_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_unsubscribe_topic_for_courses) | **DELETE** /courses/{course_id}/discussion_topics/{topic_id}/subscribed | Unsubscribe from a topic
[**discussion_topics_api_unsubscribe_topic_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_unsubscribe_topic_for_groups) | **DELETE** /groups/{group_id}/discussion_topics/{topic_id}/subscribed | Unsubscribe from a topic
[**discussion_topics_api_view_for_courses**](DiscussionTopicsApiApi.md#discussion_topics_api_view_for_courses) | **GET** /courses/{course_id}/discussion_topics/{topic_id}/view | Get the full topic
[**discussion_topics_api_view_for_groups**](DiscussionTopicsApiApi.md#discussion_topics_api_view_for_groups) | **GET** /groups/{group_id}/discussion_topics/{topic_id}/view | Get the full topic



## discussion_topics_api_add_entry_for_courses

> discussion_topics_api_add_entry_for_courses(course_id, topic_id, discussion_topics_api_add_entry_for_courses_request)
Post an entry

Create a new entry in a discussion topic. Returns a json representation of the created entry (see documentation for 'entries' method) on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_add_entry_for_courses_request** | Option<[**DiscussionTopicsApiAddEntryForCoursesRequest**](DiscussionTopicsApiAddEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_add_entry_for_groups

> discussion_topics_api_add_entry_for_groups(group_id, topic_id, discussion_topics_api_add_entry_for_courses_request)
Post an entry

Create a new entry in a discussion topic. Returns a json representation of the created entry (see documentation for 'entries' method) on success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_add_entry_for_courses_request** | Option<[**DiscussionTopicsApiAddEntryForCoursesRequest**](DiscussionTopicsApiAddEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_add_reply_for_courses

> discussion_topics_api_add_reply_for_courses(course_id, entry_id, topic_id, discussion_topics_api_add_entry_for_courses_request)
Post a reply

Add a reply to an entry in a discussion topic. Returns a json representation of the created reply (see documentation for 'replies' method) on success.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_add_entry_for_courses_request** | Option<[**DiscussionTopicsApiAddEntryForCoursesRequest**](DiscussionTopicsApiAddEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_add_reply_for_groups

> discussion_topics_api_add_reply_for_groups(group_id, entry_id, topic_id, discussion_topics_api_add_entry_for_courses_request)
Post a reply

Add a reply to an entry in a discussion topic. Returns a json representation of the created reply (see documentation for 'replies' method) on success.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_add_entry_for_courses_request** | Option<[**DiscussionTopicsApiAddEntryForCoursesRequest**](DiscussionTopicsApiAddEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_disable_summary_for_courses

> discussion_topics_api_disable_summary_for_courses(course_id, topic_id)
Disable summary

Deprecated, to remove after VICE-5047 gets merged Disables the summary for a discussion topic.

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


## discussion_topics_api_disable_summary_for_groups

> discussion_topics_api_disable_summary_for_groups(group_id, topic_id)
Disable summary

Deprecated, to remove after VICE-5047 gets merged Disables the summary for a discussion topic.

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


## discussion_topics_api_duplicate_for_courses

> String discussion_topics_api_duplicate_for_courses(course_id, topic_id)
Duplicate discussion topic

Duplicate a discussion topic according to context (Course/Group)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_duplicate_for_groups

> String discussion_topics_api_duplicate_for_groups(group_id, topic_id)
Duplicate discussion topic

Duplicate a discussion topic according to context (Course/Group)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_entries_for_courses

> discussion_topics_api_entries_for_courses(course_id, topic_id, page, per_page)
List topic entries

Retrieve the (paginated) top-level entries in a discussion topic.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  Will include the 10 most recent replies, if any, for each entry returned.  If the topic is a root topic with children corresponding to groups of a group assignment, entries from those subtopics for which the user belongs to the corresponding group will be returned.  Ordering of returned entries is newest-first by posting timestamp (reply activity is ignored).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
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


## discussion_topics_api_entries_for_groups

> discussion_topics_api_entries_for_groups(group_id, topic_id, page, per_page)
List topic entries

Retrieve the (paginated) top-level entries in a discussion topic.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  Will include the 10 most recent replies, if any, for each entry returned.  If the topic is a root topic with children corresponding to groups of a group assignment, entries from those subtopics for which the user belongs to the corresponding group will be returned.  Ordering of returned entries is newest-first by posting timestamp (reply activity is ignored).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
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


## discussion_topics_api_entry_list_for_courses

> discussion_topics_api_entry_list_for_courses(course_id, topic_id, ids, page, per_page)
List entries

Retrieve a paginated list of discussion entries, given a list of ids.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**ids** | Option<[**Vec<String>**](String.md)> | [String] A list of entry ids to retrieve. Entries will be returned in id order, smallest id first. |  |
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


## discussion_topics_api_entry_list_for_groups

> discussion_topics_api_entry_list_for_groups(group_id, topic_id, ids, page, per_page)
List entries

Retrieve a paginated list of discussion entries, given a list of ids.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**ids** | Option<[**Vec<String>**](String.md)> | [String] A list of entry ids to retrieve. Entries will be returned in id order, smallest id first. |  |
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


## discussion_topics_api_find_or_create_summary_for_courses

> discussion_topics_api_find_or_create_summary_for_courses(course_id, topic_id, discussion_topics_api_find_or_create_summary_for_courses_request)
Find or Create Summary

Generates a summary for a discussion topic. Returns the summary text and usage information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_find_or_create_summary_for_courses_request** | Option<[**DiscussionTopicsApiFindOrCreateSummaryForCoursesRequest**](DiscussionTopicsApiFindOrCreateSummaryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_find_or_create_summary_for_groups

> discussion_topics_api_find_or_create_summary_for_groups(group_id, topic_id, discussion_topics_api_find_or_create_summary_for_courses_request)
Find or Create Summary

Generates a summary for a discussion topic. Returns the summary text and usage information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_find_or_create_summary_for_courses_request** | Option<[**DiscussionTopicsApiFindOrCreateSummaryForCoursesRequest**](DiscussionTopicsApiFindOrCreateSummaryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_find_summary_for_courses

> discussion_topics_api_find_summary_for_courses(course_id, topic_id)
Find Last Summary

Returns: (1) last userInput (what current user had keyed in to produce the last discussion summary), (2) last discussion summary generated by the current user for current discussion topic, based on userInput, (3) and some usage information.

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


## discussion_topics_api_find_summary_for_groups

> discussion_topics_api_find_summary_for_groups(group_id, topic_id)
Find Last Summary

Returns: (1) last userInput (what current user had keyed in to produce the last discussion summary), (2) last discussion summary generated by the current user for current discussion topic, based on userInput, (3) and some usage information.

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


## discussion_topics_api_mark_all_read_for_courses

> discussion_topics_api_mark_all_read_for_courses(course_id, topic_id, discussion_topics_api_mark_all_read_for_courses_request)
Mark all entries as read

Mark the discussion topic and all its entries as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_mark_all_read_for_courses_request** | Option<[**DiscussionTopicsApiMarkAllReadForCoursesRequest**](DiscussionTopicsApiMarkAllReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_all_read_for_groups

> discussion_topics_api_mark_all_read_for_groups(group_id, topic_id, discussion_topics_api_mark_all_read_for_courses_request)
Mark all entries as read

Mark the discussion topic and all its entries as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_mark_all_read_for_courses_request** | Option<[**DiscussionTopicsApiMarkAllReadForCoursesRequest**](DiscussionTopicsApiMarkAllReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_all_topic_read_for_courses

> discussion_topics_api_mark_all_topic_read_for_courses(course_id)
Mark all topic as read

Mark the initial text of all the discussion topics as read in  the context.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_all_topic_read_for_groups

> discussion_topics_api_mark_all_topic_read_for_groups(group_id)
Mark all topic as read

Mark the initial text of all the discussion topics as read in  the context.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_all_unread_for_courses

> discussion_topics_api_mark_all_unread_for_courses(course_id, topic_id, forced_read_state)
Mark all entries as unread

Mark the discussion topic and all its entries as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**forced_read_state** | Option<**bool**> | A boolean value to set all of the entries' forced_read_state. No change is made if this argument is not specified. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_all_unread_for_groups

> discussion_topics_api_mark_all_unread_for_groups(group_id, topic_id, forced_read_state)
Mark all entries as unread

Mark the discussion topic and all its entries as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**forced_read_state** | Option<**bool**> | A boolean value to set all of the entries' forced_read_state. No change is made if this argument is not specified. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_entry_read_for_courses

> discussion_topics_api_mark_entry_read_for_courses(course_id, entry_id, topic_id, discussion_topics_api_mark_entry_read_for_courses_request)
Mark entry as read

Mark a discussion entry as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_mark_entry_read_for_courses_request** | Option<[**DiscussionTopicsApiMarkEntryReadForCoursesRequest**](DiscussionTopicsApiMarkEntryReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_entry_read_for_groups

> discussion_topics_api_mark_entry_read_for_groups(group_id, entry_id, topic_id, discussion_topics_api_mark_entry_read_for_courses_request)
Mark entry as read

Mark a discussion entry as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_mark_entry_read_for_courses_request** | Option<[**DiscussionTopicsApiMarkEntryReadForCoursesRequest**](DiscussionTopicsApiMarkEntryReadForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_entry_unread_for_courses

> discussion_topics_api_mark_entry_unread_for_courses(course_id, entry_id, topic_id, forced_read_state)
Mark entry as unread

Mark a discussion entry as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**forced_read_state** | Option<**bool**> | A boolean value to set the entry's forced_read_state. No change is made if this argument is not specified. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_entry_unread_for_groups

> discussion_topics_api_mark_entry_unread_for_groups(group_id, entry_id, topic_id, forced_read_state)
Mark entry as unread

Mark a discussion entry as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**forced_read_state** | Option<**bool**> | A boolean value to set the entry's forced_read_state. No change is made if this argument is not specified. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_mark_topic_read_for_courses

> discussion_topics_api_mark_topic_read_for_courses(course_id, topic_id)
Mark topic as read

Mark the initial text of the discussion topic as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

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


## discussion_topics_api_mark_topic_read_for_groups

> discussion_topics_api_mark_topic_read_for_groups(group_id, topic_id)
Mark topic as read

Mark the initial text of the discussion topic as read.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

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


## discussion_topics_api_mark_topic_unread_for_courses

> discussion_topics_api_mark_topic_unread_for_courses(course_id, topic_id)
Mark topic as unread

Mark the initial text of the discussion topic as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

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


## discussion_topics_api_mark_topic_unread_for_groups

> discussion_topics_api_mark_topic_unread_for_groups(group_id, topic_id)
Mark topic as unread

Mark the initial text of the discussion topic as unread.  No request fields are necessary.  On success, the response will be 204 No Content with an empty body.

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


## discussion_topics_api_rate_entry_for_courses

> discussion_topics_api_rate_entry_for_courses(course_id, entry_id, topic_id, discussion_topics_api_rate_entry_for_courses_request)
Rate entry

Rate a discussion entry.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_rate_entry_for_courses_request** | Option<[**DiscussionTopicsApiRateEntryForCoursesRequest**](DiscussionTopicsApiRateEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_rate_entry_for_groups

> discussion_topics_api_rate_entry_for_groups(group_id, entry_id, topic_id, discussion_topics_api_rate_entry_for_courses_request)
Rate entry

Rate a discussion entry.  On success, the response will be 204 No Content with an empty body.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_rate_entry_for_courses_request** | Option<[**DiscussionTopicsApiRateEntryForCoursesRequest**](DiscussionTopicsApiRateEntryForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_replies_for_courses

> discussion_topics_api_replies_for_courses(course_id, entry_id, topic_id, page, per_page)
List entry replies

Retrieve the (paginated) replies to a top-level entry in a discussion topic.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  Ordering of returned entries is newest-first by creation timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
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


## discussion_topics_api_replies_for_groups

> discussion_topics_api_replies_for_groups(group_id, entry_id, topic_id, page, per_page)
List entry replies

Retrieve the (paginated) replies to a top-level entry in a discussion topic.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  Ordering of returned entries is newest-first by creation timestamp.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**entry_id** | **String** | Scope response to entry_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
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


## discussion_topics_api_show_for_courses

> discussion_topics_api_show_for_courses(course_id, topic_id, include)
Get a single topic

Returns data on an individual discussion topic. See the List action for the response formatting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"all_dates\", \"sections\", \"sections_user_count\", \"overrides\"] If \"all_dates\" is passed, all dates associated with graded discussions' assignments will be included. if \"sections\" is passed, includes the course sections that are associated with the topic, if the topic is specific to certain sections of the course. If \"sections_user_count\" is passed, then:   (a) If sections were asked for *and* the topic is specific to certain       course sections, includes the number of users in each       section. (as part of the section json asked for above)   (b) Else, includes at the root level the total number of users in the       topic's context (group or course) that the topic applies to. If \"overrides\" is passed, the overrides for the assignment will be included |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_show_for_groups

> discussion_topics_api_show_for_groups(group_id, topic_id, include)
Get a single topic

Returns data on an individual discussion topic. See the List action for the response formatting.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"all_dates\", \"sections\", \"sections_user_count\", \"overrides\"] If \"all_dates\" is passed, all dates associated with graded discussions' assignments will be included. if \"sections\" is passed, includes the course sections that are associated with the topic, if the topic is specific to certain sections of the course. If \"sections_user_count\" is passed, then:   (a) If sections were asked for *and* the topic is specific to certain       course sections, includes the number of users in each       section. (as part of the section json asked for above)   (b) Else, includes at the root level the total number of users in the       topic's context (group or course) that the topic applies to. If \"overrides\" is passed, the overrides for the assignment will be included |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_subscribe_topic_for_courses

> discussion_topics_api_subscribe_topic_for_courses(course_id, topic_id)
Subscribe to a topic

Subscribe to a topic to receive notifications about new entries  On success, the response will be 204 No Content with an empty body

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


## discussion_topics_api_subscribe_topic_for_groups

> discussion_topics_api_subscribe_topic_for_groups(group_id, topic_id)
Subscribe to a topic

Subscribe to a topic to receive notifications about new entries  On success, the response will be 204 No Content with an empty body

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


## discussion_topics_api_summary_feedback_for_courses

> discussion_topics_api_summary_feedback_for_courses(course_id, summary_id, topic_id, discussion_topics_api_summary_feedback_for_courses_request)
Summary Feedback

Persists feedback on a discussion topic summary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**summary_id** | **String** | Scope response to summary_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_summary_feedback_for_courses_request** | Option<[**DiscussionTopicsApiSummaryFeedbackForCoursesRequest**](DiscussionTopicsApiSummaryFeedbackForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_summary_feedback_for_groups

> discussion_topics_api_summary_feedback_for_groups(group_id, summary_id, topic_id, discussion_topics_api_summary_feedback_for_courses_request)
Summary Feedback

Persists feedback on a discussion topic summary.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**summary_id** | **String** | Scope response to summary_id | [required] |
**topic_id** | **String** | Scope response to topic_id | [required] |
**discussion_topics_api_summary_feedback_for_courses_request** | Option<[**DiscussionTopicsApiSummaryFeedbackForCoursesRequest**](DiscussionTopicsApiSummaryFeedbackForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## discussion_topics_api_unsubscribe_topic_for_courses

> discussion_topics_api_unsubscribe_topic_for_courses(course_id, topic_id)
Unsubscribe from a topic

Unsubscribe from a topic to stop receiving notifications about new entries  On success, the response will be 204 No Content with an empty body

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


## discussion_topics_api_unsubscribe_topic_for_groups

> discussion_topics_api_unsubscribe_topic_for_groups(group_id, topic_id)
Unsubscribe from a topic

Unsubscribe from a topic to stop receiving notifications about new entries  On success, the response will be 204 No Content with an empty body

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


## discussion_topics_api_view_for_courses

> discussion_topics_api_view_for_courses(course_id, topic_id)
Get the full topic

Return a cached structure of the discussion topic, containing all entries, their authors, and their message bodies.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  In some rare situations, this cached structure may not be available yet. In that case, the server will respond with a 503 error, and the caller should try again soon.  The response is an object containing the following keys: * \"participants\": A list of summary information on users who have posted to   the discussion. Each value is an object containing their id, display_name,   and avatar_url. * \"unread_entries\": A list of entry ids that are unread by the current   user. this implies that any entry not in this list is read. * \"entry_ratings\": A map of entry ids to ratings by the current user. Entries   not in this list have no rating. Only populated if rating is enabled. * \"forced_entries\": A list of entry ids that have forced_read_state set to   true. This flag is meant to indicate the entry's read_state has been   manually set to 'unread' by the user, so the entry should not be   automatically marked as read. * \"view\": A threaded view of all the entries in the discussion, containing   the id, user_id, and message. * \"new_entries\": Because this view is eventually consistent, it's possible   that newly created or updated entries won't yet be reflected in the view.   If the application wants to also get a flat list of all entries not yet   reflected in the view, pass include_new_entries=1 to the request and this   array of entries will be returned. These entries are returned in a flat   array, in ascending created_at order.

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


## discussion_topics_api_view_for_groups

> discussion_topics_api_view_for_groups(group_id, topic_id)
Get the full topic

Return a cached structure of the discussion topic, containing all entries, their authors, and their message bodies.  May require (depending on the topic) that the user has posted in the topic. If it is required, and the user has not posted, will respond with a 403 Forbidden status and the body 'require_initial_post'.  In some rare situations, this cached structure may not be available yet. In that case, the server will respond with a 503 error, and the caller should try again soon.  The response is an object containing the following keys: * \"participants\": A list of summary information on users who have posted to   the discussion. Each value is an object containing their id, display_name,   and avatar_url. * \"unread_entries\": A list of entry ids that are unread by the current   user. this implies that any entry not in this list is read. * \"entry_ratings\": A map of entry ids to ratings by the current user. Entries   not in this list have no rating. Only populated if rating is enabled. * \"forced_entries\": A list of entry ids that have forced_read_state set to   true. This flag is meant to indicate the entry's read_state has been   manually set to 'unread' by the user, so the entry should not be   automatically marked as read. * \"view\": A threaded view of all the entries in the discussion, containing   the id, user_id, and message. * \"new_entries\": Because this view is eventually consistent, it's possible   that newly created or updated entries won't yet be reflected in the view.   If the application wants to also get a flat list of all entries not yet   reflected in the view, pass include_new_entries=1 to the request and this   array of entries will be returned. These entries are returned in a flat   array, in ascending created_at order.

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

