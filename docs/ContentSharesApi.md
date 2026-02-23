# \ContentSharesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_shares_add_users**](ContentSharesApi.md#content_shares_add_users) | **POST** /users/{user_id}/content_shares/{id}/add_users | Add users to content share
[**content_shares_create**](ContentSharesApi.md#content_shares_create) | **POST** /users/{user_id}/content_shares | Create a content share
[**content_shares_destroy**](ContentSharesApi.md#content_shares_destroy) | **DELETE** /users/{user_id}/content_shares/{id} | Remove content share
[**content_shares_index_for_users**](ContentSharesApi.md#content_shares_index_for_users) | **GET** /users/{user_id}/content_shares/sent | List content shares
[**content_shares_index_for_users2**](ContentSharesApi.md#content_shares_index_for_users2) | **GET** /users/{user_id}/content_shares/received | List content shares
[**content_shares_show**](ContentSharesApi.md#content_shares_show) | **GET** /users/{user_id}/content_shares/{id} | Get content share
[**content_shares_unread_count**](ContentSharesApi.md#content_shares_unread_count) | **GET** /users/{user_id}/content_shares/unread_count | Get unread shares count
[**content_shares_update**](ContentSharesApi.md#content_shares_update) | **PUT** /users/{user_id}/content_shares/{id} | Update a content share



## content_shares_add_users

> String content_shares_add_users(id, user_id, content_shares_add_users_request)
Add users to content share

Send a previously created content share to additional users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**content_shares_add_users_request** | Option<[**ContentSharesAddUsersRequest**](ContentSharesAddUsersRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST 'https://<canvas>/api/v1/users/self/content_shares/123/add_users?receiver_ids[]=789' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_create

> String content_shares_create(user_id, content_shares_create_request)
Create a content share

Share content directly between two or more users

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**content_shares_create_request** | Option<[**ContentSharesCreateRequest**](ContentSharesCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/self/content_shares \\       -d 'content_type=assignment' \\       -d 'content_id=1' \\       -H 'Authorization: Bearer <token>' \\       -X POST ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_destroy

> content_shares_destroy(id, user_id)
Remove content share

Remove a content share from your list. Use +self+ as the user_id. Note that this endpoint does not delete other users' copies of the content share.

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


## content_shares_index_for_users

> serde_json::Value content_shares_index_for_users(user_id, page, per_page)
List content shares

Return a paginated list of content shares a user has sent or received. Use +self+ as the user_id to retrieve your own content shares. Only linked observers and administrators may view other users' content shares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_index_for_users2

> serde_json::Value content_shares_index_for_users2(user_id, page, per_page)
List content shares

Return a paginated list of content shares a user has sent or received. Use +self+ as the user_id to retrieve your own content shares. Only linked observers and administrators may view other users' content shares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_show

> String content_shares_show(id, user_id)
Get content share

Return information about a single content share. You may use +self+ as the user_id to retrieve your own content share.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_unread_count

> serde_json::Value content_shares_unread_count(user_id)
Get unread shares count

Return the number of content shares a user has received that have not yet been read. Use +self+ as the user_id to retrieve your own content shares. Only linked observers and administrators may view other users' content shares.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_shares_update

> String content_shares_update(id, user_id, content_shares_update_request)
Update a content share

Mark a content share read or unread

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**content_shares_update_request** | Option<[**ContentSharesUpdateRequest**](ContentSharesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl -X PUT 'https://<canvas>/api/v1/users/self/content_shares/123?read_state=read' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

