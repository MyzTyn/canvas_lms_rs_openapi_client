# \NoCompatibleToolApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**no_compatible_tool_api_index_for_courses**](NoCompatibleToolApi.md#no_compatible_tool_api_index_for_courses) | **GET** /courses/{course_id}/collaborations | List collaborations
[**no_compatible_tool_api_index_for_groups**](NoCompatibleToolApi.md#no_compatible_tool_api_index_for_groups) | **GET** /groups/{group_id}/collaborations | List collaborations
[**no_compatible_tool_members**](NoCompatibleToolApi.md#no_compatible_tool_members) | **GET** /collaborations/{id}/members | List members of a collaboration.
[**no_compatible_tool_potential_collaborators_for_courses**](NoCompatibleToolApi.md#no_compatible_tool_potential_collaborators_for_courses) | **GET** /courses/{course_id}/potential_collaborators | List potential members
[**no_compatible_tool_potential_collaborators_for_groups**](NoCompatibleToolApi.md#no_compatible_tool_potential_collaborators_for_groups) | **GET** /groups/{group_id}/potential_collaborators | List potential members



## no_compatible_tool_api_index_for_courses

> serde_json::Value no_compatible_tool_api_index_for_courses(course_id, page, per_page)
List collaborations

A paginated list of collaborations the current user has access to in the context of the course provided in the url. NOTE: this only returns ExternalToolCollaboration type collaborations.    curl https://<canvas>/api/v1/courses/1/collaborations/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
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


## no_compatible_tool_api_index_for_groups

> serde_json::Value no_compatible_tool_api_index_for_groups(group_id, page, per_page)
List collaborations

A paginated list of collaborations the current user has access to in the context of the course provided in the url. NOTE: this only returns ExternalToolCollaboration type collaborations.    curl https://<canvas>/api/v1/courses/1/collaborations/

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
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


## no_compatible_tool_members

> models::Collaborator no_compatible_tool_members(id, include, page, per_page)
List members of a collaboration.

A paginated list of the collaborators of a given collaboration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"collaborator_lti_id\"|\"avatar_image_url\"] - \"collaborator_lti_id\": Optional information to include with each member.   Represents an identifier to be used for the member in an LTI context. - \"avatar_image_url\": Optional information to include with each member.   The url for the avatar of a collaborator with type 'user'. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Collaborator**](Collaborator.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## no_compatible_tool_potential_collaborators_for_courses

> serde_json::Value no_compatible_tool_potential_collaborators_for_courses(course_id, page, per_page)
List potential members

A paginated list of the users who can potentially be added to a collaboration in the given context.  For courses, this consists of all enrolled users.  For groups, it is comprised of the group members plus the admins of the course containing the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
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


## no_compatible_tool_potential_collaborators_for_groups

> serde_json::Value no_compatible_tool_potential_collaborators_for_groups(group_id, page, per_page)
List potential members

A paginated list of the users who can potentially be added to a collaboration in the given context.  For courses, this consists of all enrolled users.  For groups, it is comprised of the group members plus the admins of the course containing the group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
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

