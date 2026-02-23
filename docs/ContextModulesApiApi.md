# \ContextModulesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_modules_api_create**](ContextModulesApiApi.md#context_modules_api_create) | **POST** /courses/{course_id}/modules | Create a module
[**context_modules_api_destroy**](ContextModulesApiApi.md#context_modules_api_destroy) | **DELETE** /courses/{course_id}/modules/{id} | Delete module
[**context_modules_api_index**](ContextModulesApiApi.md#context_modules_api_index) | **GET** /courses/{course_id}/modules | List modules
[**context_modules_api_relock**](ContextModulesApiApi.md#context_modules_api_relock) | **PUT** /courses/{course_id}/modules/{id}/relock | Re-lock module progressions
[**context_modules_api_show**](ContextModulesApiApi.md#context_modules_api_show) | **GET** /courses/{course_id}/modules/{id} | Show module
[**context_modules_api_update**](ContextModulesApiApi.md#context_modules_api_update) | **PUT** /courses/{course_id}/modules/{id} | Update a module



## context_modules_api_create

> String context_modules_api_create(course_id, context_modules_api_create_request)
Create a module

Create and return a new module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**context_modules_api_create_request** | Option<[**ContextModulesApiCreateRequest**](ContextModulesApiCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/modules \\   -X POST \\   -H 'Authorization: Bearer <token>' \\   -d 'module[name]=module' \\   -d 'module[position]=2' \\   -d 'module[prerequisite_module_ids][]=121' \\   -d 'module[prerequisite_module_ids][]=122' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_modules_api_destroy

> String context_modules_api_destroy(course_id, id)
Delete module

Delete a module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_modules_api_index

> models::Module context_modules_api_index(course_id, include, search_term, student_id, page, per_page)
List modules

A paginated list of the modules in a course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"items\"|\"content_details\"] - \"items\": Return module items inline if possible.   This parameter suggests that Canvas return module items directly   in the Module object JSON, to avoid having to make separate API   requests for each module when enumerating modules and items. Canvas   is free to omit 'items' for any particular module if it deems them   too numerous to return inline. Callers must be prepared to use the   {api:ContextModuleItemsApiController#index List Module Items API}   if items are not returned. - \"content_details\": Requires 'items'. Returns additional   details with module items specific to their associated content items.   Includes standard lock information for each item. |  |
**search_term** | Option<**String**> | The partial name of the modules (and module items, if 'items' is specified with include[]) to match and return. |  |
**student_id** | Option<**String**> | Returns module completion information for the student with this id. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Module**](Module.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_modules_api_relock

> String context_modules_api_relock(course_id, id)
Re-lock module progressions

Resets module progressions to their default locked state and recalculates them based on the current requirements.  Adding progression requirements to an active course will not lock students out of modules they have already unlocked unless this action is called.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_modules_api_show

> String context_modules_api_show(course_id, id, include, student_id)
Show module

Get information about a single module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"items\"|\"content_details\"] - \"items\": Return module items inline if possible.   This parameter suggests that Canvas return module items directly   in the Module object JSON, to avoid having to make separate API   requests for each module when enumerating modules and items. Canvas   is free to omit 'items' for any particular module if it deems them   too numerous to return inline. Callers must be prepared to use the   {api:ContextModuleItemsApiController#index List Module Items API}   if items are not returned. - \"content_details\": Requires 'items'. Returns additional   details with module items specific to their associated content items.   Includes standard lock information for each item. |  |
**student_id** | Option<**String**> | Returns module completion information for the student with this id. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_modules_api_update

> String context_modules_api_update(course_id, id, context_modules_api_update_request)
Update a module

Update and return an existing module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**context_modules_api_update_request** | Option<[**ContextModulesApiUpdateRequest**](ContextModulesApiUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/modules/<module_id> \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'module[name]=module' \\   -d 'module[position]=2' \\   -d 'module[prerequisite_module_ids][]=121' \\   -d 'module[prerequisite_module_ids][]=122' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

