# \ProgressApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**progress_cancel**](ProgressApi.md#progress_cancel) | **POST** /progress/{id}/cancel | Cancel progress
[**progress_show**](ProgressApi.md#progress_show) | **GET** /progress/{id} | Query progress



## progress_cancel

> String progress_cancel(id)
Cancel progress

Cancel an asynchronous job associated with a Progress object If you include \"message\" in the POSTed data, it will be set on the Progress and returned. This is handy to distinguish between cancel and fail for a workflow_state of \"failed\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## progress_show

> String progress_show(id)
Query progress

Return completion and status information about an asynchronous job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

