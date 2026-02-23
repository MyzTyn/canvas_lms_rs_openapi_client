# \BrandConfigsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**brand_configs_api_show**](BrandConfigsApiApi.md#brand_configs_api_show) | **GET** /brand_variables | Get the brand config variables that should be used for this domain
[**brand_configs_api_show_context_for_accounts**](BrandConfigsApiApi.md#brand_configs_api_show_context_for_accounts) | **GET** /accounts/{account_id}/brand_variables | Get the brand config variables for a sub-account or course
[**brand_configs_api_show_context_for_courses**](BrandConfigsApiApi.md#brand_configs_api_show_context_for_courses) | **GET** /courses/{course_id}/brand_variables | Get the brand config variables for a sub-account or course



## brand_configs_api_show

> brand_configs_api_show()
Get the brand config variables that should be used for this domain

Will redirect to a static json file that has all of the brand variables used by this account. Even though this is a redirect, do not store the redirected url since if the account makes any changes it will redirect to a new url. Needs no authentication.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## brand_configs_api_show_context_for_accounts

> brand_configs_api_show_context_for_accounts(account_id)
Get the brand config variables for a sub-account or course

Will redirect to a static json file that has all of the brand variables used by the provided context. Even though this is a redirect, do not store the redirected url since if the sub-account makes any changes it will redirect to a new url.

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


## brand_configs_api_show_context_for_courses

> brand_configs_api_show_context_for_courses(course_id)
Get the brand config variables for a sub-account or course

Will redirect to a static json file that has all of the brand variables used by the provided context. Even though this is a redirect, do not store the redirected url since if the sub-account makes any changes it will redirect to a new url.

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

