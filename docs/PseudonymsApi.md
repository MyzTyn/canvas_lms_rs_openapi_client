# \PseudonymsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**pseudonyms_create**](PseudonymsApi.md#pseudonyms_create) | **POST** /accounts/{account_id}/logins | Create a user login
[**pseudonyms_destroy**](PseudonymsApi.md#pseudonyms_destroy) | **DELETE** /users/{user_id}/logins/{id} | Delete a user login
[**pseudonyms_forgot_password**](PseudonymsApi.md#pseudonyms_forgot_password) | **POST** /users/reset_password | Kickoff password recovery flow
[**pseudonyms_index_for_accounts**](PseudonymsApi.md#pseudonyms_index_for_accounts) | **GET** /accounts/{account_id}/logins | List user logins
[**pseudonyms_index_for_users**](PseudonymsApi.md#pseudonyms_index_for_users) | **GET** /users/{user_id}/logins | List user logins
[**pseudonyms_update**](PseudonymsApi.md#pseudonyms_update) | **PUT** /accounts/{account_id}/logins/{id} | Edit a user login



## pseudonyms_create

> pseudonyms_create(account_id, pseudonyms_create_request)
Create a user login

Create a new login for an existing user in the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**pseudonyms_create_request** | Option<[**PseudonymsCreateRequest**](PseudonymsCreateRequest.md)> | Request body parameters  **Example Request:** ``` #create a facebook login for user with ID 123 curl 'https://<canvas>/api/v1/accounts/<account_id>/logins' \\      -F 'user[id]=123' \\      -F 'login[unique_id]=112233445566' \\      -F 'login[authentication_provider_id]=facebook' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pseudonyms_destroy

> pseudonyms_destroy(id, user_id)
Delete a user login

Delete an existing login.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pseudonyms_forgot_password

> pseudonyms_forgot_password()
Kickoff password recovery flow

Given a user email, generate a nonce and email it to the user

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pseudonyms_index_for_accounts

> pseudonyms_index_for_accounts(account_id, page, per_page)
List user logins

Given a user ID, return a paginated list of that user's logins for the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pseudonyms_index_for_users

> pseudonyms_index_for_users(user_id, page, per_page)
List user logins

Given a user ID, return a paginated list of that user's logins for the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## pseudonyms_update

> pseudonyms_update(account_id, id, pseudonyms_update_request)
Edit a user login

Update an existing login for a user in the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**pseudonyms_update_request** | Option<[**PseudonymsUpdateRequest**](PseudonymsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/:account_id/logins/:login_id \\   -H \"Authorization: Bearer <ACCESS-TOKEN>\" \\   -X PUT ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

