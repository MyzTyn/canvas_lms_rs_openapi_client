# \OutcomeGroupsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outcome_groups_api_create_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_create_for_accounts) | **POST** /accounts/{account_id}/outcome_groups/{id}/subgroups | Create a subgroup
[**outcome_groups_api_create_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_create_for_courses) | **POST** /courses/{course_id}/outcome_groups/{id}/subgroups | Create a subgroup
[**outcome_groups_api_create_other**](OutcomeGroupsApiApi.md#outcome_groups_api_create_other) | **POST** /global/outcome_groups/{id}/subgroups | Create a subgroup
[**outcome_groups_api_destroy_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_destroy_for_accounts) | **DELETE** /accounts/{account_id}/outcome_groups/{id} | Delete an outcome group
[**outcome_groups_api_destroy_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_destroy_for_courses) | **DELETE** /courses/{course_id}/outcome_groups/{id} | Delete an outcome group
[**outcome_groups_api_destroy_other**](OutcomeGroupsApiApi.md#outcome_groups_api_destroy_other) | **DELETE** /global/outcome_groups/{id} | Delete an outcome group
[**outcome_groups_api_import_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_import_for_accounts) | **POST** /accounts/{account_id}/outcome_groups/{id}/import | Import an outcome group
[**outcome_groups_api_import_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_import_for_courses) | **POST** /courses/{course_id}/outcome_groups/{id}/import | Import an outcome group
[**outcome_groups_api_import_other**](OutcomeGroupsApiApi.md#outcome_groups_api_import_other) | **POST** /global/outcome_groups/{id}/import | Import an outcome group
[**outcome_groups_api_index_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_index_for_accounts) | **GET** /accounts/{account_id}/outcome_groups | Get all outcome groups for context
[**outcome_groups_api_index_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_index_for_courses) | **GET** /courses/{course_id}/outcome_groups | Get all outcome groups for context
[**outcome_groups_api_link_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_link_for_accounts) | **POST** /accounts/{account_id}/outcome_groups/{id}/outcomes | Create/link an outcome
[**outcome_groups_api_link_for_accounts2**](OutcomeGroupsApiApi.md#outcome_groups_api_link_for_accounts2) | **POST** /accounts/{account_id}/outcome_groups/{id}/outcomes/{outcome_id} | Create/link an outcome
[**outcome_groups_api_link_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_link_for_courses) | **POST** /courses/{course_id}/outcome_groups/{id}/outcomes | Create/link an outcome
[**outcome_groups_api_link_for_courses2**](OutcomeGroupsApiApi.md#outcome_groups_api_link_for_courses2) | **POST** /courses/{course_id}/outcome_groups/{id}/outcomes/{outcome_id} | Create/link an outcome
[**outcome_groups_api_link_index_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_link_index_for_accounts) | **GET** /accounts/{account_id}/outcome_group_links | Get all outcome links for context
[**outcome_groups_api_link_index_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_link_index_for_courses) | **GET** /courses/{course_id}/outcome_group_links | Get all outcome links for context
[**outcome_groups_api_link_other**](OutcomeGroupsApiApi.md#outcome_groups_api_link_other) | **POST** /global/outcome_groups/{id}/outcomes | Create/link an outcome
[**outcome_groups_api_link_other2**](OutcomeGroupsApiApi.md#outcome_groups_api_link_other2) | **POST** /global/outcome_groups/{id}/outcomes/{outcome_id} | Create/link an outcome
[**outcome_groups_api_outcomes_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_outcomes_for_accounts) | **GET** /accounts/{account_id}/outcome_groups/{id}/outcomes | List linked outcomes
[**outcome_groups_api_outcomes_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_outcomes_for_courses) | **GET** /courses/{course_id}/outcome_groups/{id}/outcomes | List linked outcomes
[**outcome_groups_api_outcomes_other**](OutcomeGroupsApiApi.md#outcome_groups_api_outcomes_other) | **GET** /global/outcome_groups/{id}/outcomes | List linked outcomes
[**outcome_groups_api_redirect_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_redirect_for_accounts) | **GET** /accounts/{account_id}/root_outcome_group | Redirect to root outcome group for context
[**outcome_groups_api_redirect_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_redirect_for_courses) | **GET** /courses/{course_id}/root_outcome_group | Redirect to root outcome group for context
[**outcome_groups_api_redirect_other**](OutcomeGroupsApiApi.md#outcome_groups_api_redirect_other) | **GET** /global/root_outcome_group | Redirect to root outcome group for context
[**outcome_groups_api_show_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_show_for_accounts) | **GET** /accounts/{account_id}/outcome_groups/{id} | Show an outcome group
[**outcome_groups_api_show_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_show_for_courses) | **GET** /courses/{course_id}/outcome_groups/{id} | Show an outcome group
[**outcome_groups_api_show_other**](OutcomeGroupsApiApi.md#outcome_groups_api_show_other) | **GET** /global/outcome_groups/{id} | Show an outcome group
[**outcome_groups_api_subgroups_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_subgroups_for_accounts) | **GET** /accounts/{account_id}/outcome_groups/{id}/subgroups | List subgroups
[**outcome_groups_api_subgroups_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_subgroups_for_courses) | **GET** /courses/{course_id}/outcome_groups/{id}/subgroups | List subgroups
[**outcome_groups_api_subgroups_other**](OutcomeGroupsApiApi.md#outcome_groups_api_subgroups_other) | **GET** /global/outcome_groups/{id}/subgroups | List subgroups
[**outcome_groups_api_unlink_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_unlink_for_accounts) | **DELETE** /accounts/{account_id}/outcome_groups/{id}/outcomes/{outcome_id} | Unlink an outcome
[**outcome_groups_api_unlink_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_unlink_for_courses) | **DELETE** /courses/{course_id}/outcome_groups/{id}/outcomes/{outcome_id} | Unlink an outcome
[**outcome_groups_api_unlink_other**](OutcomeGroupsApiApi.md#outcome_groups_api_unlink_other) | **DELETE** /global/outcome_groups/{id}/outcomes/{outcome_id} | Unlink an outcome
[**outcome_groups_api_update_for_accounts**](OutcomeGroupsApiApi.md#outcome_groups_api_update_for_accounts) | **PUT** /accounts/{account_id}/outcome_groups/{id} | Update an outcome group
[**outcome_groups_api_update_for_courses**](OutcomeGroupsApiApi.md#outcome_groups_api_update_for_courses) | **PUT** /courses/{course_id}/outcome_groups/{id} | Update an outcome group
[**outcome_groups_api_update_other**](OutcomeGroupsApiApi.md#outcome_groups_api_update_other) | **PUT** /global/outcome_groups/{id} | Update an outcome group



## outcome_groups_api_create_for_accounts

> String outcome_groups_api_create_for_accounts(account_id, id, outcome_groups_api_create_other_request)
Create a subgroup

Creates a new empty subgroup under the outcome group with the given title and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_create_other_request** | Option<[**OutcomeGroupsApiCreateOtherRequest**](OutcomeGroupsApiCreateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_create_for_courses

> String outcome_groups_api_create_for_courses(course_id, id, outcome_groups_api_create_other_request)
Create a subgroup

Creates a new empty subgroup under the outcome group with the given title and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_create_other_request** | Option<[**OutcomeGroupsApiCreateOtherRequest**](OutcomeGroupsApiCreateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_create_other

> String outcome_groups_api_create_other(id, outcome_groups_api_create_other_request)
Create a subgroup

Creates a new empty subgroup under the outcome group with the given title and description.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_create_other_request** | Option<[**OutcomeGroupsApiCreateOtherRequest**](OutcomeGroupsApiCreateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_destroy_for_accounts

> String outcome_groups_api_destroy_for_accounts(account_id, id)
Delete an outcome group

Deleting an outcome group deletes descendant outcome groups and outcome links. The linked outcomes themselves are only deleted if all links to the outcome were deleted.  Aligned outcomes cannot be deleted; as such, if all remaining links to an aligned outcome are included in this group's descendants, the group deletion will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_destroy_for_courses

> String outcome_groups_api_destroy_for_courses(course_id, id)
Delete an outcome group

Deleting an outcome group deletes descendant outcome groups and outcome links. The linked outcomes themselves are only deleted if all links to the outcome were deleted.  Aligned outcomes cannot be deleted; as such, if all remaining links to an aligned outcome are included in this group's descendants, the group deletion will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_destroy_other

> String outcome_groups_api_destroy_other(id)
Delete an outcome group

Deleting an outcome group deletes descendant outcome groups and outcome links. The linked outcomes themselves are only deleted if all links to the outcome were deleted.  Aligned outcomes cannot be deleted; as such, if all remaining links to an aligned outcome are included in this group's descendants, the group deletion will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_import_for_accounts

> String outcome_groups_api_import_for_accounts(account_id, id, outcome_groups_api_import_other_request)
Import an outcome group

Creates a new subgroup of the outcome group with the same title and description as the source group, then creates links in that new subgroup to the same outcomes that are linked in the source group. Recurses on the subgroups of the source group, importing them each in turn into the new subgroup.  Allows you to copy organizational structure, but does not create copies of the outcomes themselves, only new links.  The source group must be either global, from the same context as this outcome group, or from an associated account. The source group cannot be the root outcome group of its context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_import_other_request** | Option<[**OutcomeGroupsApiImportOtherRequest**](OutcomeGroupsApiImportOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_import_for_courses

> String outcome_groups_api_import_for_courses(course_id, id, outcome_groups_api_import_other_request)
Import an outcome group

Creates a new subgroup of the outcome group with the same title and description as the source group, then creates links in that new subgroup to the same outcomes that are linked in the source group. Recurses on the subgroups of the source group, importing them each in turn into the new subgroup.  Allows you to copy organizational structure, but does not create copies of the outcomes themselves, only new links.  The source group must be either global, from the same context as this outcome group, or from an associated account. The source group cannot be the root outcome group of its context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_import_other_request** | Option<[**OutcomeGroupsApiImportOtherRequest**](OutcomeGroupsApiImportOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_import_other

> String outcome_groups_api_import_other(id, outcome_groups_api_import_other_request)
Import an outcome group

Creates a new subgroup of the outcome group with the same title and description as the source group, then creates links in that new subgroup to the same outcomes that are linked in the source group. Recurses on the subgroups of the source group, importing them each in turn into the new subgroup.  Allows you to copy organizational structure, but does not create copies of the outcomes themselves, only new links.  The source group must be either global, from the same context as this outcome group, or from an associated account. The source group cannot be the root outcome group of its context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_import_other_request** | Option<[**OutcomeGroupsApiImportOtherRequest**](OutcomeGroupsApiImportOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_index_for_accounts

> serde_json::Value outcome_groups_api_index_for_accounts(account_id, page, per_page)
Get all outcome groups for context

Returns a list of all outcome groups in the specified context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
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


## outcome_groups_api_index_for_courses

> serde_json::Value outcome_groups_api_index_for_courses(course_id, page, per_page)
Get all outcome groups for context

Returns a list of all outcome groups in the specified context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
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


## outcome_groups_api_link_for_accounts

> String outcome_groups_api_link_for_accounts(account_id, id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_link_for_accounts2

> String outcome_groups_api_link_for_accounts2(outcome_id, account_id, id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome | [required] |
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_link_for_courses

> String outcome_groups_api_link_for_courses(course_id, id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_link_for_courses2

> String outcome_groups_api_link_for_courses2(outcome_id, course_id, id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome | [required] |
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_link_index_for_accounts

> serde_json::Value outcome_groups_api_link_index_for_accounts(account_id, outcome_style, outcome_group_style, page, per_page)
Get all outcome links for context

Returns a list of all outcome links in the specified context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**outcome_style** | Option<**String**> | The detail level of the outcomes. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
**outcome_group_style** | Option<**String**> | The detail level of the outcome groups. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
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


## outcome_groups_api_link_index_for_courses

> serde_json::Value outcome_groups_api_link_index_for_courses(course_id, outcome_style, outcome_group_style, page, per_page)
Get all outcome links for context

Returns a list of all outcome links in the specified context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**outcome_style** | Option<**String**> | The detail level of the outcomes. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
**outcome_group_style** | Option<**String**> | The detail level of the outcome groups. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
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


## outcome_groups_api_link_other

> String outcome_groups_api_link_other(id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_link_other2

> String outcome_groups_api_link_other2(outcome_id, id, outcome_groups_api_link_other_request)
Create/link an outcome

Link an outcome into the outcome group. The outcome to link can either be specified by a PUT to the link URL for a specific outcome (the outcome_id in the PUT URLs) or by supplying the information for a new outcome (title, description, ratings, mastery_points) in a POST to the collection.  If linking an existing outcome, the outcome_id must identify an outcome available to this context; i.e. an outcome owned by this group's context, an outcome owned by an associated account, or a global outcome. With outcome_id present, any other parameters (except move_from) are ignored.  If defining a new outcome, the outcome is created in the outcome group's context using the provided title, description, ratings, and mastery points; the title is required but all other fields are optional. The new outcome is then linked into the outcome group.  If ratings are provided when creating a new outcome, an embedded rubric criterion is included in the new outcome. This criterion's mastery_points default to the maximum points in the highest rating if not specified in the mastery_points parameter. Any ratings lacking a description are given a default of \"No description\". Any ratings lacking a point value are given a default of 0. If no ratings are provided, the mastery_points parameter is ignored.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_id** | **String** | ID of the outcome | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_link_other_request** | Option<[**OutcomeGroupsApiLinkOtherRequest**](OutcomeGroupsApiLinkOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_outcomes_for_accounts

> serde_json::Value outcome_groups_api_outcomes_for_accounts(account_id, id, outcome_style, page, per_page)
List linked outcomes

A paginated list of the immediate OutcomeLink children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_style** | Option<**String**> | The detail level of the outcomes. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
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


## outcome_groups_api_outcomes_for_courses

> serde_json::Value outcome_groups_api_outcomes_for_courses(course_id, id, outcome_style, page, per_page)
List linked outcomes

A paginated list of the immediate OutcomeLink children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_style** | Option<**String**> | The detail level of the outcomes. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
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


## outcome_groups_api_outcomes_other

> serde_json::Value outcome_groups_api_outcomes_other(id, outcome_style, page, per_page)
List linked outcomes

A paginated list of the immediate OutcomeLink children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_style** | Option<**String**> | The detail level of the outcomes. Defaults to \"abbrev\". Specify \"full\" for more information. |  |
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


## outcome_groups_api_redirect_for_accounts

> outcome_groups_api_redirect_for_accounts(account_id)
Redirect to root outcome group for context

Convenience redirect to find the root outcome group for a particular context. Will redirect to the appropriate outcome group's URL.

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


## outcome_groups_api_redirect_for_courses

> outcome_groups_api_redirect_for_courses(course_id)
Redirect to root outcome group for context

Convenience redirect to find the root outcome group for a particular context. Will redirect to the appropriate outcome group's URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_redirect_other

> outcome_groups_api_redirect_other()
Redirect to root outcome group for context

Convenience redirect to find the root outcome group for a particular context. Will redirect to the appropriate outcome group's URL.

### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_show_for_accounts

> String outcome_groups_api_show_for_accounts(account_id, id)
Show an outcome group

Returns detailed information about a specific outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_show_for_courses

> String outcome_groups_api_show_for_courses(course_id, id)
Show an outcome group

Returns detailed information about a specific outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_show_other

> String outcome_groups_api_show_other(id)
Show an outcome group

Returns detailed information about a specific outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_subgroups_for_accounts

> serde_json::Value outcome_groups_api_subgroups_for_accounts(account_id, id, page, per_page)
List subgroups

A paginated list of the immediate OutcomeGroup children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
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


## outcome_groups_api_subgroups_for_courses

> serde_json::Value outcome_groups_api_subgroups_for_courses(course_id, id, page, per_page)
List subgroups

A paginated list of the immediate OutcomeGroup children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
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


## outcome_groups_api_subgroups_other

> serde_json::Value outcome_groups_api_subgroups_other(id, page, per_page)
List subgroups

A paginated list of the immediate OutcomeGroup children of the outcome group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
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


## outcome_groups_api_unlink_for_accounts

> String outcome_groups_api_unlink_for_accounts(account_id, id, outcome_id)
Unlink an outcome

Unlinking an outcome only deletes the outcome itself if this was the last link to the outcome in any group in any context. Aligned outcomes cannot be deleted; as such, if this is the last link to an aligned outcome, the unlinking will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_id** | **String** | Scope response to outcome_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_unlink_for_courses

> String outcome_groups_api_unlink_for_courses(course_id, id, outcome_id)
Unlink an outcome

Unlinking an outcome only deletes the outcome itself if this was the last link to the outcome in any group in any context. Aligned outcomes cannot be deleted; as such, if this is the last link to an aligned outcome, the unlinking will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_id** | **String** | Scope response to outcome_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_unlink_other

> String outcome_groups_api_unlink_other(id, outcome_id)
Unlink an outcome

Unlinking an outcome only deletes the outcome itself if this was the last link to the outcome in any group in any context. Aligned outcomes cannot be deleted; as such, if this is the last link to an aligned outcome, the unlinking will fail.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_id** | **String** | Scope response to outcome_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_update_for_accounts

> String outcome_groups_api_update_for_accounts(account_id, id, outcome_groups_api_update_other_request)
Update an outcome group

Modify an existing outcome group. Fields not provided are left as is; unrecognized fields are ignored.  When changing the parent outcome group, the new parent group must belong to the same context as this outcome group, and must not be a descendant of this outcome group (i.e. no cycles allowed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_update_other_request** | Option<[**OutcomeGroupsApiUpdateOtherRequest**](OutcomeGroupsApiUpdateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_update_for_courses

> String outcome_groups_api_update_for_courses(course_id, id, outcome_groups_api_update_other_request)
Update an outcome group

Modify an existing outcome group. Fields not provided are left as is; unrecognized fields are ignored.  When changing the parent outcome group, the new parent group must belong to the same context as this outcome group, and must not be a descendant of this outcome group (i.e. no cycles allowed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_update_other_request** | Option<[**OutcomeGroupsApiUpdateOtherRequest**](OutcomeGroupsApiUpdateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_groups_api_update_other

> String outcome_groups_api_update_other(id, outcome_groups_api_update_other_request)
Update an outcome group

Modify an existing outcome group. Fields not provided are left as is; unrecognized fields are ignored.  When changing the parent outcome group, the new parent group must belong to the same context as this outcome group, and must not be a descendant of this outcome group (i.e. no cycles allowed).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**outcome_groups_api_update_other_request** | Option<[**OutcomeGroupsApiUpdateOtherRequest**](OutcomeGroupsApiUpdateOtherRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

