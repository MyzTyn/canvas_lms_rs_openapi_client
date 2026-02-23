# \CourseNicknamesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**course_nicknames_clear**](CourseNicknamesApi.md#course_nicknames_clear) | **DELETE** /users/self/course_nicknames | Clear course nicknames
[**course_nicknames_delete**](CourseNicknamesApi.md#course_nicknames_delete) | **DELETE** /users/self/course_nicknames/{course_id} | Remove course nickname
[**course_nicknames_index**](CourseNicknamesApi.md#course_nicknames_index) | **GET** /users/self/course_nicknames | List course nicknames
[**course_nicknames_show**](CourseNicknamesApi.md#course_nicknames_show) | **GET** /users/self/course_nicknames/{course_id} | Get course nickname
[**course_nicknames_update**](CourseNicknamesApi.md#course_nicknames_update) | **PUT** /users/self/course_nicknames/{course_id} | Set course nickname



## course_nicknames_clear

> course_nicknames_clear()
Clear course nicknames

Remove all stored course nicknames.

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


## course_nicknames_delete

> String course_nicknames_delete(course_id)
Remove course nickname

Remove the nickname for the given course. Subsequent course API calls will return the actual name for the course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_nicknames_index

> models::CourseNickname course_nicknames_index(page, per_page)
List course nicknames

Returns all course nicknames you have set.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::CourseNickname**](CourseNickname.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_nicknames_show

> String course_nicknames_show(course_id)
Get course nickname

Returns the nickname for a specific course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## course_nicknames_update

> String course_nicknames_update(course_id, course_nicknames_update_request)
Set course nickname

Set a nickname for the given course. This will replace the course's name in output of API calls you make subsequently, as well as in selected places in the Canvas web user interface.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**course_nicknames_update_request** | Option<[**CourseNicknamesUpdateRequest**](CourseNicknamesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/self/course_nicknames/<course_id> \\   -X PUT \\   -F 'nickname=Physics' \\   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

