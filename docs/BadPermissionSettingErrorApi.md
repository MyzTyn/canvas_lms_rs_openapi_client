# \BadPermissionSettingErrorApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bad_permission_setting_error_activate_role**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_activate_role) | **POST** /accounts/{account_id}/roles/{id}/activate | Activate a role
[**bad_permission_setting_error_add_role**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_add_role) | **POST** /accounts/{account_id}/roles | Create a new role
[**bad_permission_setting_error_api_index**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_api_index) | **GET** /accounts/{account_id}/roles | List roles
[**bad_permission_setting_error_manageable_permissions**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_manageable_permissions) | **GET** /accounts/{account_id}/roles/permissions | List assignable permissions
[**bad_permission_setting_error_remove_role**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_remove_role) | **DELETE** /accounts/{account_id}/roles/{id} | Deactivate a role
[**bad_permission_setting_error_show**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_show) | **GET** /accounts/{account_id}/roles/{id} | Get a single role
[**bad_permission_setting_error_update**](BadPermissionSettingErrorApi.md#bad_permission_setting_error_update) | **PUT** /accounts/{account_id}/roles/{id} | Update a role



## bad_permission_setting_error_activate_role

> String bad_permission_setting_error_activate_role(account_id, id, bad_permission_setting_error_activate_role_request)
Activate a role

Re-activates an inactive role (allowing it to be assigned to new users)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**bad_permission_setting_error_activate_role_request** | Option<[**BadPermissionSettingErrorActivateRoleRequest**](BadPermissionSettingErrorActivateRoleRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bad_permission_setting_error_add_role

> String bad_permission_setting_error_add_role(account_id, bad_permission_setting_error_add_role_request)
Create a new role

Create a new course-level or account-level role.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**bad_permission_setting_error_add_role_request** | Option<[**BadPermissionSettingErrorAddRoleRequest**](BadPermissionSettingErrorAddRoleRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bad_permission_setting_error_api_index

> serde_json::Value bad_permission_setting_error_api_index(account_id, state, show_inherited, page, per_page)
List roles

A paginated list of the roles available to an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The id of the account to retrieve roles for. | [required] |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"inactive\"] Filter by role state. If this argument is omitted, only 'active' roles are returned. |  |
**show_inherited** | Option<**bool**> | If this argument is true, all roles inherited from parent accounts will be included. |  |
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


## bad_permission_setting_error_manageable_permissions

> serde_json::Value bad_permission_setting_error_manageable_permissions(account_id, search_term)
List assignable permissions

List all permissions that can be granted to roles in the given account.  This returns largely the same information documented on the {file:file.permissions.html Permissions list page}, with a few caveats: * Permission labels and group labels returned by this API are localized (the same text visible in the web UI). * This API includes permissions added by plugins. * This API excludes permissions that are disabled in or otherwise do not apply to the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**search_term** | Option<**String**> | If provided, return only permissions whose key, label, group, or group_label match the search string. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bad_permission_setting_error_remove_role

> String bad_permission_setting_error_remove_role(account_id, id, role_id, role)
Deactivate a role

Deactivates a custom role.  This hides it in the user interface and prevents it from being assigned to new users.  Existing users assigned to the role will continue to function with the same permissions they had previously. Built-in roles cannot be deactivated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**role_id** | **i32** | The unique identifier for the role | [required] |
**role** | Option<**String**> | The name for the role |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bad_permission_setting_error_show

> String bad_permission_setting_error_show(account_id, id, role_id, role)
Get a single role

Retrieve information about a single role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The id of the account containing the role | [required] |
**id** | **String** | Scope response to id | [required] |
**role_id** | **i32** | The unique identifier for the role | [required] |
**role** | Option<**String**> | The name for the role |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## bad_permission_setting_error_update

> String bad_permission_setting_error_update(account_id, id, bad_permission_setting_error_update_request)
Update a role

Update permissions for an existing role.  Recognized roles are: * TeacherEnrollment * StudentEnrollment * TaEnrollment * ObserverEnrollment * DesignerEnrollment * AccountAdmin * Any previously created custom role

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**bad_permission_setting_error_update_request** | Option<[**BadPermissionSettingErrorUpdateRequest**](BadPermissionSettingErrorUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

