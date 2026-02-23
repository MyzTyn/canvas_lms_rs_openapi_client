# \PermissionsHelpApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**permissions_help_groups**](PermissionsHelpApi.md#permissions_help_groups) | **GET** /permissions/groups | Retrieve permission groups
[**permissions_help_help**](PermissionsHelpApi.md#permissions_help_help) | **GET** /permissions/{context_type}/{permission}/help | Get help text for permissions



## permissions_help_groups

> permissions_help_groups()
Retrieve permission groups

Retrieve information about groups of granular permissions  The return value is a dictionary of permission group keys to objects containing +label+ and +subtitle+ keys.

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


## permissions_help_help

> String permissions_help_help(context_type, permission)
Get help text for permissions

Retrieve information about what Canvas permissions do and considerations for their use.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_type** | **String** | Scope response to context_type | [required] |
**permission** | **String** | Scope response to permission | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

