# \ObserverPairingCodesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**observer_pairing_codes_api_create**](ObserverPairingCodesApiApi.md#observer_pairing_codes_api_create) | **POST** /users/{user_id}/observer_pairing_codes | Create observer pairing code



## observer_pairing_codes_api_create

> String observer_pairing_codes_api_create(user_id)
Create observer pairing code

If the user is a student, will generate a code to be used with self registration or observees APIs to link another user to this student.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

