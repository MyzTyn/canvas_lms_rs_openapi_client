# \GradingPeriodsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**grading_periods_batch_update_for_courses**](GradingPeriodsApi.md#grading_periods_batch_update_for_courses) | **PATCH** /courses/{course_id}/grading_periods/batch_update | Batch update grading periods
[**grading_periods_batch_update_other**](GradingPeriodsApi.md#grading_periods_batch_update_other) | **PATCH** /grading_period_sets/{set_id}/grading_periods/batch_update | Batch update grading periods
[**grading_periods_destroy_for_accounts**](GradingPeriodsApi.md#grading_periods_destroy_for_accounts) | **DELETE** /accounts/{account_id}/grading_periods/{id} | Delete a grading period
[**grading_periods_destroy_for_courses**](GradingPeriodsApi.md#grading_periods_destroy_for_courses) | **DELETE** /courses/{course_id}/grading_periods/{id} | Delete a grading period
[**grading_periods_index_for_accounts**](GradingPeriodsApi.md#grading_periods_index_for_accounts) | **GET** /accounts/{account_id}/grading_periods | List grading periods
[**grading_periods_index_for_courses**](GradingPeriodsApi.md#grading_periods_index_for_courses) | **GET** /courses/{course_id}/grading_periods | List grading periods
[**grading_periods_show**](GradingPeriodsApi.md#grading_periods_show) | **GET** /courses/{course_id}/grading_periods/{id} | Get a single grading period
[**grading_periods_update**](GradingPeriodsApi.md#grading_periods_update) | **PUT** /courses/{course_id}/grading_periods/{id} | Update a single grading period



## grading_periods_batch_update_for_courses

> grading_periods_batch_update_for_courses(course_id, grading_periods_batch_update_for_courses_request)
Batch update grading periods

Update multiple grading periods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**grading_periods_batch_update_for_courses_request** | Option<[**GradingPeriodsBatchUpdateForCoursesRequest**](GradingPeriodsBatchUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_batch_update_other

> grading_periods_batch_update_other(set_id, grading_periods_batch_update_for_courses_request)
Batch update grading periods

Update multiple grading periods

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**set_id** | **String** | ID of the set | [required] |
**grading_periods_batch_update_for_courses_request** | Option<[**GradingPeriodsBatchUpdateForCoursesRequest**](GradingPeriodsBatchUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_destroy_for_accounts

> grading_periods_destroy_for_accounts(account_id, id)
Delete a grading period

<b>204 No Content</b> response code is returned if the deletion was successful.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_destroy_for_courses

> grading_periods_destroy_for_courses(course_id, id)
Delete a grading period

<b>204 No Content</b> response code is returned if the deletion was successful.

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


## grading_periods_index_for_accounts

> grading_periods_index_for_accounts(account_id)
List grading periods

Returns the paginated list of grading periods for the current course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_index_for_courses

> grading_periods_index_for_courses(course_id)
List grading periods

Returns the paginated list of grading periods for the current course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_show

> grading_periods_show(course_id, id)
Get a single grading period

Returns the grading period with the given id

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## grading_periods_update

> grading_periods_update(course_id, id, grading_periods_update_request)
Update a single grading period

Update an existing grading period.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**grading_periods_update_request** | Option<[**GradingPeriodsUpdateRequest**](GradingPeriodsUpdateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

