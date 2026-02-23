# \SectionsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sections_create**](SectionsApi.md#sections_create) | **POST** /courses/{course_id}/sections | Create course section
[**sections_crosslist**](SectionsApi.md#sections_crosslist) | **POST** /sections/{id}/crosslist/{new_course_id} | Cross-list a Section
[**sections_destroy**](SectionsApi.md#sections_destroy) | **DELETE** /sections/{id} | Delete a section
[**sections_index**](SectionsApi.md#sections_index) | **GET** /courses/{course_id}/sections | List course sections
[**sections_show_for_courses**](SectionsApi.md#sections_show_for_courses) | **GET** /courses/{course_id}/sections/{id} | Get section information
[**sections_show_other**](SectionsApi.md#sections_show_other) | **GET** /sections/{id} | Get section information
[**sections_uncrosslist**](SectionsApi.md#sections_uncrosslist) | **DELETE** /sections/{id}/crosslist | De-cross-list a Section
[**sections_update**](SectionsApi.md#sections_update) | **PUT** /sections/{id} | Edit a section
[**sections_users**](SectionsApi.md#sections_users) | **GET** /sections/{id}/users | List section's users



## sections_create

> String sections_create(course_id, sections_create_request)
Create course section

Creates a new section for this course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**sections_create_request** | Option<[**SectionsCreateRequest**](SectionsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_crosslist

> String sections_crosslist(id, new_course_id, sections_crosslist_request)
Cross-list a Section

Move the Section to another course.  The new course may be in a different account (department), but must belong to the same root account (institution).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**new_course_id** | **String** | Scope response to new_course_id | [required] |
**sections_crosslist_request** | Option<[**SectionsCrosslistRequest**](SectionsCrosslistRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_destroy

> String sections_destroy(id)
Delete a section

Delete an existing section.  Returns the former Section.

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


## sections_index

> models::Section sections_index(course_id, include, search_term, page, per_page)
List course sections

A paginated list of the list of sections for this course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"students\"|\"avatar_url\"|\"enrollments\"|\"total_students\"|\"passback_status\"|\"permissions\"] - \"students\": Associations to include with the group. Note: this is only   available if you have permission to view users or grades in the course - \"avatar_url\": Include the avatar URLs for students returned. - \"enrollments\": If 'students' is also included, return the section   enrollment for each student - \"total_students\": Returns the total amount of active and invited students   for the course section - \"passback_status\": Include the grade passback status. - \"permissions\": Include whether section grants :manage_calendar permission   to the caller |  |
**search_term** | Option<**String**> | When included, searches course sections for the term. Returns only matching results. Term must be at least 2 characters. |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Section**](Section.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_show_for_courses

> String sections_show_for_courses(course_id, id, include)
Get section information

Gets details about a specific section

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"students\"|\"avatar_url\"|\"enrollments\"|\"total_students\"|\"passback_status\"|\"permissions\"] - \"students\": Associations to include with the group. Note: this is only   available if you have permission to view users or grades in the course - \"avatar_url\": Include the avatar URLs for students returned. - \"enrollments\": If 'students' is also included, return the section   enrollment for each student - \"total_students\": Returns the total amount of active and invited students   for the course section - \"passback_status\": Include the grade passback status. - \"permissions\": Include whether section grants :manage_calendar permission   to the caller |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_show_other

> String sections_show_other(id, include)
Get section information

Gets details about a specific section

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"students\"|\"avatar_url\"|\"enrollments\"|\"total_students\"|\"passback_status\"|\"permissions\"] - \"students\": Associations to include with the group. Note: this is only   available if you have permission to view users or grades in the course - \"avatar_url\": Include the avatar URLs for students returned. - \"enrollments\": If 'students' is also included, return the section   enrollment for each student - \"total_students\": Returns the total amount of active and invited students   for the course section - \"passback_status\": Include the grade passback status. - \"permissions\": Include whether section grants :manage_calendar permission   to the caller |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_uncrosslist

> String sections_uncrosslist(id, override_sis_stickiness)
De-cross-list a Section

Undo cross-listing of a Section, returning it to its original course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**override_sis_stickiness** | Option<**bool**> | Default is true. If false, any fields containing “sticky” changes will not be updated. See SIS CSV Format documentation for information on which fields can have SIS stickiness |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_update

> String sections_update(id, sections_update_request)
Edit a section

Modify an existing section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**sections_update_request** | Option<[**SectionsUpdateRequest**](SectionsUpdateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sections_users

> models::User sections_users(id, search_term, include, exclude_inactive, enrollment_type, page, per_page)
List section's users

Returns a paginated list of users in the section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. Must be at least 2 characters. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"] \"avatar_url\": Include users' avatar_urls. |  |
**exclude_inactive** | Option<**bool**> | Whether to filter out inactive users from the results. Defaults to false unless explicitly provided. |  |
**enrollment_type** | Option<**String**> | When set, only return users with the specified enrollment type for the given section. |  |
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

