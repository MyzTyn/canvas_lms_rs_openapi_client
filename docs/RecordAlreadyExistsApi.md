# \RecordAlreadyExistsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**record_already_exists_create**](RecordAlreadyExistsApi.md#record_already_exists_create) | **POST** /courses/{id}/late_policy | Create a late policy
[**record_already_exists_show**](RecordAlreadyExistsApi.md#record_already_exists_show) | **GET** /courses/{id}/late_policy | Get a late policy
[**record_already_exists_update**](RecordAlreadyExistsApi.md#record_already_exists_update) | **PATCH** /courses/{id}/late_policy | Patch a late policy



## record_already_exists_create

> record_already_exists_create(id, record_already_exists_create_request)
Create a late policy

Create a late policy. If the course already has a late policy, a bad_request is returned since there can only be one late policy per course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**record_already_exists_create_request** | Option<[**RecordAlreadyExistsCreateRequest**](RecordAlreadyExistsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_already_exists_show

> record_already_exists_show(id)
Get a late policy

Returns the late policy for a course.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## record_already_exists_update

> record_already_exists_update(id, record_already_exists_create_request)
Patch a late policy

Patch a late policy. No body is returned upon success.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**record_already_exists_create_request** | Option<[**RecordAlreadyExistsCreateRequest**](RecordAlreadyExistsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

