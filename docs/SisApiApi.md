# \SisApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**sis_api_sis_assignments_for_accounts**](SisApiApi.md#sis_api_sis_assignments_for_accounts) | **GET** /accounts/{account_id}/assignments | Retrieve assignments enabled for grade export to SIS
[**sis_api_sis_assignments_for_courses**](SisApiApi.md#sis_api_sis_assignments_for_courses) | **GET** /courses/{course_id}/assignments | Retrieve assignments enabled for grade export to SIS



## sis_api_sis_assignments_for_accounts

> sis_api_sis_assignments_for_accounts(account_id, course_id, starts_before, ends_after, include)
Retrieve assignments enabled for grade export to SIS

Retrieve a list of published assignments flagged as \"post_to_sis\". See the Assignments API for more details on assignments. Assignment group and section information are included for convenience.  Each section includes course information for the origin course and the cross-listed course, if applicable. The `origin_course` is the course to which the section belongs or the course from which the section was cross-listed. Generally, the `origin_course` should be preferred when performing integration work. The `xlist_course` is provided for consistency and is only present when the section has been cross-listed. See Sections API and Courses Api for me details.  The `override` is only provided if the Differentiated Assignments course feature is turned on and the assignment has an override for that section. When there is an override for the assignment the override object's keys/values can be merged with the top level assignment object to create a view of the assignment object specific to that section. See Assignments api for more information on assignment overrides.  restricts to courses that start before this date (if they have a start date) restricts to courses that end after this date (if they have an end date) information to include.    \"student_overrides\":: returns individual student override information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The ID of the account to query. | [required] |
**course_id** | Option<**i32**> | The ID of the course to query. |  |
**starts_before** | Option<**String**> | When searching on an account, |  |
**ends_after** | Option<**String**> | When searching on an account, |  |
**include** | Option<**String**> | Array of additional |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## sis_api_sis_assignments_for_courses

> sis_api_sis_assignments_for_courses(course_id, course_id2, starts_before, ends_after, include)
Retrieve assignments enabled for grade export to SIS

Retrieve a list of published assignments flagged as \"post_to_sis\". See the Assignments API for more details on assignments. Assignment group and section information are included for convenience.  Each section includes course information for the origin course and the cross-listed course, if applicable. The `origin_course` is the course to which the section belongs or the course from which the section was cross-listed. Generally, the `origin_course` should be preferred when performing integration work. The `xlist_course` is provided for consistency and is only present when the section has been cross-listed. See Sections API and Courses Api for me details.  The `override` is only provided if the Differentiated Assignments course feature is turned on and the assignment has an override for that section. When there is an override for the assignment the override object's keys/values can be merged with the top level assignment object to create a view of the assignment object specific to that section. See Assignments api for more information on assignment overrides.  restricts to courses that start before this date (if they have a start date) restricts to courses that end after this date (if they have an end date) information to include.    \"student_overrides\":: returns individual student override information

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**course_id2** | Option<**i32**> | The ID of the course to query. |  |
**starts_before** | Option<**String**> | When searching on an account, |  |
**ends_after** | Option<**String**> | When searching on an account, |  |
**include** | Option<**String**> | Array of additional |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

