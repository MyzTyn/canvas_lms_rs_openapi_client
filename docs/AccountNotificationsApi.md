# \AccountNotificationsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**account_notifications_create**](AccountNotificationsApi.md#account_notifications_create) | **POST** /accounts/{account_id}/account_notifications | Create a global notification
[**account_notifications_show**](AccountNotificationsApi.md#account_notifications_show) | **GET** /accounts/{account_id}/account_notifications/{id} | Show a global notification
[**account_notifications_update**](AccountNotificationsApi.md#account_notifications_update) | **PUT** /accounts/{account_id}/account_notifications/{id} | Update a global notification
[**account_notifications_user_close_notification**](AccountNotificationsApi.md#account_notifications_user_close_notification) | **DELETE** /accounts/{account_id}/account_notifications/{id} | Close notification for user. Destroy notification for admin
[**account_notifications_user_index**](AccountNotificationsApi.md#account_notifications_user_index) | **GET** /accounts/{account_id}/account_notifications | Index of active global notification for the user



## account_notifications_create

> account_notifications_create(account_id, account_notifications_create_request)
Create a global notification

Create and return a new global notification for an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**account_notifications_create_request** | Option<[**AccountNotificationsCreateRequest**](AccountNotificationsCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl -X POST -H 'Authorization: Bearer <token>' \\ https://<canvas>/api/v1/accounts/2/account_notifications \\ -d 'account_notification[subject]=New notification' \\ -d 'account_notification[start_at]=2014-01-01T00:00:00Z' \\ -d 'account_notification[end_at]=2014-02-01T00:00:00Z' \\ -d 'account_notification[message]=This is a global notification' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_notifications_show

> String account_notifications_show(account_id, id)
Show a global notification

Returns a global notification for the current user A notification that has been closed by the user will not be returned

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_notifications_update

> account_notifications_update(account_id, id, account_notifications_update_request)
Update a global notification

Update global notification for an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**account_notifications_update_request** | Option<[**AccountNotificationsUpdateRequest**](AccountNotificationsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl -X PUT -H 'Authorization: Bearer <token>' \\ https://<canvas>/api/v1/accounts/2/account_notifications/1 \\ -d 'account_notification[subject]=New notification' \\ -d 'account_notification[start_at]=2014-01-01T00:00:00Z' \\ -d 'account_notification[end_at]=2014-02-01T00:00:00Z' \\ -d 'account_notification[message]=This is a global notification' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_notifications_user_close_notification

> String account_notifications_user_close_notification(account_id, id, remove)
Close notification for user. Destroy notification for admin

If the current user no longer wants to see this account notification, it can be closed with this call. This affects the current user only.  If the current user is an admin and they pass a remove parameter with a value of \"true\", the account notification will be destroyed. This affects all users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**remove** | Option<**bool**> | Destroy the account notification. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## account_notifications_user_index

> models::AccountNotification account_notifications_user_index(account_id, include_past, include_all, show_is_closed)
Index of active global notification for the user

Returns a list of all global notifications in the account for the current user Any notifications that have been closed by the user will not be returned, unless a include_past parameter is passed in as true. Admins can request all global notifications for the account by passing in an include_all parameter.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**include_past** | Option<**bool**> | Include past and dismissed global announcements. |  |
**include_all** | Option<**bool**> | Include all global announcements, regardless of user's role or availability date. Only available to account admins. |  |
**show_is_closed** | Option<**bool**> | Include a flag for each notification indicating whether it has been read by the user. |  |

### Return type

[**models::AccountNotification**](AccountNotification.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

