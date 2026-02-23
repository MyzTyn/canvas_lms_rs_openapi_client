# \TokensApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tokens_create**](TokensApi.md#tokens_create) | **POST** /users/{user_id}/tokens | Create an access token
[**tokens_destroy**](TokensApi.md#tokens_destroy) | **DELETE** /users/{user_id}/tokens/{id} | Delete an access token
[**tokens_show**](TokensApi.md#tokens_show) | **GET** /users/{user_id}/tokens/{id} | Show an access token
[**tokens_update**](TokensApi.md#tokens_update) | **PUT** /users/{user_id}/tokens/{id} | Update an access token
[**tokens_user_generated_tokens**](TokensApi.md#tokens_user_generated_tokens) | **GET** /users/{user_id}/user_generated_tokens | List access tokens for a user



## tokens_create

> tokens_create(user_id, tokens_create_request)
Create an access token

Create a new access token for the specified user. If the user is not the current user, the token will be created as \"pending\", and must be activated by the user before it can be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**tokens_create_request** | Option<[**TokensCreateRequest**](TokensCreateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_destroy

> tokens_destroy(id, user_id)
Delete an access token

The ID can be the actual database ID of the token, or the 'token_hint' value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_show

> tokens_show(id, user_id)
Show an access token

The ID can be the actual database ID of the token, or the 'token_hint' value.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_update

> tokens_update(id, user_id, tokens_update_request)
Update an access token

Update an existing access token.  The ID can be the actual database ID of the token, or the 'token_hint' value.  Regenerating an expired token requires a new expiration date.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**tokens_update_request** | Option<[**TokensUpdateRequest**](TokensUpdateRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tokens_user_generated_tokens

> models::Token tokens_user_generated_tokens(user_id, per_page, page)
List access tokens for a user

Returns a list of manually generated access tokens for the specified user. Note that the actual token values are only returned when the token is first created.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**per_page** | Option<**i32**> | The number of results to return per page. Defaults to 10. Maximum of 100. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |

### Return type

[**models::Token**](Token.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

