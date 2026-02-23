# \AiConversationsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ai_conversations_active_conversation**](AiConversationsApi.md#ai_conversations_active_conversation) | **GET** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations | Get active conversation
[**ai_conversations_create**](AiConversationsApi.md#ai_conversations_create) | **POST** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations | Create AI conversation
[**ai_conversations_destroy**](AiConversationsApi.md#ai_conversations_destroy) | **DELETE** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations/{id} | Delete AI conversation
[**ai_conversations_evaluation**](AiConversationsApi.md#ai_conversations_evaluation) | **GET** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations/{id}/evaluation | Get conversation evaluation
[**ai_conversations_post_message**](AiConversationsApi.md#ai_conversations_post_message) | **POST** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations/{id}/messages | Post message to conversation
[**ai_conversations_show**](AiConversationsApi.md#ai_conversations_show) | **GET** /courses/{course_id}/ai_experiences/{ai_experience_id}/conversations/{id} | Show conversation



## ai_conversations_active_conversation

> serde_json::Value ai_conversations_active_conversation(ai_experience_id, course_id)
Get active conversation

Get the active conversation for the current user and AI experience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_conversations_create

> serde_json::Value ai_conversations_create(ai_experience_id, course_id)
Create AI conversation

Initialize a new conversation with the AI experience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_conversations_destroy

> serde_json::Value ai_conversations_destroy(ai_experience_id, course_id, id)
Delete AI conversation

Mark a conversation as completed/deleted

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_conversations_evaluation

> serde_json::Value ai_conversations_evaluation(ai_experience_id, course_id, id)
Get conversation evaluation

Fetch evaluation data for a conversation from the llm-conversation service

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_conversations_post_message

> serde_json::Value ai_conversations_post_message(ai_experience_id, course_id, id, ai_conversations_post_message_request)
Post message to conversation

Send a message to an existing conversation and get the AI response

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**ai_conversations_post_message_request** | Option<[**AiConversationsPostMessageRequest**](AiConversationsPostMessageRequest.md)> | Request body parameters |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_conversations_show

> serde_json::Value ai_conversations_show(ai_experience_id, course_id, id)
Show conversation

Get a specific conversation by ID (for teachers viewing student conversations)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ai_experience_id** | **String** | Scope response to ai_experience_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

