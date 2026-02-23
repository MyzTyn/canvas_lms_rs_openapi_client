# \ContextModuleItemsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_module_items_api_create**](ContextModuleItemsApiApi.md#context_module_items_api_create) | **POST** /courses/{course_id}/modules/{module_id}/items | Create a module item
[**context_module_items_api_destroy**](ContextModuleItemsApiApi.md#context_module_items_api_destroy) | **DELETE** /courses/{course_id}/modules/{module_id}/items/{id} | Delete module item
[**context_module_items_api_index**](ContextModuleItemsApiApi.md#context_module_items_api_index) | **GET** /courses/{course_id}/modules/{module_id}/items | List module items
[**context_module_items_api_item_sequence**](ContextModuleItemsApiApi.md#context_module_items_api_item_sequence) | **GET** /courses/{course_id}/module_item_sequence | Get module item sequence
[**context_module_items_api_mark_as_done**](ContextModuleItemsApiApi.md#context_module_items_api_mark_as_done) | **PUT** /courses/{course_id}/modules/{module_id}/items/{id}/done | Mark module item as done/not done
[**context_module_items_api_mark_item_read**](ContextModuleItemsApiApi.md#context_module_items_api_mark_item_read) | **POST** /courses/{course_id}/modules/{module_id}/items/{id}/mark_read | Mark module item read
[**context_module_items_api_select_mastery_path**](ContextModuleItemsApiApi.md#context_module_items_api_select_mastery_path) | **POST** /courses/{course_id}/modules/{module_id}/items/{id}/select_mastery_path | Select a mastery path
[**context_module_items_api_show**](ContextModuleItemsApiApi.md#context_module_items_api_show) | **GET** /courses/{course_id}/modules/{module_id}/items/{id} | Show module item
[**context_module_items_api_update**](ContextModuleItemsApiApi.md#context_module_items_api_update) | **PUT** /courses/{course_id}/modules/{module_id}/items/{id} | Update a module item



## context_module_items_api_create

> String context_module_items_api_create(course_id, module_id, context_module_items_api_create_request)
Create a module item

Create and return a new module item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |
**context_module_items_api_create_request** | Option<[**ContextModuleItemsApiCreateRequest**](ContextModuleItemsApiCreateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/modules/<module_id>/items \\   -X POST \\   -H 'Authorization: Bearer <token>' \\   -d 'module_item[title]=module item' \\   -d 'module_item[type]=ExternalTool' \\   -d 'module_item[content_id]=10' \\   -d 'module_item[position]=2' \\   -d 'module_item[indent]=1' \\   -d 'module_item[new_tab]=true' \\   -d 'module_item[iframe][width]=300' \\   -d 'module_item[iframe][height]=200' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_destroy

> String context_module_items_api_destroy(course_id, id, module_id)
Delete module item

Delete a module item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_index

> models::ModuleItem context_module_items_api_index(course_id, module_id, include, search_term, student_id, page, per_page)
List module items

A paginated list of the items in a module

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"content_details\"] If included, will return additional details specific to the content associated with each item. Refer to the {api:Modules:Module%20Item Module Item specification} for more details. Includes standard lock information for each item. |  |
**search_term** | Option<**String**> | The partial title of the items to match and return. |  |
**student_id** | Option<**String**> | Returns module completion information for the student with this id. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::ModuleItem**](ModuleItem.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_item_sequence

> String context_module_items_api_item_sequence(course_id, asset_type, asset_id)
Get module item sequence

Given an asset in a course, find the ModuleItem it belongs to, the previous and next Module Items in the course sequence, and also any applicable mastery path rules

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**asset_type** | Option<**String**> | The type of asset to find module sequence information for. Use the ModuleItem if it is known (e.g., the user navigated from a module item), since this will avoid ambiguity if the asset appears more than once in the module sequence. |  |
**asset_id** | Option<**i32**> | The id of the asset (or the url in the case of a Page) |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_mark_as_done

> context_module_items_api_mark_as_done(course_id, id, module_id)
Mark module item as done/not done

Mark a module item as done/not done. Use HTTP method PUT to mark as done, and DELETE to mark as not done.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_mark_item_read

> context_module_items_api_mark_item_read(course_id, id, module_id)
Mark module item read

Fulfills \"must view\" requirement for a module item. It is generally not necessary to do this explicitly, but it is provided for applications that need to access external content directly (bypassing the html_url redirect that normally allows Canvas to fulfill \"must view\" requirements).  This endpoint cannot be used to complete requirements on locked or unpublished module items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_select_mastery_path

> context_module_items_api_select_mastery_path(course_id, id, module_id, context_module_items_api_select_mastery_path_request)
Select a mastery path

Select a mastery path when module item includes several possible paths. Requires Mastery Paths feature to be enabled.  Returns a compound document with the assignments included in the given path and any module items related to those assignments

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |
**context_module_items_api_select_mastery_path_request** | Option<[**ContextModuleItemsApiSelectMasteryPathRequest**](ContextModuleItemsApiSelectMasteryPathRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/modules/<module_id>/items/<item_id>/select_master_path \\   -X POST \\   -H 'Authorization: Bearer <token>' \\   -d 'assignment_set_id=2992' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_show

> String context_module_items_api_show(course_id, id, module_id, include, student_id)
Show module item

Get information about a single module item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"content_details\"] If included, will return additional details specific to the content associated with this item. Refer to the {api:Modules:Module%20Item Module Item specification} for more details. Includes standard lock information for each item. |  |
**student_id** | Option<**String**> | Returns module completion information for the student with this id. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_module_items_api_update

> String context_module_items_api_update(course_id, id, module_id, context_module_items_api_update_request)
Update a module item

Update and return an existing module item

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**module_id** | **String** | Scope response to module_id | [required] |
**context_module_items_api_update_request** | Option<[**ContextModuleItemsApiUpdateRequest**](ContextModuleItemsApiUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/modules/<module_id>/items/<item_id> \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'module_item[position]=2' \\   -d 'module_item[indent]=1' \\   -d 'module_item[new_tab]=true' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

