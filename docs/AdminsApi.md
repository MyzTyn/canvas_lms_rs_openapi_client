# \AdminsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**admins_create**](AdminsApi.md#admins_create) | **POST** /accounts/{account_id}/admins | Make an account admin
[**admins_destroy**](AdminsApi.md#admins_destroy) | **DELETE** /accounts/{account_id}/admins/{user_id} | Remove account admin
[**admins_index**](AdminsApi.md#admins_index) | **GET** /accounts/{account_id}/admins | List account admins
[**admins_self_roles**](AdminsApi.md#admins_self_roles) | **GET** /accounts/{account_id}/admins/self | List my admin roles



## admins_create

> String admins_create(account_id, admins_create_request)
Make an account admin

Flag an existing user as an admin within the account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**admins_create_request** | Option<[**AdminsCreateRequest**](AdminsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admins_destroy

> String admins_destroy(account_id, user_id, role_id, role)
Remove account admin

Remove the rights associated with an account admin role from a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**role_id** | **i32** | The id of the role representing the user's admin relationship with the account. | [required] |
**role** | Option<**String**> | [DEPRECATED] Account role to remove from the user. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admins_index

> models::Admin admins_index(account_id, user_id, search_term, include_deleted, page, per_page)
List account admins

A paginated list of the admins in the account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**user_id** | Option<[**Vec<String>**](String.md)> | [[Integer]] Scope the results to those with user IDs equal to any of the IDs specified here. |  |
**search_term** | Option<**String**> | The partial name or full ID of the admins to match and return in the results list. Must be at least 2 characters. |  |
**include_deleted** | Option<**bool**> | When set to true, returns admins who have been deleted |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Admin**](Admin.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## admins_self_roles

> models::Admin admins_self_roles(account_id, page, per_page)
List my admin roles

A paginated list of the current user's roles in the account. The results are the same as those returned by the {api:AdminsController#index List account admins} endpoint with +user_id+ set to +self+, except the \"Admins - Add / Remove\" permission is not required.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Admin**](Admin.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

