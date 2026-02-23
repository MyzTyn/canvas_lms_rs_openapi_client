# \GroupCategoriesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**group_categories_assign_unassigned_members**](GroupCategoriesApi.md#group_categories_assign_unassigned_members) | **POST** /group_categories/{group_category_id}/assign_unassigned_members | Assign unassigned members
[**group_categories_bulk_manage_differentiation_tag**](GroupCategoriesApi.md#group_categories_bulk_manage_differentiation_tag) | **POST** /courses/{course_id}/group_categories/bulk_manage_differentiation_tag | Bulk manage differentiation tags
[**group_categories_create_for_accounts**](GroupCategoriesApi.md#group_categories_create_for_accounts) | **POST** /accounts/{account_id}/group_categories | Create a Group Category
[**group_categories_create_for_courses**](GroupCategoriesApi.md#group_categories_create_for_courses) | **POST** /courses/{course_id}/group_categories | Create a Group Category
[**group_categories_destroy**](GroupCategoriesApi.md#group_categories_destroy) | **DELETE** /group_categories/{group_category_id} | Delete a Group Category
[**group_categories_export**](GroupCategoriesApi.md#group_categories_export) | **GET** /group_categories/{group_category_id}/export | export groups in and users in category
[**group_categories_export_tags**](GroupCategoriesApi.md#group_categories_export_tags) | **GET** /courses/{course_id}/group_categories/export_tags | export tags and users in course
[**group_categories_groups**](GroupCategoriesApi.md#group_categories_groups) | **GET** /group_categories/{group_category_id}/groups | List groups in group category
[**group_categories_import**](GroupCategoriesApi.md#group_categories_import) | **POST** /group_categories/{group_category_id}/import | Import category groups
[**group_categories_import_tags**](GroupCategoriesApi.md#group_categories_import_tags) | **POST** /courses/{course_id}/group_categories/import_tags | Import differentiation tags
[**group_categories_index_for_accounts**](GroupCategoriesApi.md#group_categories_index_for_accounts) | **GET** /accounts/{account_id}/group_categories | List group categories for a context
[**group_categories_index_for_courses**](GroupCategoriesApi.md#group_categories_index_for_courses) | **GET** /courses/{course_id}/group_categories | List group categories for a context
[**group_categories_show**](GroupCategoriesApi.md#group_categories_show) | **GET** /group_categories/{group_category_id} | Get a single group category
[**group_categories_update**](GroupCategoriesApi.md#group_categories_update) | **PUT** /group_categories/{group_category_id} | Update a Group Category
[**group_categories_users**](GroupCategoriesApi.md#group_categories_users) | **GET** /group_categories/{group_category_id}/users | List users in group category



## group_categories_assign_unassigned_members

> String group_categories_assign_unassigned_members(group_category_id, group_categories_assign_unassigned_members_request)
Assign unassigned members

Assign all unassigned members as evenly as possible among the existing student groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |
**group_categories_assign_unassigned_members_request** | Option<[**GroupCategoriesAssignUnassignedMembersRequest**](GroupCategoriesAssignUnassignedMembersRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/group_categories/1/assign_unassigned_members \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_bulk_manage_differentiation_tag

> String group_categories_bulk_manage_differentiation_tag(course_id, group_categories_bulk_manage_differentiation_tag_request)
Bulk manage differentiation tags

This API is only meant for Groups and GroupCategories where non_collaborative is true.  Perform bulk operations on groups within a group category, or create a new group category along with the groups in one transaction. If creation of the GroupCategory or any Group fails, the entire operation will be rolled back.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**group_categories_bulk_manage_differentiation_tag_request** | Option<[**GroupCategoriesBulkManageDifferentiationTagRequest**](GroupCategoriesBulkManageDifferentiationTagRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/:course_id/group_categories/bulk_manage_differentiation_tag \\      -X POST \\      -H 'Authorization: Bearer <token>' \\      -H 'Content-Type: application/json' \\      -d '{            \"operations\": {              \"create\": [{\"name\": \"New Group\"}],              \"update\": [{\"id\": 123, \"name\": \"Updated Group\"}],              \"delete\": [{\"id\": 456}]            },            \"group_category\": {\"id\": 1, \"name\": \"New Category Name\"}          }' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_create_for_accounts

> String group_categories_create_for_accounts(account_id, group_categories_create_for_accounts_request)
Create a Group Category

Create a new group category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**group_categories_create_for_accounts_request** | Option<[**GroupCategoriesCreateForAccountsRequest**](GroupCategoriesCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_create_for_courses

> String group_categories_create_for_courses(course_id, group_categories_create_for_accounts_request)
Create a Group Category

Create a new group category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**group_categories_create_for_accounts_request** | Option<[**GroupCategoriesCreateForAccountsRequest**](GroupCategoriesCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_destroy

> group_categories_destroy(group_category_id)
Delete a Group Category

Deletes a group category and all groups under it. Protected group categories can not be deleted, i.e. \"communities\" and \"student_organized\".

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_export

> group_categories_export(group_category_id)
export groups in and users in category

Returns a csv file of users in format ready to import.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_export_tags

> group_categories_export_tags(course_id)
export tags and users in course

Returns a csv file of users in format ready to import.

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


## group_categories_groups

> models::Group group_categories_groups(group_category_id, page, per_page)
List groups in group category

Returns a paginated list of groups in a group category

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Group**](Group.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_import

> String group_categories_import(group_category_id, group_categories_import_request)
Import category groups

Create Groups in a Group Category through a CSV import  For more information on the format that's expected here, please see the \"Group Category CSV\" section in the API docs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |
**group_categories_import_request** | Option<[**GroupCategoriesImportRequest**](GroupCategoriesImportRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_import_tags

> String group_categories_import_tags(course_id, group_categories_import_tags_request)
Import differentiation tags

Create Differentiation Tags through a CSV import  For more information on the format that's expected here, please see the \"Differentiation Tag CSV\" section in the API docs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**group_categories_import_tags_request** | Option<[**GroupCategoriesImportTagsRequest**](GroupCategoriesImportTagsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_index_for_accounts

> serde_json::Value group_categories_index_for_accounts(account_id, collaboration_state, page, per_page)
List group categories for a context

Returns a paginated list of group categories in a context. The list returned depends on the permissions of the current user and the specified collaboration state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**collaboration_state** | Option<**String**> | Filter group categories by their collaboration state: - \"all\": Return both collaborative and non-collaborative group categories - \"collaborative\": Return only collaborative group categories (default) - \"non_collaborative\": Return only non-collaborative group categories |  |
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


## group_categories_index_for_courses

> serde_json::Value group_categories_index_for_courses(course_id, collaboration_state, page, per_page)
List group categories for a context

Returns a paginated list of group categories in a context. The list returned depends on the permissions of the current user and the specified collaboration state.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**collaboration_state** | Option<**String**> | Filter group categories by their collaboration state: - \"all\": Return both collaborative and non-collaborative group categories - \"collaborative\": Return only collaborative group categories (default) - \"non_collaborative\": Return only non-collaborative group categories |  |
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


## group_categories_show

> String group_categories_show(group_category_id)
Get a single group category

Returns the data for a single group category, or a 401 if the caller doesn't have the rights to see it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_update

> String group_categories_update(group_category_id, group_categories_update_request)
Update a Group Category

Modifies an existing group category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |
**group_categories_update_request** | Option<[**GroupCategoriesUpdateRequest**](GroupCategoriesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/group_categories/<group_category_id> \\     -X PUT \\     -F 'name=Project Groups' \\     -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_categories_users

> models::User group_categories_users(group_category_id, search_term, unassigned, page, per_page)
List users in group category

Returns a paginated list of users in the group category.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | Scope response to group_category_id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. Must be at least 3 characters. |  |
**unassigned** | Option<**bool**> | Set this value to true if you wish only to search unassigned users in the group category. |  |
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

