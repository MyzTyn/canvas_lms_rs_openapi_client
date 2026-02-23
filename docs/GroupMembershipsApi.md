# \GroupMembershipsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**group_memberships_create**](GroupMembershipsApi.md#group_memberships_create) | **POST** /groups/{group_id}/memberships | Create a membership
[**group_memberships_destroy_bulk**](GroupMembershipsApi.md#group_memberships_destroy_bulk) | **DELETE** /groups/{group_id}/users | Bulk delete memberships
[**group_memberships_destroy_for_groups**](GroupMembershipsApi.md#group_memberships_destroy_for_groups) | **DELETE** /groups/{group_id}/memberships/{membership_id} | Leave a group
[**group_memberships_destroy_for_groups2**](GroupMembershipsApi.md#group_memberships_destroy_for_groups2) | **DELETE** /groups/{group_id}/users/{user_id} | Leave a group
[**group_memberships_index**](GroupMembershipsApi.md#group_memberships_index) | **GET** /groups/{group_id}/memberships | List group memberships
[**group_memberships_show_for_groups**](GroupMembershipsApi.md#group_memberships_show_for_groups) | **GET** /groups/{group_id}/memberships/{membership_id} | Get a single group membership
[**group_memberships_show_for_groups2**](GroupMembershipsApi.md#group_memberships_show_for_groups2) | **GET** /groups/{group_id}/users/{user_id} | Get a single group membership
[**group_memberships_update_for_groups**](GroupMembershipsApi.md#group_memberships_update_for_groups) | **PUT** /groups/{group_id}/memberships/{membership_id} | Update a membership
[**group_memberships_update_for_groups2**](GroupMembershipsApi.md#group_memberships_update_for_groups2) | **PUT** /groups/{group_id}/users/{user_id} | Update a membership



## group_memberships_create

> String group_memberships_create(group_id, group_memberships_create_request)
Create a membership

Join, or request to join, a group, depending on the join_level of the group. If the membership or join request already exists, then it is simply returned.  For differentiation tags, you can bulk add users using one of two methods:  1. Provide an array of user IDs via the `members[]` parameter.  2. Use the course-wide option with the following parameters:    - `all_in_group_course` [Boolean]: If set to true, the endpoint will add      every currently enrolled student (from the course context) to the      differentiation tag.    - `exclude_user_ids[]` [Integer]: When using `all_in_group_course`, you can      optionally exclude specific users by providing their IDs in this parameter.  In this context, these parameters only apply to differentiation tag memberships.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**group_memberships_create_request** | Option<[**GroupMembershipsCreateRequest**](GroupMembershipsCreateRequest.md)> | Request body parameters  **Example Request:** ``` (Individual membership creation) curl https://<canvas>/api/v1/groups/<group_id>/memberships \\      -F 'user_id=self' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_destroy_bulk

> serde_json::Value group_memberships_destroy_bulk(group_id, user_ids)
Bulk delete memberships

Bulk deletes memberships by providing an array of user IDs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] - An array of user IDs to delete memberships in bulk. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_destroy_for_groups

> group_memberships_destroy_for_groups(group_id, membership_id)
Leave a group

Leave a group if you are allowed to leave (some groups, such as sets of course groups created by teachers, cannot be left). You may also use 'self' in place of a membership_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**membership_id** | **String** | Scope response to membership_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_destroy_for_groups2

> group_memberships_destroy_for_groups2(user_id, group_id)
Leave a group

Leave a group if you are allowed to leave (some groups, such as sets of course groups created by teachers, cannot be left). You may also use 'self' in place of a membership_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_index

> models::GroupMembership group_memberships_index(group_id, filter_states, page, per_page)
List group memberships

A paginated list of the members of a group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**filter_states** | Option<[**Vec<String>**](String.md)> | [String, \"accepted\"|\"invited\"|\"requested\"] Only list memberships with the given workflow_states. By default it will return all memberships. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::GroupMembership**](GroupMembership.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_show_for_groups

> String group_memberships_show_for_groups(group_id, membership_id)
Get a single group membership

Returns the group membership with the given membership id or user id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**membership_id** | **String** | Scope response to membership_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_show_for_groups2

> String group_memberships_show_for_groups2(user_id, group_id)
Get a single group membership

Returns the group membership with the given membership id or user id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_update_for_groups

> String group_memberships_update_for_groups(group_id, membership_id, group_memberships_update_for_groups_request)
Update a membership

Accept a membership request, or add/remove moderator rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**membership_id** | **String** | Scope response to membership_id | [required] |
**group_memberships_update_for_groups_request** | Option<[**GroupMembershipsUpdateForGroupsRequest**](GroupMembershipsUpdateForGroupsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## group_memberships_update_for_groups2

> String group_memberships_update_for_groups2(user_id, group_id, group_memberships_update_for_groups_request)
Update a membership

Accept a membership request, or add/remove moderator rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**group_id** | **String** | Scope response to group_id | [required] |
**group_memberships_update_for_groups_request** | Option<[**GroupMembershipsUpdateForGroupsRequest**](GroupMembershipsUpdateForGroupsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

