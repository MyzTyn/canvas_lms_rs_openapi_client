# \GradingPeriodSetsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grading_period_sets_create**](GradingPeriodSetsApi.md#grading_period_sets_create) | **POST** /accounts/{account_id}/grading_period_sets | Create a grading period set
[**grading_period_sets_destroy**](GradingPeriodSetsApi.md#grading_period_sets_destroy) | **DELETE** /accounts/{account_id}/grading_period_sets/{id} | Delete a grading period set
[**grading_period_sets_index**](GradingPeriodSetsApi.md#grading_period_sets_index) | **GET** /accounts/{account_id}/grading_period_sets | List grading period sets
[**grading_period_sets_update**](GradingPeriodSetsApi.md#grading_period_sets_update) | **PATCH** /accounts/{account_id}/grading_period_sets/{id} | Update a grading period set



## grading_period_sets_create

> grading_period_sets_create(account_id, grading_period_sets_create_request)
Create a grading period set

Create and return a new grading period set

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**grading_period_sets_create_request** | Option<[**GradingPeriodSetsCreateRequest**](GradingPeriodSetsCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_period_sets_destroy

> grading_period_sets_destroy(account_id, id)
Delete a grading period set

<b>204 No Content</b> response code is returned if the deletion was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_period_sets_index

> grading_period_sets_index(account_id, page, per_page)
List grading period sets

Returns the paginated list of grading period sets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_period_sets_update

> grading_period_sets_update(account_id, id, grading_period_sets_update_request)
Update a grading period set

Update an existing grading period set  <b>204 No Content</b> response code is returned if the update was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**grading_period_sets_update_request** | Option<[**GradingPeriodSetsUpdateRequest**](GradingPeriodSetsUpdateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

