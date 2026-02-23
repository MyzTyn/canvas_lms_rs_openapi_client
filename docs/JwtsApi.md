# \JwtsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**jwts_create**](JwtsApi.md#jwts_create) | **POST** /jwts | Create JWT
[**jwts_refresh**](JwtsApi.md#jwts_refresh) | **POST** /jwts/refresh | Refresh JWT



## jwts_create

> String jwts_create(jwts_create_request)
Create JWT

Create a unique JWT for use with other Canvas services  Generates a different JWT each time it's called. Each JWT expires after a short window (1 hour)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwts_create_request** | Option<[**JwtsCreateRequest**](JwtsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/jwts' \\       -X POST \\       -H \"Accept: application/json\" \\       -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## jwts_refresh

> String jwts_refresh(jwts_refresh_request)
Refresh JWT

Refresh a JWT for use with other canvas services  Generates a different JWT each time it's called, each one expires after a short window (1 hour).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**jwts_refresh_request** | Option<[**JwtsRefreshRequest**](JwtsRefreshRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/jwts/refresh' \\       -X POST \\       -H \"Accept: application/json\" \\       -H 'Authorization: Bearer <token>'       -d 'jwt=<jwt>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

