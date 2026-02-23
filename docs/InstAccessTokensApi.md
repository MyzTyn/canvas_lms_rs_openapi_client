# \InstAccessTokensApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**inst_access_tokens_create**](InstAccessTokensApi.md#inst_access_tokens_create) | **POST** /inst_access_tokens | Create InstAccess token



## inst_access_tokens_create

> String inst_access_tokens_create()
Create InstAccess token

Create a unique, encrypted InstAccess token.  Generates a different InstAccess token each time it's called, each one expires after a short window (1 hour).

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

