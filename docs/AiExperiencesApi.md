# \AiExperiencesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**ai_experiences_ai_conversation_show**](AiExperiencesApi.md#ai_experiences_ai_conversation_show) | **GET** /courses/{course_id}/ai_experiences/{id}/ai_conversations/{conversation_id} | Show student AI conversation
[**ai_experiences_ai_conversations_index**](AiExperiencesApi.md#ai_experiences_ai_conversations_index) | **GET** /courses/{course_id}/ai_experiences/{id}/ai_conversations | List student AI conversations
[**ai_experiences_create**](AiExperiencesApi.md#ai_experiences_create) | **POST** /courses/{course_id}/ai_experiences | Create an AI experience
[**ai_experiences_destroy**](AiExperiencesApi.md#ai_experiences_destroy) | **DELETE** /courses/{course_id}/ai_experiences/{id} | Delete an AI experience
[**ai_experiences_edit**](AiExperiencesApi.md#ai_experiences_edit) | **GET** /courses/{course_id}/ai_experiences/{id}/edit | Show edit AI experience form
[**ai_experiences_index**](AiExperiencesApi.md#ai_experiences_index) | **GET** /courses/{course_id}/ai_experiences | List AI experiences
[**ai_experiences_new**](AiExperiencesApi.md#ai_experiences_new) | **GET** /courses/{course_id}/ai_experiences/new | Show new AI experience form
[**ai_experiences_show**](AiExperiencesApi.md#ai_experiences_show) | **GET** /courses/{course_id}/ai_experiences/{id} | Show an AI experience
[**ai_experiences_update**](AiExperiencesApi.md#ai_experiences_update) | **PUT** /courses/{course_id}/ai_experiences/{id} | Update an AI experience



## ai_experiences_ai_conversation_show

> String ai_experiences_ai_conversation_show(conversation_id, course_id, id)
Show student AI conversation

Retrieve a specific student's AI conversation with full message history. Only available to teachers and course managers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conversation_id** | **String** | Scope response to conversation_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_ai_conversations_index

> serde_json::Value ai_experiences_ai_conversations_index(course_id, id)
List student AI conversations

Retrieve the latest AI conversation for each student in the course for this AI experience. Only available to teachers and course managers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
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


## ai_experiences_create

> String ai_experiences_create(course_id, ai_experiences_create_request)
Create an AI experience

Create a new AI experience for the specified course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**ai_experiences_create_request** | Option<[**AiExperiencesCreateRequest**](AiExperiencesCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_destroy

> String ai_experiences_destroy(course_id, id)
Delete an AI experience

Delete an AI experience (soft delete - marks as deleted)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_edit

> ai_experiences_edit(course_id, id)
Show edit AI experience form

Display the form for editing an existing AI experience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_index

> models::AiExperience ai_experiences_index(course_id, workflow_state)
List AI experiences

Retrieve the paginated list of AI experiences for a course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**workflow_state** | Option<**String**> | Only return experiences with the specified workflow state. Allowed values: published, unpublished, deleted |  |

### Return type

[**models::AiExperience**](AiExperience.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_new

> ai_experiences_new(course_id)
Show new AI experience form

Display the form for creating a new AI experience

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


## ai_experiences_show

> String ai_experiences_show(course_id, id)
Show an AI experience

Retrieve an AI experience by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ai_experiences_update

> String ai_experiences_update(course_id, id, ai_experiences_update_request)
Update an AI experience

Update an existing AI experience

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**ai_experiences_update_request** | Option<[**AiExperiencesUpdateRequest**](AiExperiencesUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

