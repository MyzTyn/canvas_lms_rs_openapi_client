# \LearningMasteryGradebookSettingsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**learning_mastery_gradebook_settings_api_show**](LearningMasteryGradebookSettingsApiApi.md#learning_mastery_gradebook_settings_api_show) | **GET** /courses/{course_id}/learning_mastery_gradebook_settings | Get Learning Mastery Gradebook Settings
[**learning_mastery_gradebook_settings_api_update**](LearningMasteryGradebookSettingsApiApi.md#learning_mastery_gradebook_settings_api_update) | **PUT** /courses/{course_id}/learning_mastery_gradebook_settings | Update Learning Mastery Gradebook Settings



## learning_mastery_gradebook_settings_api_show

> serde_json::Value learning_mastery_gradebook_settings_api_show(course_id)
Get Learning Mastery Gradebook Settings

Get the current user's Learning Mastery Gradebook settings for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## learning_mastery_gradebook_settings_api_update

> serde_json::Value learning_mastery_gradebook_settings_api_update(course_id)
Update Learning Mastery Gradebook Settings

Update the current user's Learning Mastery Gradebook settings for the current context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

