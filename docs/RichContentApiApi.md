# \RichContentApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**rich_content_api_generate**](RichContentApiApi.md#rich_content_api_generate) | **POST** /rich_content/generate | Generate rich content



## rich_content_api_generate

> rich_content_api_generate(rich_content_api_generate_request)
Generate rich content

Generates a rich content.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**rich_content_api_generate_request** | Option<[**RichContentApiGenerateRequest**](RichContentApiGenerateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/rich_content/generate?course_id=1 \\     -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

