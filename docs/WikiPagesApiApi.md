# \WikiPagesApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**wiki_pages_api_create_for_courses**](WikiPagesApiApi.md#wiki_pages_api_create_for_courses) | **POST** /courses/{course_id}/pages | Create page
[**wiki_pages_api_create_for_groups**](WikiPagesApiApi.md#wiki_pages_api_create_for_groups) | **POST** /groups/{group_id}/pages | Create page
[**wiki_pages_api_destroy_for_courses**](WikiPagesApiApi.md#wiki_pages_api_destroy_for_courses) | **DELETE** /courses/{course_id}/pages/{url_or_id} | Delete page
[**wiki_pages_api_destroy_for_groups**](WikiPagesApiApi.md#wiki_pages_api_destroy_for_groups) | **DELETE** /groups/{group_id}/pages/{url_or_id} | Delete page
[**wiki_pages_api_duplicate**](WikiPagesApiApi.md#wiki_pages_api_duplicate) | **POST** /courses/{course_id}/pages/{url_or_id}/duplicate | Duplicate page
[**wiki_pages_api_index_for_courses**](WikiPagesApiApi.md#wiki_pages_api_index_for_courses) | **GET** /courses/{course_id}/pages | List pages
[**wiki_pages_api_index_for_groups**](WikiPagesApiApi.md#wiki_pages_api_index_for_groups) | **GET** /groups/{group_id}/pages | List pages
[**wiki_pages_api_revert_for_courses**](WikiPagesApiApi.md#wiki_pages_api_revert_for_courses) | **POST** /courses/{course_id}/pages/{url_or_id}/revisions/{revision_id} | Revert to revision
[**wiki_pages_api_revert_for_groups**](WikiPagesApiApi.md#wiki_pages_api_revert_for_groups) | **POST** /groups/{group_id}/pages/{url_or_id}/revisions/{revision_id} | Revert to revision
[**wiki_pages_api_revisions_for_courses**](WikiPagesApiApi.md#wiki_pages_api_revisions_for_courses) | **GET** /courses/{course_id}/pages/{url_or_id}/revisions | List revisions
[**wiki_pages_api_revisions_for_groups**](WikiPagesApiApi.md#wiki_pages_api_revisions_for_groups) | **GET** /groups/{group_id}/pages/{url_or_id}/revisions | List revisions
[**wiki_pages_api_show_for_courses**](WikiPagesApiApi.md#wiki_pages_api_show_for_courses) | **GET** /courses/{course_id}/pages/{url_or_id} | Show page
[**wiki_pages_api_show_for_groups**](WikiPagesApiApi.md#wiki_pages_api_show_for_groups) | **GET** /groups/{group_id}/pages/{url_or_id} | Show page
[**wiki_pages_api_show_front_page_for_courses**](WikiPagesApiApi.md#wiki_pages_api_show_front_page_for_courses) | **GET** /courses/{course_id}/front_page | Show front page
[**wiki_pages_api_show_front_page_for_groups**](WikiPagesApiApi.md#wiki_pages_api_show_front_page_for_groups) | **GET** /groups/{group_id}/front_page | Show front page
[**wiki_pages_api_show_revision_for_courses**](WikiPagesApiApi.md#wiki_pages_api_show_revision_for_courses) | **GET** /courses/{course_id}/pages/{url_or_id}/revisions/latest | Show revision
[**wiki_pages_api_show_revision_for_courses2**](WikiPagesApiApi.md#wiki_pages_api_show_revision_for_courses2) | **GET** /courses/{course_id}/pages/{url_or_id}/revisions/{revision_id} | Show revision
[**wiki_pages_api_show_revision_for_groups**](WikiPagesApiApi.md#wiki_pages_api_show_revision_for_groups) | **GET** /groups/{group_id}/pages/{url_or_id}/revisions/latest | Show revision
[**wiki_pages_api_show_revision_for_groups2**](WikiPagesApiApi.md#wiki_pages_api_show_revision_for_groups2) | **GET** /groups/{group_id}/pages/{url_or_id}/revisions/{revision_id} | Show revision
[**wiki_pages_api_update_for_courses**](WikiPagesApiApi.md#wiki_pages_api_update_for_courses) | **PUT** /courses/{course_id}/pages/{url_or_id} | Update/create page
[**wiki_pages_api_update_for_groups**](WikiPagesApiApi.md#wiki_pages_api_update_for_groups) | **PUT** /groups/{group_id}/pages/{url_or_id} | Update/create page
[**wiki_pages_api_update_front_page_for_courses**](WikiPagesApiApi.md#wiki_pages_api_update_front_page_for_courses) | **PUT** /courses/{course_id}/front_page | Update/create front page
[**wiki_pages_api_update_front_page_for_groups**](WikiPagesApiApi.md#wiki_pages_api_update_front_page_for_groups) | **PUT** /groups/{group_id}/front_page | Update/create front page



## wiki_pages_api_create_for_courses

> String wiki_pages_api_create_for_courses(course_id, wiki_pages_api_create_for_courses_request)
Create page

Create a new wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**wiki_pages_api_create_for_courses_request** | Option<[**WikiPagesApiCreateForCoursesRequest**](WikiPagesApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_create_for_groups

> String wiki_pages_api_create_for_groups(group_id, wiki_pages_api_create_for_courses_request)
Create page

Create a new wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**wiki_pages_api_create_for_courses_request** | Option<[**WikiPagesApiCreateForCoursesRequest**](WikiPagesApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_destroy_for_courses

> String wiki_pages_api_destroy_for_courses(course_id, url_or_id)
Delete page

Delete a wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_destroy_for_groups

> String wiki_pages_api_destroy_for_groups(group_id, url_or_id)
Delete page

Delete a wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_duplicate

> String wiki_pages_api_duplicate(course_id, url_or_id)
Duplicate page

Duplicate a wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_index_for_courses

> serde_json::Value wiki_pages_api_index_for_courses(course_id, sort, order, search_term, published, include, page, per_page)
List pages

A paginated list of the wiki pages associated with a course or group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**sort** | Option<**String**> | Sort results by this field. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
**search_term** | Option<**String**> | The partial title of the pages to match and return. |  |
**published** | Option<**bool**> | If true, include only published paqes. If false, exclude published pages. If not present, do not filter on published status. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"body\"] - \"enrollments\": Optionally include the page body with each Page. If this is a block_editor page, returns the block_editor_attributes. |  |
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


## wiki_pages_api_index_for_groups

> serde_json::Value wiki_pages_api_index_for_groups(group_id, sort, order, search_term, published, include, page, per_page)
List pages

A paginated list of the wiki pages associated with a course or group

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**sort** | Option<**String**> | Sort results by this field. |  |
**order** | Option<**String**> | The sorting order. Defaults to 'asc'. |  |
**search_term** | Option<**String**> | The partial title of the pages to match and return. |  |
**published** | Option<**bool**> | If true, include only published paqes. If false, exclude published pages. If not present, do not filter on published status. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"body\"] - \"enrollments\": Optionally include the page body with each Page. If this is a block_editor page, returns the block_editor_attributes. |  |
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


## wiki_pages_api_revert_for_courses

> String wiki_pages_api_revert_for_courses(course_id, revision_id, url_or_id)
Revert to revision

Revert a page to a prior revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**revision_id** | **String** | The revision to revert to (use the {api:WikiPagesApiController#revisions List Revisions API} to see available revisions) | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_revert_for_groups

> String wiki_pages_api_revert_for_groups(group_id, revision_id, url_or_id)
Revert to revision

Revert a page to a prior revision.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**revision_id** | **String** | The revision to revert to (use the {api:WikiPagesApiController#revisions List Revisions API} to see available revisions) | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_revisions_for_courses

> serde_json::Value wiki_pages_api_revisions_for_courses(course_id, url_or_id, page, per_page)
List revisions

A paginated list of the revisions of a page. Callers must have update rights on the page in order to see page history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
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


## wiki_pages_api_revisions_for_groups

> serde_json::Value wiki_pages_api_revisions_for_groups(group_id, url_or_id, page, per_page)
List revisions

A paginated list of the revisions of a page. Callers must have update rights on the page in order to see page history.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
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


## wiki_pages_api_show_for_courses

> String wiki_pages_api_show_for_courses(course_id, url_or_id)
Show page

Retrieve the content of a wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_for_groups

> String wiki_pages_api_show_for_groups(group_id, url_or_id)
Show page

Retrieve the content of a wiki page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_front_page_for_courses

> String wiki_pages_api_show_front_page_for_courses(course_id)
Show front page

Retrieve the content of the front page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_front_page_for_groups

> String wiki_pages_api_show_front_page_for_groups(group_id)
Show front page

Retrieve the content of the front page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_revision_for_courses

> String wiki_pages_api_show_revision_for_courses(course_id, url_or_id, summary)
Show revision

Retrieve the metadata and optionally content of a revision of the page. Note that retrieving historic versions of pages requires edit rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**summary** | Option<**bool**> | If set, exclude page content from results |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_revision_for_courses2

> String wiki_pages_api_show_revision_for_courses2(revision_id, course_id, url_or_id, summary)
Show revision

Retrieve the metadata and optionally content of a revision of the page. Note that retrieving historic versions of pages requires edit rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **String** | ID of the revision | [required] |
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**summary** | Option<**bool**> | If set, exclude page content from results |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_revision_for_groups

> String wiki_pages_api_show_revision_for_groups(group_id, url_or_id, summary)
Show revision

Retrieve the metadata and optionally content of a revision of the page. Note that retrieving historic versions of pages requires edit rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**summary** | Option<**bool**> | If set, exclude page content from results |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_show_revision_for_groups2

> String wiki_pages_api_show_revision_for_groups2(revision_id, group_id, url_or_id, summary)
Show revision

Retrieve the metadata and optionally content of a revision of the page. Note that retrieving historic versions of pages requires edit rights.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**revision_id** | **String** | ID of the revision | [required] |
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**summary** | Option<**bool**> | If set, exclude page content from results |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_update_for_courses

> String wiki_pages_api_update_for_courses(course_id, url_or_id, wiki_pages_api_update_for_courses_request)
Update/create page

Update the title or contents of a wiki page  NOTE: You cannot specify the ID when creating a page. If you pass a numeric value as the page identifier and that does not represent a page ID that already exists, it will be interpreted as a URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**wiki_pages_api_update_for_courses_request** | Option<[**WikiPagesApiUpdateForCoursesRequest**](WikiPagesApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_update_for_groups

> String wiki_pages_api_update_for_groups(group_id, url_or_id, wiki_pages_api_update_for_courses_request)
Update/create page

Update the title or contents of a wiki page  NOTE: You cannot specify the ID when creating a page. If you pass a numeric value as the page identifier and that does not represent a page ID that already exists, it will be interpreted as a URL.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**url_or_id** | **String** | Scope response to url_or_id | [required] |
**wiki_pages_api_update_for_courses_request** | Option<[**WikiPagesApiUpdateForCoursesRequest**](WikiPagesApiUpdateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_update_front_page_for_courses

> String wiki_pages_api_update_front_page_for_courses(course_id, wiki_pages_api_update_front_page_for_courses_request)
Update/create front page

Update the title or contents of the front page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**wiki_pages_api_update_front_page_for_courses_request** | Option<[**WikiPagesApiUpdateFrontPageForCoursesRequest**](WikiPagesApiUpdateFrontPageForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## wiki_pages_api_update_front_page_for_groups

> String wiki_pages_api_update_front_page_for_groups(group_id, wiki_pages_api_update_front_page_for_courses_request)
Update/create front page

Update the title or contents of the front page

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**wiki_pages_api_update_front_page_for_courses_request** | Option<[**WikiPagesApiUpdateFrontPageForCoursesRequest**](WikiPagesApiUpdateFrontPageForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

