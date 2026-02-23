# \RubricAssessmentsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rubric_assessments_create**](RubricAssessmentsApi.md#rubric_assessments_create) | **POST** /courses/{course_id}/rubric_associations/{rubric_association_id}/rubric_assessments | Create a single rubric assessment
[**rubric_assessments_destroy**](RubricAssessmentsApi.md#rubric_assessments_destroy) | **DELETE** /courses/{course_id}/rubric_associations/{rubric_association_id}/rubric_assessments/{id} | Delete a single rubric assessment
[**rubric_assessments_update**](RubricAssessmentsApi.md#rubric_assessments_update) | **PUT** /courses/{course_id}/rubric_associations/{rubric_association_id}/rubric_assessments/{id} | Update a single rubric assessment



## rubric_assessments_create

> rubric_assessments_create(course_id, rubric_association_id, rubric_assessments_create_request)
Create a single rubric assessment

Returns the rubric assessment with the given id. The returned object also provides the information of   :ratings, :assessor_name, :related_group_submissions_and_assessments, :artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**rubric_association_id** | **String** | The id of the object with which this rubric assessment is associated | [required] |
**rubric_assessments_create_request** | Option<[**RubricAssessmentsCreateRequest**](RubricAssessmentsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubric_assessments_destroy

> String rubric_assessments_destroy(course_id, id, rubric_association_id)
Delete a single rubric assessment

Deletes a rubric assessment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**rubric_association_id** | **String** | Scope response to rubric_association_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## rubric_assessments_update

> rubric_assessments_update(course_id, id, rubric_association_id, rubric_assessments_create_request)
Update a single rubric assessment

Returns the rubric assessment with the given id. The returned object also provides the information of   :ratings, :assessor_name, :related_group_submissions_and_assessments, :artifact

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The id of the course | [required] |
**id** | **String** | The id of the rubric assessment | [required] |
**rubric_association_id** | **String** | The id of the object with which this rubric assessment is associated | [required] |
**rubric_assessments_create_request** | Option<[**RubricAssessmentsCreateRequest**](RubricAssessmentsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

