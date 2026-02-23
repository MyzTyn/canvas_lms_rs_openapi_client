# \RubricsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rubrics_create**](RubricsApi.md#rubrics_create) | **POST** /courses/{course_id}/rubrics | Create a single rubric
[**rubrics_destroy**](RubricsApi.md#rubrics_destroy) | **DELETE** /courses/{course_id}/rubrics/{id} | Delete a single
[**rubrics_update**](RubricsApi.md#rubrics_update) | **PUT** /courses/{course_id}/rubrics/{id} | Update a single rubric



## rubrics_create

> rubrics_create(course_id, rubrics_create_request)
Create a single rubric

Returns the rubric with the given id.  Unfortunately this endpoint does not return a standard Rubric object, instead it returns a hash that looks like   { 'rubric': Rubric, 'rubric_association': RubricAssociation }  This may eventually be deprecated in favor of a more standardized return value, but that is not currently planned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**rubrics_create_request** | Option<[**RubricsCreateRequest**](RubricsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubrics_destroy

> String rubrics_destroy(course_id, id)
Delete a single

Deletes a Rubric and removes all RubricAssociations.

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


## rubrics_update

> rubrics_update(course_id, id, rubrics_update_request)
Update a single rubric

Returns the rubric with the given id.  Unfortunately this endpoint does not return a standard Rubric object, instead it returns a hash that looks like   { 'rubric': Rubric, 'rubric_association': RubricAssociation }  This may eventually be deprecated in favor of a more standardized return value, but that is not currently planned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | The id of the rubric | [required] |
**rubrics_update_request** | Option<[**RubricsUpdateRequest**](RubricsUpdateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

