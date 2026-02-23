# \NotificationPreferencesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**notification_preferences_category_index**](NotificationPreferencesApi.md#notification_preferences_category_index) | **GET** /users/{user_id}/communication_channels/{communication_channel_id}/notification_preference_categories | List of preference categories
[**notification_preferences_index_for_users**](NotificationPreferencesApi.md#notification_preferences_index_for_users) | **GET** /users/{user_id}/communication_channels/{communication_channel_id}/notification_preferences | List preferences
[**notification_preferences_index_for_users2**](NotificationPreferencesApi.md#notification_preferences_index_for_users2) | **GET** /users/{user_id}/communication_channels/{type}/{address}/notification_preferences | List preferences
[**notification_preferences_show_for_users**](NotificationPreferencesApi.md#notification_preferences_show_for_users) | **GET** /users/{user_id}/communication_channels/{communication_channel_id}/notification_preferences/{notification} | Get a preference
[**notification_preferences_show_for_users2**](NotificationPreferencesApi.md#notification_preferences_show_for_users2) | **GET** /users/{user_id}/communication_channels/{type}/{address}/notification_preferences/{notification} | Get a preference
[**notification_preferences_update_all_for_users**](NotificationPreferencesApi.md#notification_preferences_update_all_for_users) | **PUT** /users/self/communication_channels/{communication_channel_id}/notification_preferences | Update multiple preferences
[**notification_preferences_update_all_for_users2**](NotificationPreferencesApi.md#notification_preferences_update_all_for_users2) | **PUT** /users/self/communication_channels/{type}/{address}/notification_preferences | Update multiple preferences
[**notification_preferences_update_for_users**](NotificationPreferencesApi.md#notification_preferences_update_for_users) | **PUT** /users/self/communication_channels/{communication_channel_id}/notification_preferences/{notification} | Update a preference
[**notification_preferences_update_for_users2**](NotificationPreferencesApi.md#notification_preferences_update_for_users2) | **PUT** /users/self/communication_channels/{type}/{address}/notification_preferences/{notification} | Update a preference
[**notification_preferences_update_preferences_by_category**](NotificationPreferencesApi.md#notification_preferences_update_preferences_by_category) | **PUT** /users/self/communication_channels/{communication_channel_id}/notification_preference_categories/{category} | Update preferences by category



## notification_preferences_category_index

> notification_preferences_category_index(communication_channel_id, user_id)
List of preference categories

Fetch all notification preference categories for the given communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_index_for_users

> serde_json::Value notification_preferences_index_for_users(communication_channel_id, user_id)
List preferences

Fetch all preferences for the given communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_index_for_users2

> serde_json::Value notification_preferences_index_for_users2(address, r#type, user_id)
List preferences

Fetch all preferences for the given communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | ID of the address | [required] |
**r#type** | **String** | ID of the type | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_show_for_users

> String notification_preferences_show_for_users(communication_channel_id, notification, user_id)
Get a preference

Fetch the preference for the given notification for the given communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**notification** | **String** | Scope response to notification | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_show_for_users2

> String notification_preferences_show_for_users2(address, r#type, notification, user_id)
Get a preference

Fetch the preference for the given notification for the given communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | ID of the address | [required] |
**r#type** | **String** | ID of the type | [required] |
**notification** | **String** | Scope response to notification | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_update_all_for_users

> notification_preferences_update_all_for_users(communication_channel_id, notification_preferences_update_all_for_users_request)
Update multiple preferences

Change the preferences for multiple notifications for a single communication channel at once

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**notification_preferences_update_all_for_users_request** | Option<[**NotificationPreferencesUpdateAllForUsersRequest**](NotificationPreferencesUpdateAllForUsersRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_update_all_for_users2

> notification_preferences_update_all_for_users2(address, r#type, notification_preferences_update_all_for_users_request)
Update multiple preferences

Change the preferences for multiple notifications for a single communication channel at once

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | ID of the address | [required] |
**r#type** | **String** | ID of the type | [required] |
**notification_preferences_update_all_for_users_request** | Option<[**NotificationPreferencesUpdateAllForUsersRequest**](NotificationPreferencesUpdateAllForUsersRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_update_for_users

> notification_preferences_update_for_users(communication_channel_id, notification, notification_preferences_update_for_users_request)
Update a preference

Change the preference for a single notification for a single communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**notification** | **String** | Scope response to notification | [required] |
**notification_preferences_update_for_users_request** | Option<[**NotificationPreferencesUpdateForUsersRequest**](NotificationPreferencesUpdateForUsersRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_update_for_users2

> notification_preferences_update_for_users2(address, r#type, notification, notification_preferences_update_for_users_request)
Update a preference

Change the preference for a single notification for a single communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**address** | **String** | ID of the address | [required] |
**r#type** | **String** | ID of the type | [required] |
**notification** | **String** | Scope response to notification | [required] |
**notification_preferences_update_for_users_request** | Option<[**NotificationPreferencesUpdateForUsersRequest**](NotificationPreferencesUpdateForUsersRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## notification_preferences_update_preferences_by_category

> notification_preferences_update_preferences_by_category(category, communication_channel_id, notification_preferences_update_preferences_by_category_request)
Update preferences by category

Change the preferences for multiple notifications based on the category for a single communication channel

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**category** | **String** | The name of the category. Must be parameterized (e.g. The category \"Course Content\" should be \"course_content\") | [required] |
**communication_channel_id** | **String** | Scope response to communication_channel_id | [required] |
**notification_preferences_update_preferences_by_category_request** | Option<[**NotificationPreferencesUpdatePreferencesByCategoryRequest**](NotificationPreferencesUpdatePreferencesByCategoryRequest.md)> | Request body parameters |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

