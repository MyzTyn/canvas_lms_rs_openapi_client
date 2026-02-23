# \ConversationsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**conversations_add_message**](ConversationsApi.md#conversations_add_message) | **POST** /conversations/{id}/add_message | Add a message
[**conversations_add_recipients**](ConversationsApi.md#conversations_add_recipients) | **POST** /conversations/{id}/add_recipients | Add recipients
[**conversations_batch_update**](ConversationsApi.md#conversations_batch_update) | **PUT** /conversations | Batch update conversations
[**conversations_batches**](ConversationsApi.md#conversations_batches) | **GET** /conversations/batches | Get running batches
[**conversations_create**](ConversationsApi.md#conversations_create) | **POST** /conversations | Create a conversation
[**conversations_destroy**](ConversationsApi.md#conversations_destroy) | **DELETE** /conversations/{id} | Delete a conversation
[**conversations_index**](ConversationsApi.md#conversations_index) | **GET** /conversations | List conversations
[**conversations_mark_all_as_read**](ConversationsApi.md#conversations_mark_all_as_read) | **POST** /conversations/mark_all_as_read | Mark all as read
[**conversations_remove_messages**](ConversationsApi.md#conversations_remove_messages) | **POST** /conversations/{id}/remove_messages | Delete a message
[**conversations_show**](ConversationsApi.md#conversations_show) | **GET** /conversations/{id} | Get a single conversation
[**conversations_unread_count**](ConversationsApi.md#conversations_unread_count) | **GET** /conversations/unread_count | Unread count
[**conversations_update**](ConversationsApi.md#conversations_update) | **PUT** /conversations/{id} | Edit a conversation



## conversations_add_message

> conversations_add_message(id, conversations_add_message_request)
Add a message

Add a message to an existing conversation. Response is similar to the GET/show action, except that only includes the latest message (i.e. what we just sent)  An array of user ids. Defaults to all of the current conversation recipients. To explicitly send a message to no other recipients, this array should consist of the logged-in user id.  An array of message ids from this conversation to send to recipients of the new message. Recipients who already had a copy of included messages will not be affected.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**conversations_add_message_request** | Option<[**ConversationsAddMessageRequest**](ConversationsAddMessageRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_add_recipients

> conversations_add_recipients(id, conversations_add_recipients_request)
Add recipients

Add recipients to an existing group conversation. Response is similar to the GET/show action, except that only includes the latest message (e.g. \"joe was added to the conversation by bob\")

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**conversations_add_recipients_request** | Option<[**ConversationsAddRecipientsRequest**](ConversationsAddRecipientsRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_batch_update

> String conversations_batch_update(conversations_batch_update_request)
Batch update conversations

Perform a change on a set of conversations. Operates asynchronously; use the {api:ProgressController#show progress endpoint} to query the status of an operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversations_batch_update_request** | Option<[**ConversationsBatchUpdateRequest**](ConversationsBatchUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/conversations \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'event=mark_as_read' \\   -d 'conversation_ids[]=1' \\   -d 'conversation_ids[]=2' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_batches

> conversations_batches(page, per_page)
Get running batches

Returns any currently running conversation batches for the current user. Conversation batches are created when a bulk private message is sent asynchronously (see the mode argument to the {api:ConversationsController#create create API action}).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_create

> conversations_create(conversations_create_request)
Create a conversation

Create a new conversation with one or more recipients. If there is already an existing private conversation with the given recipients, it will be reused.   (either numeric IDs or UUIDs prefixed with \"uuid:\"),   or course/group ids prefixed with \"course_\" or \"group_\" respectively, e.g.   recipients[]=1&recipients[]=uuid:W9GQIcdoDTqwX8mxIunDQQVL6WZTaGmpa5xovmCBx&recipients[]=course_3.   If the course/group has over 100 enrollments, 'bulk_message' and 'group_conversation' must be   set to true.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversations_create_request** | Option<[**ConversationsCreateRequest**](ConversationsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_destroy

> conversations_destroy(id)
Delete a conversation

Delete this conversation and its messages. Note that this only deletes this user's view of the conversation.  Response includes same fields as UPDATE action

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_index

> models::Conversation conversations_index(scope, filter, filter_mode_left_square_bracket_double_quote_and_double_quote_pipe_double_quote_or_double_quote_right_square_bracket, interleave_submissions, include_all_conversation_ids, include, page, per_page)
List conversations

Returns the paginated list of conversations for the current user, most recent ones first.   \"uuid:W9GQIcdoDTqwX8mxIunDQQVL6WZTaGmpa5xovmCB\", or \"course_456\".  For users, you can use either their numeric ID or UUID prefixed with \"uuid:\".  Can be an array (by setting \"filter[]\") or single value (by setting \"filter\")

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**scope** | Option<**String**> | When set, only return conversations of the specified type. For example, set to \"unread\" to return only conversations that haven't been read. The default behavior is to return all non-archived conversations (i.e. read and unread). |  |
**filter** | Option<[**Vec<String>**](String.md)> | [String, course_id|group_id|user_id] When set, only return conversations for the specified courses, groups or users. The id should be prefixed with its type, e.g. \"user_123\", |  |
**filter_mode_left_square_bracket_double_quote_and_double_quote_pipe_double_quote_or_double_quote_right_square_bracket** | Option<**String**> | When filter[] contains multiple filters, combine them with this mode, filtering conversations that at have at least all of the contexts (\"and\") or at least one of the contexts (\"or\") |  |
**interleave_submissions** | Option<**bool**> | (Obsolete) Submissions are no longer linked to conversations. This parameter is ignored. |  |
**include_all_conversation_ids** | Option<**bool**> | Default is false. If true, the top-level element of the response will be an object rather than an array, and will have the keys \"conversations\" which will contain the paged conversation data, and \"conversation_ids\" which will contain the ids of all conversations under this scope/filter in the same order. |  |
**include** | Option<[**Vec<String>**](String.md)> | [Optional, String, \"participant_avatars\"|\"uuid\"] \"participant_avatars\":: Optionally include an \"avatar_url\" key for each user participating in the conversation \"uuid\":: Optionally include an \"uuid\" key for each user participating in the conversation |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Conversation**](Conversation.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_mark_all_as_read

> conversations_mark_all_as_read()
Mark all as read

Mark all conversations as read.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_remove_messages

> conversations_remove_messages(id, conversations_remove_messages_request)
Delete a message

Delete messages from this conversation. Note that this only affects this user's view of the conversation. If all messages are deleted, the conversation will be as well (equivalent to DELETE)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**conversations_remove_messages_request** | Option<[**ConversationsRemoveMessagesRequest**](ConversationsRemoveMessagesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_show

> conversations_show(id, interleave_submissions, scope, filter, filter_mode_left_square_bracket_double_quote_and_double_quote_pipe_double_quote_or_double_quote_right_square_bracket, auto_mark_as_read)
Get a single conversation

Returns information for a single conversation for the current user. Response includes all fields that are present in the list/index action as well as messages and extended participant information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**interleave_submissions** | Option<**bool**> | (Obsolete) Submissions are no longer linked to conversations. This parameter is ignored. |  |
**scope** | Option<**String**> | Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} |  |
**filter** | Option<[**Vec<String>**](String.md)> | [String, course_id|group_id|user_id] Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} |  |
**filter_mode_left_square_bracket_double_quote_and_double_quote_pipe_double_quote_or_double_quote_right_square_bracket** | Option<**String**> | Used when generating \"visible\" in the API response. See the explanation under the {api:ConversationsController#index index API action} |  |
**auto_mark_as_read** | Option<**bool**> | Default true. If true, unread conversations will be automatically marked as read. This will default to false in a future API release, so clients should explicitly send true if that is the desired behavior. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## conversations_unread_count

> conversations_unread_count()
Unread count

Get the number of unread conversations for the current user

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


## conversations_update

> conversations_update(id, conversations_update_request)
Edit a conversation

Updates attributes for a single conversation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**conversations_update_request** | Option<[**ConversationsUpdateRequest**](ConversationsUpdateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

