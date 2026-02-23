# \CareerExperienceApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**career_experience_enabled**](CareerExperienceApi.md#career_experience_enabled) | **GET** /career/enabled | Check if Canvas Career is enabled
[**career_experience_experience_summary**](CareerExperienceApi.md#career_experience_experience_summary) | **GET** /career/experience_summary | Get current and available experiences
[**career_experience_switch_experience**](CareerExperienceApi.md#career_experience_switch_experience) | **POST** /career/switch_experience | Switch experience
[**career_experience_switch_role**](CareerExperienceApi.md#career_experience_switch_role) | **POST** /career/switch_role | Switch role



## career_experience_enabled

> serde_json::Value career_experience_enabled()
Check if Canvas Career is enabled

Returns whether the root account has Canvas Career (Horizon) enabled in at least one subaccount.

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## career_experience_experience_summary

> String career_experience_experience_summary()
Get current and available experiences

Returns the current user's active experience and available experiences they can switch to.

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


## career_experience_switch_experience

> serde_json::Value career_experience_switch_experience(career_experience_switch_experience_request)
Switch experience

Switch the current user's active experience to the specified one.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**career_experience_switch_experience_request** | Option<[**CareerExperienceSwitchExperienceRequest**](CareerExperienceSwitchExperienceRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST https://<canvas>/api/v1/career/switch_experience \\   -H 'Authorization: Bearer <token>' \\   -d 'experience=academic' ``` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## career_experience_switch_role

> serde_json::Value career_experience_switch_role(career_experience_switch_role_request)
Switch role

Switch the current user's role within the current experience.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**career_experience_switch_role_request** | Option<[**CareerExperienceSwitchRoleRequest**](CareerExperienceSwitchRoleRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST https://<canvas>/api/v1/career/switch_role \\   -H 'Authorization: Bearer <token>' \\   -d 'role=learner' ``` |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

