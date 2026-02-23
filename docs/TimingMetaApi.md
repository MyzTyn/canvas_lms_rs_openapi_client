# \TimingMetaApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**timing_meta_add_top_nav_favorite**](TimingMetaApi.md#timing_meta_add_top_nav_favorite) | **POST** /accounts/{account_id}/external_tools/top_nav_favorites/{id} | Add tool to Top Navigation Favorites
[**timing_meta_all_visible_nav_tools**](TimingMetaApi.md#timing_meta_all_visible_nav_tools) | **GET** /external_tools/visible_course_nav_tools | Get visible course navigation tools
[**timing_meta_create_for_accounts**](TimingMetaApi.md#timing_meta_create_for_accounts) | **POST** /accounts/{account_id}/external_tools | Create an external tool
[**timing_meta_create_for_courses**](TimingMetaApi.md#timing_meta_create_for_courses) | **POST** /courses/{course_id}/external_tools | Create an external tool
[**timing_meta_destroy_for_accounts**](TimingMetaApi.md#timing_meta_destroy_for_accounts) | **DELETE** /accounts/{account_id}/external_tools/{external_tool_id} | Delete an external tool
[**timing_meta_destroy_for_courses**](TimingMetaApi.md#timing_meta_destroy_for_courses) | **DELETE** /courses/{course_id}/external_tools/{external_tool_id} | Delete an external tool
[**timing_meta_generate_sessionless_launch_for_accounts**](TimingMetaApi.md#timing_meta_generate_sessionless_launch_for_accounts) | **GET** /accounts/{account_id}/external_tools/sessionless_launch | Get a sessionless launch url for an external tool.
[**timing_meta_generate_sessionless_launch_for_courses**](TimingMetaApi.md#timing_meta_generate_sessionless_launch_for_courses) | **GET** /courses/{course_id}/external_tools/sessionless_launch | Get a sessionless launch url for an external tool.
[**timing_meta_index_for_accounts**](TimingMetaApi.md#timing_meta_index_for_accounts) | **GET** /accounts/{account_id}/external_tools | List external tools
[**timing_meta_index_for_courses**](TimingMetaApi.md#timing_meta_index_for_courses) | **GET** /courses/{course_id}/external_tools | List external tools
[**timing_meta_index_for_groups**](TimingMetaApi.md#timing_meta_index_for_groups) | **GET** /groups/{group_id}/external_tools | List external tools
[**timing_meta_mark_rce_favorite**](TimingMetaApi.md#timing_meta_mark_rce_favorite) | **POST** /accounts/{account_id}/external_tools/rce_favorites/{id} | Mark tool as RCE Favorite
[**timing_meta_remove_top_nav_favorite**](TimingMetaApi.md#timing_meta_remove_top_nav_favorite) | **DELETE** /accounts/{account_id}/external_tools/top_nav_favorites/{id} | Remove tool from Top Navigation Favorites
[**timing_meta_show_for_accounts**](TimingMetaApi.md#timing_meta_show_for_accounts) | **GET** /accounts/{account_id}/external_tools/{external_tool_id} | Get a single external tool
[**timing_meta_show_for_courses**](TimingMetaApi.md#timing_meta_show_for_courses) | **GET** /courses/{course_id}/external_tools/{external_tool_id} | Get a single external tool
[**timing_meta_unmark_rce_favorite**](TimingMetaApi.md#timing_meta_unmark_rce_favorite) | **DELETE** /accounts/{account_id}/external_tools/rce_favorites/{id} | Unmark tool as RCE Favorite
[**timing_meta_update_for_accounts**](TimingMetaApi.md#timing_meta_update_for_accounts) | **PUT** /accounts/{account_id}/external_tools/{external_tool_id} | Edit an external tool
[**timing_meta_update_for_courses**](TimingMetaApi.md#timing_meta_update_for_courses) | **PUT** /courses/{course_id}/external_tools/{external_tool_id} | Edit an external tool
[**timing_meta_visible_course_nav_tools**](TimingMetaApi.md#timing_meta_visible_course_nav_tools) | **GET** /courses/{course_id}/external_tools/visible_course_nav_tools | Get visible course navigation tools for a single course



## timing_meta_add_top_nav_favorite

> timing_meta_add_top_nav_favorite(account_id, id)
Add tool to Top Navigation Favorites

Adds a dedicated button in Top Navigation for the specified tool for the given account. Cannot set more than 2 top_navigation Favorites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_all_visible_nav_tools

> timing_meta_all_visible_nav_tools(context_codes)
Get visible course navigation tools

Get a list of external tools with the course_navigation placement that have not been hidden in course settings and whose visibility settings apply to the requesting user. These tools are the same that appear in the course navigation.  The response format is the same as for List external tools, but with additional context_id and context_name fields on each element in the array.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_codes** | [**Vec<String>**](String.md) | [Required] List of context_codes to retrieve visible course nav tools for (for example, +course_123+). Only courses are presently supported. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_create_for_accounts

> String timing_meta_create_for_accounts(account_id, timing_meta_create_for_courses_request)
Create an external tool

Create an external tool in the specified course/account. The created tool will be returned, see the \"show\" endpoint for an example. If a client ID is supplied canvas will attempt to create a context external tool using the LTI 1.3 standard.  See the <a href=\"file.lti_dev_key_config.html#placements-params\">Placements Documentation</a> for more information on what placements are available, the possible fields, and their accepted values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**timing_meta_create_for_courses_request** | Option<[**TimingMetaCreateForCoursesRequest**](TimingMetaCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_create_for_courses

> String timing_meta_create_for_courses(course_id, timing_meta_create_for_courses_request)
Create an external tool

Create an external tool in the specified course/account. The created tool will be returned, see the \"show\" endpoint for an example. If a client ID is supplied canvas will attempt to create a context external tool using the LTI 1.3 standard.  See the <a href=\"file.lti_dev_key_config.html#placements-params\">Placements Documentation</a> for more information on what placements are available, the possible fields, and their accepted values.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**timing_meta_create_for_courses_request** | Option<[**TimingMetaCreateForCoursesRequest**](TimingMetaCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_destroy_for_accounts

> String timing_meta_destroy_for_accounts(account_id, external_tool_id)
Delete an external tool

Remove the specified external tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_destroy_for_courses

> String timing_meta_destroy_for_courses(course_id, external_tool_id)
Delete an external tool

Remove the specified external tool

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_generate_sessionless_launch_for_accounts

> timing_meta_generate_sessionless_launch_for_accounts(account_id, assignment_id, module_item_id, id, url, launch_type, resource_link_lookup_uuid)
Get a sessionless launch url for an external tool.

Returns a sessionless launch url for an external tool. Prefers the resource_link_lookup_uuid, but defaults to the other passed   parameters id, url, and launch_type  NOTE: Either the resource_link_lookup_uuid, id, or url must be provided unless launch_type is assessment or module_item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**assignment_id** | **String** | The assignment id for an assignment launch. Required if launch_type is set to \"assessment\". | [required] |
**module_item_id** | **String** | The assignment id for a module item launch. Required if launch_type is set to \"module_item\". | [required] |
**id** | Option<**String**> | The external id of the tool to launch. |  |
**url** | Option<**String**> | The LTI launch url for the external tool. |  |
**launch_type** | Option<**String**> | The type of launch to perform on the external tool. Placement names (eg. \"course_navigation\") can also be specified to use the custom launch url for that placement; if done, the tool id must be provided. |  |
**resource_link_lookup_uuid** | Option<**String**> | The identifier to lookup a resource link. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_generate_sessionless_launch_for_courses

> timing_meta_generate_sessionless_launch_for_courses(course_id, assignment_id, module_item_id, id, url, launch_type, resource_link_lookup_uuid)
Get a sessionless launch url for an external tool.

Returns a sessionless launch url for an external tool. Prefers the resource_link_lookup_uuid, but defaults to the other passed   parameters id, url, and launch_type  NOTE: Either the resource_link_lookup_uuid, id, or url must be provided unless launch_type is assessment or module_item.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_id** | **String** | The assignment id for an assignment launch. Required if launch_type is set to \"assessment\". | [required] |
**module_item_id** | **String** | The assignment id for a module item launch. Required if launch_type is set to \"module_item\". | [required] |
**id** | Option<**String**> | The external id of the tool to launch. |  |
**url** | Option<**String**> | The LTI launch url for the external tool. |  |
**launch_type** | Option<**String**> | The type of launch to perform on the external tool. Placement names (eg. \"course_navigation\") can also be specified to use the custom launch url for that placement; if done, the tool id must be provided. |  |
**resource_link_lookup_uuid** | Option<**String**> | The identifier to lookup a resource link. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_index_for_accounts

> serde_json::Value timing_meta_index_for_accounts(account_id, search_term, selectable, include_parents, placement, page, per_page)
List external tools

Returns the paginated list of external tools for the current context. See the get request docs for a single tool for a list of properties on an external tool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**search_term** | Option<**String**> | The partial name of the tools to match and return. |  |
**selectable** | Option<**bool**> | If true, then only tools that are meant to be selectable are returned. |  |
**include_parents** | Option<**bool**> | If true, then include tools installed in all accounts above the current context |  |
**placement** | Option<**String**> | The placement type to filter by.  Return all tools at the current context as well as all tools from the parent, and filter the tools list to only those with a placement of 'editor_button' |  |
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


## timing_meta_index_for_courses

> serde_json::Value timing_meta_index_for_courses(course_id, search_term, selectable, include_parents, placement, page, per_page)
List external tools

Returns the paginated list of external tools for the current context. See the get request docs for a single tool for a list of properties on an external tool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**search_term** | Option<**String**> | The partial name of the tools to match and return. |  |
**selectable** | Option<**bool**> | If true, then only tools that are meant to be selectable are returned. |  |
**include_parents** | Option<**bool**> | If true, then include tools installed in all accounts above the current context |  |
**placement** | Option<**String**> | The placement type to filter by.  Return all tools at the current context as well as all tools from the parent, and filter the tools list to only those with a placement of 'editor_button' |  |
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


## timing_meta_index_for_groups

> serde_json::Value timing_meta_index_for_groups(group_id, search_term, selectable, include_parents, placement, page, per_page)
List external tools

Returns the paginated list of external tools for the current context. See the get request docs for a single tool for a list of properties on an external tool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**search_term** | Option<**String**> | The partial name of the tools to match and return. |  |
**selectable** | Option<**bool**> | If true, then only tools that are meant to be selectable are returned. |  |
**include_parents** | Option<**bool**> | If true, then include tools installed in all accounts above the current context |  |
**placement** | Option<**String**> | The placement type to filter by.  Return all tools at the current context as well as all tools from the parent, and filter the tools list to only those with a placement of 'editor_button' |  |
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


## timing_meta_mark_rce_favorite

> timing_meta_mark_rce_favorite(account_id, id)
Mark tool as RCE Favorite

Mark the specified editor_button external tool as a favorite in the RCE editor for courses in the given account and its subaccounts (if the subaccounts haven't set their own RCE Favorites). This places the tool in a preferred location in the RCE. Cannot mark more than 2 tools as RCE Favorites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_remove_top_nav_favorite

> timing_meta_remove_top_nav_favorite(account_id, id)
Remove tool from Top Navigation Favorites

Removes the dedicated button in Top Navigation for the specified tool for the given account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_show_for_accounts

> String timing_meta_show_for_accounts(account_id, external_tool_id)
Get a single external tool

Returns the specified external tool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_show_for_courses

> String timing_meta_show_for_courses(course_id, external_tool_id)
Get a single external tool

Returns the specified external tool.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_unmark_rce_favorite

> timing_meta_unmark_rce_favorite(account_id, id)
Unmark tool as RCE Favorite

Unmark the specified external tool as a favorite in the RCE editor for the given account. The tool will remain available but will no longer appear in the preferred favorites location.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_update_for_accounts

> String timing_meta_update_for_accounts(account_id, external_tool_id)
Edit an external tool

Update the specified external tool. Uses same parameters as create. Returns the updated tool.  NOTE: Any updates made to LTI 1.3 tools with this API will be overridden if any changes are made to the tool's associated LTI Registration/Developer Key configuration. In almost all cases, changes should be made to the tool's associated LTI Registration configuration, not individual tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_update_for_courses

> String timing_meta_update_for_courses(course_id, external_tool_id)
Edit an external tool

Update the specified external tool. Uses same parameters as create. Returns the updated tool.  NOTE: Any updates made to LTI 1.3 tools with this API will be overridden if any changes are made to the tool's associated LTI Registration/Developer Key configuration. In almost all cases, changes should be made to the tool's associated LTI Registration configuration, not individual tools.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**external_tool_id** | **String** | Scope response to external_tool_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## timing_meta_visible_course_nav_tools

> timing_meta_visible_course_nav_tools(course_id)
Get visible course navigation tools for a single course

Get a list of external tools with the course_navigation placement that have not been hidden in course settings and whose visibility settings apply to the requesting user. These tools are the same that appear in the course navigation.  The response format is the same as Get visible course navigation tools.

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

