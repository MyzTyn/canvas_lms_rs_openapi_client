# \ModerationSetApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**moderation_set_create**](ModerationSetApi.md#moderation_set_create) | **POST** /courses/{course_id}/assignments/{assignment_id}/moderated_students | Select students for moderation
[**moderation_set_index**](ModerationSetApi.md#moderation_set_index) | **GET** /courses/{course_id}/assignments/{assignment_id}/moderated_students | List students selected for moderation



## moderation_set_create

> models::User moderation_set_create(assignment_id, course_id, moderation_set_create_request)
Select students for moderation

Returns an array of users that were selected for moderation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**moderation_set_create_request** | Option<[**ModerationSetCreateRequest**](ModerationSetCreateRequest.md)> | Request body parameters |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## moderation_set_index

> models::User moderation_set_index(assignment_id, course_id, page, per_page)
List students selected for moderation

Returns a paginated list of students selected for moderation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assignment_id** | **String** | Scope response to assignment_id | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

