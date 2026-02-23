# \FavoritesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**favorites_add_favorite_course**](FavoritesApi.md#favorites_add_favorite_course) | **POST** /users/self/favorites/courses/{id} | Add course to favorites
[**favorites_add_favorite_groups**](FavoritesApi.md#favorites_add_favorite_groups) | **POST** /users/self/favorites/groups/{id} | Add group to favorites
[**favorites_list_favorite_courses**](FavoritesApi.md#favorites_list_favorite_courses) | **GET** /users/self/favorites/courses | List favorite courses
[**favorites_list_favorite_groups**](FavoritesApi.md#favorites_list_favorite_groups) | **GET** /users/self/favorites/groups | List favorite groups
[**favorites_remove_favorite_course**](FavoritesApi.md#favorites_remove_favorite_course) | **DELETE** /users/self/favorites/courses/{id} | Remove course from favorites
[**favorites_remove_favorite_groups**](FavoritesApi.md#favorites_remove_favorite_groups) | **DELETE** /users/self/favorites/groups/{id} | Remove group from favorites
[**favorites_reset_course_favorites**](FavoritesApi.md#favorites_reset_course_favorites) | **DELETE** /users/self/favorites/courses | Reset course favorites
[**favorites_reset_groups_favorites**](FavoritesApi.md#favorites_reset_groups_favorites) | **DELETE** /users/self/favorites/groups | Reset group favorites



## favorites_add_favorite_course

> String favorites_add_favorite_course(id)
Add course to favorites

Add a course to the current user's favorites.  If the course is already in the user's favorites, nothing happens. Canvas for Elementary subject and homeroom courses can be added to favorites, but this has no effect in the UI.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or SIS ID of the course to add.  The current user must be registered in the course. | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_add_favorite_groups

> String favorites_add_favorite_groups(id)
Add group to favorites

Add a group to the current user's favorites.  If the group is already in the user's favorites, nothing happens.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The ID or SIS ID of the group to add.  The current user must be a member of the group. | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_list_favorite_courses

> models::Course favorites_list_favorite_courses(exclude_blueprint_courses)
List favorite courses

Retrieve the paginated list of favorite courses for the current user. If the user has not chosen any favorites, then a selection of currently enrolled courses will be returned.  See the {api:CoursesController#index List courses API} for details on accepted include[] parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**exclude_blueprint_courses** | Option<**bool**> | When set, only return courses that are not configured as blueprint courses. |  |

### Return type

[**models::Course**](Course.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_list_favorite_groups

> models::Group favorites_list_favorite_groups()
List favorite groups

Retrieve the paginated list of favorite groups for the current user. If the user has not chosen any favorites, then a selection of groups that the user is a member of will be returned.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::Group**](Group.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_remove_favorite_course

> String favorites_remove_favorite_course(id)
Remove course from favorites

Remove a course from the current user's favorites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the ID or SIS ID of the course to remove | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_remove_favorite_groups

> String favorites_remove_favorite_groups(id)
Remove group from favorites

Remove a group from the current user's favorites.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | the ID or SIS ID of the group to remove | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## favorites_reset_course_favorites

> favorites_reset_course_favorites()
Reset course favorites

Reset the current user's course favorites to the default automatically generated list of enrolled courses

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


## favorites_reset_groups_favorites

> favorites_reset_groups_favorites()
Reset group favorites

Reset the current user's group favorites to the default automatically generated list of enrolled group

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

