# \UserObserveesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**user_observees_create**](UserObserveesApi.md#user_observees_create) | **POST** /users/{user_id}/observees | Add an observee with credentials
[**user_observees_destroy**](UserObserveesApi.md#user_observees_destroy) | **DELETE** /users/{user_id}/observees/{observee_id} | Remove an observee
[**user_observees_index**](UserObserveesApi.md#user_observees_index) | **GET** /users/{user_id}/observees | List linked observees
[**user_observees_observers**](UserObserveesApi.md#user_observees_observers) | **GET** /users/{user_id}/observers | List linked observers
[**user_observees_show**](UserObserveesApi.md#user_observees_show) | **GET** /users/{user_id}/observees/{observee_id} | Show an observee
[**user_observees_show_observer**](UserObserveesApi.md#user_observees_show_observer) | **GET** /users/{user_id}/observers/{observer_id} | Show an observer
[**user_observees_update**](UserObserveesApi.md#user_observees_update) | **PUT** /users/{user_id}/observees/{observee_id} | Add an observee



## user_observees_create

> String user_observees_create(user_id, user_observees_create_request)
Add an observee with credentials

Register the given user to observe another user, given the observee's credentials.  *Note:* all users are allowed to add their own observees, given the observee's credentials or access token are provided. Administrators can add observees given credentials, access token or the {api:UserObserveesController#update observee's id}.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**user_observees_create_request** | Option<[**UserObserveesCreateRequest**](UserObserveesCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/<user_id>/observees \\      -X POST \\      -H 'Authorization: Bearer <token>' \\      -F 'observee[unique_id]=UNIQUE_ID' \\      -F 'observee[password]=PASSWORD' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_destroy

> String user_observees_destroy(observee_id, user_id, root_account_id)
Remove an observee

Unregisters a user as being observed by the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**observee_id** | **String** | Scope response to observee_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**root_account_id** | Option<**i32**> | If specified, only removes the link for the given root account |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_index

> models::User user_observees_index(user_id, include, page, per_page)
List linked observees

A paginated list of users that the given user is observing. This endpoint returns users linked to the observer at the account level (such that the observer is automatically enrolled in observees' courses); it doesn't return one-off observer enrollments from individual courses.  *Note:* all users are allowed to list their own observees. Administrators can list other users' observees.  The returned observees will include an attribute \"observation_link_root_account_ids\", a list of ids for the root accounts the observer and observee are linked on. The observer will only be able to observe in courses associated with these root accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"] - \"avatar_url\": Optionally include avatar_url. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_observers

> models::User user_observees_observers(user_id, include, page, per_page)
List linked observers

A paginated list of observers linked to a given user.  *Note:* all users are allowed to list their own observers. Administrators can list other users' observers.  The returned observers will include an attribute \"observation_link_root_account_ids\", a list of ids for the root accounts the observer and observee are linked on. The observer will only be able to observe in courses associated with these root accounts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"] - \"avatar_url\": Optionally include avatar_url. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_show

> String user_observees_show(observee_id, user_id)
Show an observee

Gets information about an observed user.  *Note:* all users are allowed to view their own observees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**observee_id** | **String** | Scope response to observee_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_show_observer

> String user_observees_show_observer(observer_id, user_id)
Show an observer

Gets information about an observer.  *Note:* all users are allowed to view their own observers.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**observer_id** | **String** | Scope response to observer_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## user_observees_update

> String user_observees_update(observee_id, user_id, user_observees_update_request)
Add an observee

Registers a user as being observed by the given user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**observee_id** | **String** | Scope response to observee_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**user_observees_update_request** | Option<[**UserObserveesUpdateRequest**](UserObserveesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/<user_id>/observees/<observee_id> \\      -X PUT \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

