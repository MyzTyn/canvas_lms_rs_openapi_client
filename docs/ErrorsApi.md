# \ErrorsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**errors_create**](ErrorsApi.md#errors_create) | **POST** /error_reports | Create Error Report



## errors_create

> errors_create(errors_create_request)
Create Error Report

Create a new error report documenting an experienced problem  Performs the same action as when a user uses the \"help -> report a problem\" dialog.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**errors_create_request** | Option<[**ErrorsCreateRequest**](ErrorsCreateRequest.md)> | Request body parameters  **Example Request:** ``` # Create error report curl 'https://<canvas>/api/v1/error_reports' \\       -X POST \\       -F 'error[subject]=\"things are broken\"' \\       -F 'error[url]=http://<canvas>/courses/1' \\       -F 'error[description]=\"All my thoughts on what I saw\"' \\       -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

