# \GroupsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**groups_activity_stream**](GroupsApi.md#groups_activity_stream) | **GET** /groups/{group_id}/activity_stream | Group activity stream
[**groups_activity_stream_summary**](GroupsApi.md#groups_activity_stream_summary) | **GET** /groups/{group_id}/activity_stream/summary | Group activity stream summary
[**groups_bulk_user_tags**](GroupsApi.md#groups_bulk_user_tags) | **GET** /courses/{course_id}/bulk_user_tags | Bulk fetch user tags for multiple users in a course
[**groups_context_index_for_accounts**](GroupsApi.md#groups_context_index_for_accounts) | **GET** /accounts/{account_id}/groups | List the groups available in a context.
[**groups_context_index_for_courses**](GroupsApi.md#groups_context_index_for_courses) | **GET** /courses/{course_id}/groups | List the groups available in a context.
[**groups_create_file**](GroupsApi.md#groups_create_file) | **POST** /groups/{group_id}/files | Upload a file
[**groups_create_other**](GroupsApi.md#groups_create_other) | **POST** /group_categories/{group_category_id}/groups | Create a group
[**groups_create_other2**](GroupsApi.md#groups_create_other2) | **POST** /groups | Create a group
[**groups_destroy**](GroupsApi.md#groups_destroy) | **DELETE** /groups/{group_id} | Delete a group
[**groups_index**](GroupsApi.md#groups_index) | **GET** /users/self/groups | List your groups
[**groups_invite**](GroupsApi.md#groups_invite) | **POST** /groups/{group_id}/invite | Invite others to a group
[**groups_permissions**](GroupsApi.md#groups_permissions) | **GET** /groups/{group_id}/permissions | Permissions
[**groups_preview_html**](GroupsApi.md#groups_preview_html) | **POST** /groups/{group_id}/preview_html | Preview processed html
[**groups_show**](GroupsApi.md#groups_show) | **GET** /groups/{group_id} | Get a single group
[**groups_update**](GroupsApi.md#groups_update) | **PUT** /groups/{group_id} | Edit a group
[**groups_users**](GroupsApi.md#groups_users) | **GET** /groups/{group_id}/users | List group's users



## groups_activity_stream

> groups_activity_stream(group_id)
Group activity stream

Returns the current user's group-specific activity stream, paginated.  For full documentation, see the API documentation for the user activity stream, in the user api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_activity_stream_summary

> groups_activity_stream_summary(group_id)
Group activity stream summary

Returns a summary of the current user's group-specific activity stream.  For full documentation, see the API documentation for the user activity stream summary, in the user api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_bulk_user_tags

> serde_json::Value groups_bulk_user_tags(course_id, user_ids)
Bulk fetch user tags for multiple users in a course

Returns a mapping of user IDs to arrays of non-collaborative group (tag) IDs for each user in the given course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | The ID of the course context (from the route). | [required] |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] An array of user IDs to fetch tags for. |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_context_index_for_accounts

> serde_json::Value groups_context_index_for_accounts(account_id, only_own_groups, include, collaboration_state, page, per_page)
List the groups available in a context.

Returns the paginated list of active groups in the given context that are visible to user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**only_own_groups** | Option<**bool**> | Will only include groups that the user belongs to if this is set |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"tabs\"] - \"tabs\": Include the list of tabs configured for each group.  See the   {api:TabsController#index List available tabs API} for more information. |  |
**collaboration_state** | Option<**String**> | Filter groups by their collaboration state: - \"all\": Return both collaborative and non-collaborative groups - \"collaborative\": Return only collaborative groups (default) - \"non_collaborative\": Return only non-collaborative groups |  |
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


## groups_context_index_for_courses

> serde_json::Value groups_context_index_for_courses(course_id, only_own_groups, include, collaboration_state, page, per_page)
List the groups available in a context.

Returns the paginated list of active groups in the given context that are visible to user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**only_own_groups** | Option<**bool**> | Will only include groups that the user belongs to if this is set |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"tabs\"] - \"tabs\": Include the list of tabs configured for each group.  See the   {api:TabsController#index List available tabs API} for more information. |  |
**collaboration_state** | Option<**String**> | Filter groups by their collaboration state: - \"all\": Return both collaborative and non-collaborative groups - \"collaborative\": Return only collaborative groups (default) - \"non_collaborative\": Return only non-collaborative groups |  |
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


## groups_create_file

> groups_create_file(group_id)
Upload a file

Upload a file to the group.  This API endpoint is the first step in uploading a file to a group. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  Only those with the \"Manage Files\" permission on a group can upload files to the group. By default, this is anybody participating in the group, or any admin over the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create_other

> String groups_create_other(group_category_id, groups_create_other_request)
Create a group

Creates a new group. Groups created using the \"/api/v1/groups/\" endpoint will be community groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_category_id** | **String** | ID of the group category | [required] |
**groups_create_other_request** | Option<[**GroupsCreateOtherRequest**](GroupsCreateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_create_other2

> String groups_create_other2(groups_create_other_request)
Create a group

Creates a new group. Groups created using the \"/api/v1/groups/\" endpoint will be community groups.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**groups_create_other_request** | Option<[**GroupsCreateOtherRequest**](GroupsCreateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_destroy

> String groups_destroy(group_id)
Delete a group

Deletes a group and removes all members.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_index

> models::Group groups_index(context_type, include, page, per_page)
List your groups

Returns a paginated list of active groups for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_type** | Option<**String**> | Only include groups that are in this type of context. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"tabs\"] - \"tabs\": Include the list of tabs configured for each group.  See the   {api:TabsController#index List available tabs API} for more information. |  |
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


## groups_invite

> groups_invite(group_id, groups_invite_request)
Invite others to a group

Sends an invitation to all supplied email addresses which will allow the receivers to join the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**groups_invite_request** | Option<[**GroupsInviteRequest**](GroupsInviteRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/groups/<group_id>/invite \\      -F 'invitees[]=leonard@example.com' \\      -F 'invitees[]=sheldon@example.com' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_permissions

> groups_permissions(group_id, permissions)
Permissions

Returns permission information for the calling user in the given group. See also the {api:AccountsController#permissions Account} and {api:CoursesController#permissions Course} counterparts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**permissions** | Option<[**Vec<String>**](String.md)> | [String] List of permissions to check against the authenticated user. Permission names are documented in the {api:RoleOverridesController#manageable_permissions List assignable permissions} endpoint. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_preview_html

> groups_preview_html(group_id, courses_preview_html_request)
Preview processed html

Preview html content processed for this group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**courses_preview_html_request** | Option<[**CoursesPreviewHtmlRequest**](CoursesPreviewHtmlRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/groups/<group_id>/preview_html \\      -F 'html=<p><badhtml></badhtml>processed html</p>' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_show

> String groups_show(group_id, include)
Get a single group

Returns the data for a single group, or a 401 if the caller doesn't have the rights to see it.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"permissions\", \"tabs\"] - \"permissions\": Include permissions the current user has   for the group. - \"tabs\": Include the list of tabs configured for each group.  See the   {api:TabsController#index List available tabs API} for more information. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_update

> String groups_update(group_id, groups_update_request)
Edit a group

Modifies an existing group.  Note that to set an avatar image for the group, you must first upload the image file to the group, and the use the id in the response as the argument to this function.  See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**groups_update_request** | Option<[**GroupsUpdateRequest**](GroupsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/groups/<group_id> \\      -X PUT \\      -F 'name=Algebra Teachers' \\      -F 'join_level=parent_context_request' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## groups_users

> models::User groups_users(group_id, search_term, include, exclude_inactive, page, per_page)
List group's users

Returns a paginated list of users in the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | Scope response to group_id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. Must be at least 2 characters. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"] \"avatar_url\": Include users' avatar_urls. |  |
**exclude_inactive** | Option<**bool**> | Whether to filter out inactive users from the results. Defaults to false unless explicitly provided. |  |
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

