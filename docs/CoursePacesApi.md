# \CoursePacesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**course_paces_api_show**](CoursePacesApi.md#course_paces_api_show) | **GET** /courses/{course_id}/course_pacing/{id} | Show a Course pace
[**course_paces_create**](CoursePacesApi.md#course_paces_create) | **POST** /courses/{course_id}/course_pacing | Create a Course pace
[**course_paces_destroy**](CoursePacesApi.md#course_paces_destroy) | **DELETE** /courses/{course_id}/course_pacing/{id} | Delete a Course pace
[**course_paces_update**](CoursePacesApi.md#course_paces_update) | **PUT** /courses/{course_id}/course_pacing/{id} | Update a Course pace



## course_paces_api_show

> String course_paces_api_show(course_id, id, course_pace_id)
Show a Course pace

Returns a course pace for the course and pace id provided

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**course_pace_id** | **i32** | The id of the course_pace | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_paces_create

> String course_paces_create(course_id, course_paces_create_request)
Create a Course pace

Creates a new course pace with specified parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**course_paces_create_request** | Option<[**CoursePacesCreateRequest**](CoursePacesCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/1/course_pacing \\   -X POST \\   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_paces_destroy

> String course_paces_destroy(course_id, id, course_pace_id)
Delete a Course pace

Returns the updated course pace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**course_pace_id** | **i32** | The id of the course_pace | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_paces_update

> String course_paces_update(course_id, id, course_paces_update_request)
Update a Course pace

Returns the updated course pace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**course_paces_update_request** | Option<[**CoursePacesUpdateRequest**](CoursePacesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/1/course_pacing/1 \\   -X PUT \\   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

