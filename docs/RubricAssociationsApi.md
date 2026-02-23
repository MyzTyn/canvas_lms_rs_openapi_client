# \RubricAssociationsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rubric_associations_create**](RubricAssociationsApi.md#rubric_associations_create) | **POST** /courses/{course_id}/rubric_associations | Create a RubricAssociation
[**rubric_associations_destroy**](RubricAssociationsApi.md#rubric_associations_destroy) | **DELETE** /courses/{course_id}/rubric_associations/{id} | Delete a RubricAssociation
[**rubric_associations_update**](RubricAssociationsApi.md#rubric_associations_update) | **PUT** /courses/{course_id}/rubric_associations/{id} | Update a RubricAssociation



## rubric_associations_create

> String rubric_associations_create(course_id, rubric_associations_create_request)
Create a RubricAssociation

Returns the rubric with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**rubric_associations_create_request** | Option<[**RubricAssociationsCreateRequest**](RubricAssociationsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubric_associations_destroy

> String rubric_associations_destroy(course_id, id)
Delete a RubricAssociation

Delete the RubricAssociation with the given ID

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


## rubric_associations_update

> String rubric_associations_update(course_id, id, rubric_associations_create_request)
Update a RubricAssociation

Returns the rubric with the given id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | The id of the RubricAssociation to update | [required] |
**rubric_associations_create_request** | Option<[**RubricAssociationsCreateRequest**](RubricAssociationsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

