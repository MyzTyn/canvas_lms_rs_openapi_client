# \FeatureFlagsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**feature_flags_delete_for_accounts**](FeatureFlagsApi.md#feature_flags_delete_for_accounts) | **DELETE** /accounts/{account_id}/features/flags/{feature} | Remove feature flag
[**feature_flags_delete_for_courses**](FeatureFlagsApi.md#feature_flags_delete_for_courses) | **DELETE** /courses/{course_id}/features/flags/{feature} | Remove feature flag
[**feature_flags_delete_for_users**](FeatureFlagsApi.md#feature_flags_delete_for_users) | **DELETE** /users/{user_id}/features/flags/{feature} | Remove feature flag
[**feature_flags_enabled_features_for_accounts**](FeatureFlagsApi.md#feature_flags_enabled_features_for_accounts) | **GET** /accounts/{account_id}/features/enabled | List enabled features
[**feature_flags_enabled_features_for_courses**](FeatureFlagsApi.md#feature_flags_enabled_features_for_courses) | **GET** /courses/{course_id}/features/enabled | List enabled features
[**feature_flags_enabled_features_for_users**](FeatureFlagsApi.md#feature_flags_enabled_features_for_users) | **GET** /users/{user_id}/features/enabled | List enabled features
[**feature_flags_environment**](FeatureFlagsApi.md#feature_flags_environment) | **GET** /features/environment | List environment features
[**feature_flags_index_for_accounts**](FeatureFlagsApi.md#feature_flags_index_for_accounts) | **GET** /accounts/{account_id}/features | List features
[**feature_flags_index_for_courses**](FeatureFlagsApi.md#feature_flags_index_for_courses) | **GET** /courses/{course_id}/features | List features
[**feature_flags_index_for_users**](FeatureFlagsApi.md#feature_flags_index_for_users) | **GET** /users/{user_id}/features | List features
[**feature_flags_show_for_accounts**](FeatureFlagsApi.md#feature_flags_show_for_accounts) | **GET** /accounts/{account_id}/features/flags/{feature} | Get feature flag
[**feature_flags_show_for_courses**](FeatureFlagsApi.md#feature_flags_show_for_courses) | **GET** /courses/{course_id}/features/flags/{feature} | Get feature flag
[**feature_flags_show_for_users**](FeatureFlagsApi.md#feature_flags_show_for_users) | **GET** /users/{user_id}/features/flags/{feature} | Get feature flag
[**feature_flags_update_for_accounts**](FeatureFlagsApi.md#feature_flags_update_for_accounts) | **PUT** /accounts/{account_id}/features/flags/{feature} | Set feature flag
[**feature_flags_update_for_courses**](FeatureFlagsApi.md#feature_flags_update_for_courses) | **PUT** /courses/{course_id}/features/flags/{feature} | Set feature flag
[**feature_flags_update_for_users**](FeatureFlagsApi.md#feature_flags_update_for_users) | **PUT** /users/{user_id}/features/flags/{feature} | Set feature flag



## feature_flags_delete_for_accounts

> String feature_flags_delete_for_accounts(account_id, feature)
Remove feature flag

Remove feature flag for a given Account, Course, or User.  (Note that the flag must be defined on the Account, Course, or User directly.)  The object will then inherit the feature flags from a higher account, if any exist.  If this flag was 'on' or 'off', then lower-level account flags that were masked by this one will apply again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_delete_for_courses

> String feature_flags_delete_for_courses(course_id, feature)
Remove feature flag

Remove feature flag for a given Account, Course, or User.  (Note that the flag must be defined on the Account, Course, or User directly.)  The object will then inherit the feature flags from a higher account, if any exist.  If this flag was 'on' or 'off', then lower-level account flags that were masked by this one will apply again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_delete_for_users

> String feature_flags_delete_for_users(user_id, feature)
Remove feature flag

Remove feature flag for a given Account, Course, or User.  (Note that the flag must be defined on the Account, Course, or User directly.)  The object will then inherit the feature flags from a higher account, if any exist.  If this flag was 'on' or 'off', then lower-level account flags that were masked by this one will apply again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_enabled_features_for_accounts

> feature_flags_enabled_features_for_accounts(account_id)
List enabled features

A paginated list of all features that are enabled on a given Account, Course, or User. Only the feature names are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_enabled_features_for_courses

> feature_flags_enabled_features_for_courses(course_id)
List enabled features

A paginated list of all features that are enabled on a given Account, Course, or User. Only the feature names are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_enabled_features_for_users

> feature_flags_enabled_features_for_users(user_id)
List enabled features

A paginated list of all features that are enabled on a given Account, Course, or User. Only the feature names are returned.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_environment

> feature_flags_environment()
List environment features

Return a hash of global feature options that pertain to the Canvas user interface. This is the same information supplied to the web interface as +ENV.FEATURES+.

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


## feature_flags_index_for_accounts

> serde_json::Value feature_flags_index_for_accounts(account_id, hide_inherited_enabled, page, per_page)
List features

A paginated list of all features that apply to a given Account, Course, or User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**hide_inherited_enabled** | Option<**bool**> | When true, feature flags that are enabled in a higher context and cannot be overridden will be omitted. |  |
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


## feature_flags_index_for_courses

> serde_json::Value feature_flags_index_for_courses(course_id, hide_inherited_enabled, page, per_page)
List features

A paginated list of all features that apply to a given Account, Course, or User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**hide_inherited_enabled** | Option<**bool**> | When true, feature flags that are enabled in a higher context and cannot be overridden will be omitted. |  |
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


## feature_flags_index_for_users

> serde_json::Value feature_flags_index_for_users(user_id, hide_inherited_enabled, page, per_page)
List features

A paginated list of all features that apply to a given Account, Course, or User.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**hide_inherited_enabled** | Option<**bool**> | When true, feature flags that are enabled in a higher context and cannot be overridden will be omitted. |  |
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


## feature_flags_show_for_accounts

> String feature_flags_show_for_accounts(account_id, feature)
Get feature flag

Get the feature flag that applies to a given Account, Course, or User. The flag may be defined on the object, or it may be inherited from a parent account. You can look at the context_id and context_type of the returned object to determine which is the case. If these fields are missing, then the object is the global Canvas default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_show_for_courses

> String feature_flags_show_for_courses(course_id, feature)
Get feature flag

Get the feature flag that applies to a given Account, Course, or User. The flag may be defined on the object, or it may be inherited from a parent account. You can look at the context_id and context_type of the returned object to determine which is the case. If these fields are missing, then the object is the global Canvas default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_show_for_users

> String feature_flags_show_for_users(user_id, feature)
Get feature flag

Get the feature flag that applies to a given Account, Course, or User. The flag may be defined on the object, or it may be inherited from a parent account. You can look at the context_id and context_type of the returned object to determine which is the case. If these fields are missing, then the object is the global Canvas default.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**feature** | **String** | Scope response to feature | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_update_for_accounts

> String feature_flags_update_for_accounts(account_id, feature, feature_flags_update_for_courses_request)
Set feature flag

Set a feature flag for a given Account, Course, or User. This call will fail if a parent account sets a feature flag for the same feature in any state other than \"allowed\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**feature** | **String** | Scope response to feature | [required] |
**feature_flags_update_for_courses_request** | Option<[**FeatureFlagsUpdateForCoursesRequest**](FeatureFlagsUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_update_for_courses

> String feature_flags_update_for_courses(course_id, feature, feature_flags_update_for_courses_request)
Set feature flag

Set a feature flag for a given Account, Course, or User. This call will fail if a parent account sets a feature flag for the same feature in any state other than \"allowed\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**feature** | **String** | Scope response to feature | [required] |
**feature_flags_update_for_courses_request** | Option<[**FeatureFlagsUpdateForCoursesRequest**](FeatureFlagsUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## feature_flags_update_for_users

> String feature_flags_update_for_users(user_id, feature, feature_flags_update_for_courses_request)
Set feature flag

Set a feature flag for a given Account, Course, or User. This call will fail if a parent account sets a feature flag for the same feature in any state other than \"allowed\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**feature** | **String** | Scope response to feature | [required] |
**feature_flags_update_for_courses_request** | Option<[**FeatureFlagsUpdateForCoursesRequest**](FeatureFlagsUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

