# \EnrollmentsApiApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**enrollments_api_accept**](EnrollmentsApiApi.md#enrollments_api_accept) | **POST** /courses/{course_id}/enrollments/{id}/accept | Accept Course Invitation
[**enrollments_api_bulk_enrollment**](EnrollmentsApiApi.md#enrollments_api_bulk_enrollment) | **POST** /accounts/{account_id}/bulk_enrollment | Enroll multiple users to one or more courses
[**enrollments_api_create_for_courses**](EnrollmentsApiApi.md#enrollments_api_create_for_courses) | **POST** /courses/{course_id}/enrollments | Enroll a user
[**enrollments_api_create_other**](EnrollmentsApiApi.md#enrollments_api_create_other) | **POST** /sections/{section_id}/enrollments | Enroll a user
[**enrollments_api_destroy**](EnrollmentsApiApi.md#enrollments_api_destroy) | **DELETE** /courses/{course_id}/enrollments/{id} | Conclude, deactivate, or delete an enrollment
[**enrollments_api_index_for_courses**](EnrollmentsApiApi.md#enrollments_api_index_for_courses) | **GET** /courses/{course_id}/enrollments | List enrollments
[**enrollments_api_index_for_users**](EnrollmentsApiApi.md#enrollments_api_index_for_users) | **GET** /users/{user_id}/enrollments | List enrollments
[**enrollments_api_index_other**](EnrollmentsApiApi.md#enrollments_api_index_other) | **GET** /sections/{section_id}/enrollments | List enrollments
[**enrollments_api_last_attended**](EnrollmentsApiApi.md#enrollments_api_last_attended) | **PUT** /courses/{course_id}/users/{user_id}/last_attended | Add last attended date
[**enrollments_api_reactivate**](EnrollmentsApiApi.md#enrollments_api_reactivate) | **PUT** /courses/{course_id}/enrollments/{id}/reactivate | Re-activate an enrollment
[**enrollments_api_reject**](EnrollmentsApiApi.md#enrollments_api_reject) | **POST** /courses/{course_id}/enrollments/{id}/reject | Reject Course Invitation
[**enrollments_api_show**](EnrollmentsApiApi.md#enrollments_api_show) | **GET** /accounts/{account_id}/enrollments/{id} | Enrollment by ID
[**enrollments_api_show_temporary_enrollment_status**](EnrollmentsApiApi.md#enrollments_api_show_temporary_enrollment_status) | **GET** /users/{user_id}/temporary_enrollment_status | Show Temporary Enrollment recipient and provider status



## enrollments_api_accept

> enrollments_api_accept(course_id, id)
Accept Course Invitation

accepts a pending course invitation for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_bulk_enrollment

> String enrollments_api_bulk_enrollment(account_id, enrollments_api_bulk_enrollment_request)
Enroll multiple users to one or more courses

Enrolls multiple users in one or more courses in a single operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**enrollments_api_bulk_enrollment_request** | Option<[**EnrollmentsApiBulkEnrollmentRequest**](EnrollmentsApiBulkEnrollmentRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/:account_id/bulk_enrollment \\   -X POST \\   -F 'user_ids[]=1' \\   -F 'user_ids[]=2' \\   -F 'course_ids[]=10' \\   -F 'course_ids[]=11' \\ ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_create_for_courses

> String enrollments_api_create_for_courses(course_id, enrollments_api_create_for_courses_request)
Enroll a user

Create a new user enrollment for a course or section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**enrollments_api_create_for_courses_request** | Option<[**EnrollmentsApiCreateForCoursesRequest**](EnrollmentsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_create_other

> String enrollments_api_create_other(section_id, enrollments_api_create_for_courses_request)
Enroll a user

Create a new user enrollment for a course or section.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**enrollments_api_create_for_courses_request** | Option<[**EnrollmentsApiCreateForCoursesRequest**](EnrollmentsApiCreateForCoursesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_destroy

> String enrollments_api_destroy(course_id, id, task)
Conclude, deactivate, or delete an enrollment

Conclude, deactivate, or delete an enrollment. If the +task+ argument isn't given, the enrollment will be concluded.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |
**task** | Option<**String**> | The action to take on the enrollment. When inactive, a user will still appear in the course roster to admins, but be unable to participate. (\"inactivate\" and \"deactivate\" are equivalent tasks) |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_index_for_courses

> serde_json::Value enrollments_api_index_for_courses(course_id, r#type, role, state, include, user_id, grading_period_id, enrollment_term_id, sis_account_id, sis_course_id, sis_section_id, sis_user_id, created_for_sis_id, page, per_page)
List enrollments

Depending on the URL given, return a paginated list of either (1) all of the enrollments in a course, (2) all of the enrollments in a section or (3) all of a user's enrollments. This includes student, teacher, TA, and observer enrollments.  If a user has multiple enrollments in a context (e.g. as a teacher and a student or in multiple course sections), each enrollment will be listed separately.  note: Currently, only a root level admin user can return other users' enrollments. A user can, however, return his/her own enrollments.  Enrollments scoped to a course context will include inactive states by default if the caller has account admin authorization and the state[] parameter is omitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**r#type** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment types to return. Accepted values are 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment', and 'ObserverEnrollment.' If omitted, all enrollment types are returned. This argument is ignored if `role` is given. |  |
**role** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment roles to return. Accepted values include course-level roles created by the {api:RoleOverridesController#add_role Add Role API} as well as the base enrollment types accepted by the `type` argument above. |  |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"invited\"|\"creation_pending\"|\"deleted\"|\"rejected\"|\"completed\"|\"inactive\"|\"current_and_invited\"|\"current_and_future\"|\"current_future_and_restricted\"|\"current_and_concluded\"] Filter by enrollment state. If omitted, 'active' and 'invited' enrollments are returned. The following synthetic states are supported only when querying a user's enrollments (either via user_id argument or via user enrollments endpoint): +current_and_invited+, +current_and_future+, +current_future_and_restricted+, +current_and_concluded+ |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"|\"group_ids\"|\"locked\"|\"observed_users\"|\"can_be_removed\"|\"uuid\"|\"current_points\"] Array of additional information to include on the enrollment or user records. \"avatar_url\" and \"group_ids\" will be returned on the user record. If \"current_points\" is specified, the fields \"current_points\" and (if the caller has permissions to manage grades) \"unposted_current_points\" will be included in the \"grades\" hash for student enrollments. |  |
**user_id** | Option<**String**> | Filter by user_id (only valid for course or section enrollment queries). If set to the current user's id, this is a way to determine if the user has any enrollments in the course or section, independent of whether the user has permission to view other people on the roster. |  |
**grading_period_id** | Option<**i32**> | Return grades for the given grading_period.  If this parameter is not specified, the returned grades will be for the whole course. |  |
**enrollment_term_id** | Option<**i32**> | Returns only enrollments for the specified enrollment term. This parameter only applies to the user enrollments path. May pass the ID from the enrollment terms api or the SIS id prepended with 'sis_term_id:'. |  |
**sis_account_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS account ID(s). Does not look into sub_accounts. May pass in array or string. |  |
**sis_course_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments matching the specified SIS course ID(s). May pass in array or string. |  |
**sis_section_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only section enrollments matching the specified SIS section ID(s). May pass in array or string. |  |
**sis_user_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS user ID(s). May pass in array or string. |  |
**created_for_sis_id** | Option<[**Vec<String>**](String.md)> | [Boolean] If sis_user_id is present and created_for_sis_id is true, Returns only enrollments for the specified SIS ID(s). If a user has two sis_id's, one enrollment may be created using one of the two ids. This would limit the enrollments returned from the endpoint to enrollments that were created from a sis_import with that sis_user_id |  |
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


## enrollments_api_index_for_users

> serde_json::Value enrollments_api_index_for_users(user_id, r#type, role, state, include, user_id2, grading_period_id, enrollment_term_id, sis_account_id, sis_course_id, sis_section_id, sis_user_id, created_for_sis_id, page, per_page)
List enrollments

Depending on the URL given, return a paginated list of either (1) all of the enrollments in a course, (2) all of the enrollments in a section or (3) all of a user's enrollments. This includes student, teacher, TA, and observer enrollments.  If a user has multiple enrollments in a context (e.g. as a teacher and a student or in multiple course sections), each enrollment will be listed separately.  note: Currently, only a root level admin user can return other users' enrollments. A user can, however, return his/her own enrollments.  Enrollments scoped to a course context will include inactive states by default if the caller has account admin authorization and the state[] parameter is omitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**r#type** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment types to return. Accepted values are 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment', and 'ObserverEnrollment.' If omitted, all enrollment types are returned. This argument is ignored if `role` is given. |  |
**role** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment roles to return. Accepted values include course-level roles created by the {api:RoleOverridesController#add_role Add Role API} as well as the base enrollment types accepted by the `type` argument above. |  |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"invited\"|\"creation_pending\"|\"deleted\"|\"rejected\"|\"completed\"|\"inactive\"|\"current_and_invited\"|\"current_and_future\"|\"current_future_and_restricted\"|\"current_and_concluded\"] Filter by enrollment state. If omitted, 'active' and 'invited' enrollments are returned. The following synthetic states are supported only when querying a user's enrollments (either via user_id argument or via user enrollments endpoint): +current_and_invited+, +current_and_future+, +current_future_and_restricted+, +current_and_concluded+ |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"|\"group_ids\"|\"locked\"|\"observed_users\"|\"can_be_removed\"|\"uuid\"|\"current_points\"] Array of additional information to include on the enrollment or user records. \"avatar_url\" and \"group_ids\" will be returned on the user record. If \"current_points\" is specified, the fields \"current_points\" and (if the caller has permissions to manage grades) \"unposted_current_points\" will be included in the \"grades\" hash for student enrollments. |  |
**user_id2** | Option<**String**> | Filter by user_id (only valid for course or section enrollment queries). If set to the current user's id, this is a way to determine if the user has any enrollments in the course or section, independent of whether the user has permission to view other people on the roster. |  |
**grading_period_id** | Option<**i32**> | Return grades for the given grading_period.  If this parameter is not specified, the returned grades will be for the whole course. |  |
**enrollment_term_id** | Option<**i32**> | Returns only enrollments for the specified enrollment term. This parameter only applies to the user enrollments path. May pass the ID from the enrollment terms api or the SIS id prepended with 'sis_term_id:'. |  |
**sis_account_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS account ID(s). Does not look into sub_accounts. May pass in array or string. |  |
**sis_course_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments matching the specified SIS course ID(s). May pass in array or string. |  |
**sis_section_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only section enrollments matching the specified SIS section ID(s). May pass in array or string. |  |
**sis_user_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS user ID(s). May pass in array or string. |  |
**created_for_sis_id** | Option<[**Vec<String>**](String.md)> | [Boolean] If sis_user_id is present and created_for_sis_id is true, Returns only enrollments for the specified SIS ID(s). If a user has two sis_id's, one enrollment may be created using one of the two ids. This would limit the enrollments returned from the endpoint to enrollments that were created from a sis_import with that sis_user_id |  |
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


## enrollments_api_index_other

> serde_json::Value enrollments_api_index_other(section_id, r#type, role, state, include, user_id, grading_period_id, enrollment_term_id, sis_account_id, sis_course_id, sis_section_id, sis_user_id, created_for_sis_id, page, per_page)
List enrollments

Depending on the URL given, return a paginated list of either (1) all of the enrollments in a course, (2) all of the enrollments in a section or (3) all of a user's enrollments. This includes student, teacher, TA, and observer enrollments.  If a user has multiple enrollments in a context (e.g. as a teacher and a student or in multiple course sections), each enrollment will be listed separately.  note: Currently, only a root level admin user can return other users' enrollments. A user can, however, return his/her own enrollments.  Enrollments scoped to a course context will include inactive states by default if the caller has account admin authorization and the state[] parameter is omitted.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**section_id** | **String** | ID of the section | [required] |
**r#type** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment types to return. Accepted values are 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'DesignerEnrollment', and 'ObserverEnrollment.' If omitted, all enrollment types are returned. This argument is ignored if `role` is given. |  |
**role** | Option<[**Vec<String>**](String.md)> | [String] A list of enrollment roles to return. Accepted values include course-level roles created by the {api:RoleOverridesController#add_role Add Role API} as well as the base enrollment types accepted by the `type` argument above. |  |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"invited\"|\"creation_pending\"|\"deleted\"|\"rejected\"|\"completed\"|\"inactive\"|\"current_and_invited\"|\"current_and_future\"|\"current_future_and_restricted\"|\"current_and_concluded\"] Filter by enrollment state. If omitted, 'active' and 'invited' enrollments are returned. The following synthetic states are supported only when querying a user's enrollments (either via user_id argument or via user enrollments endpoint): +current_and_invited+, +current_and_future+, +current_future_and_restricted+, +current_and_concluded+ |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"avatar_url\"|\"group_ids\"|\"locked\"|\"observed_users\"|\"can_be_removed\"|\"uuid\"|\"current_points\"] Array of additional information to include on the enrollment or user records. \"avatar_url\" and \"group_ids\" will be returned on the user record. If \"current_points\" is specified, the fields \"current_points\" and (if the caller has permissions to manage grades) \"unposted_current_points\" will be included in the \"grades\" hash for student enrollments. |  |
**user_id** | Option<**String**> | Filter by user_id (only valid for course or section enrollment queries). If set to the current user's id, this is a way to determine if the user has any enrollments in the course or section, independent of whether the user has permission to view other people on the roster. |  |
**grading_period_id** | Option<**i32**> | Return grades for the given grading_period.  If this parameter is not specified, the returned grades will be for the whole course. |  |
**enrollment_term_id** | Option<**i32**> | Returns only enrollments for the specified enrollment term. This parameter only applies to the user enrollments path. May pass the ID from the enrollment terms api or the SIS id prepended with 'sis_term_id:'. |  |
**sis_account_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS account ID(s). Does not look into sub_accounts. May pass in array or string. |  |
**sis_course_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments matching the specified SIS course ID(s). May pass in array or string. |  |
**sis_section_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only section enrollments matching the specified SIS section ID(s). May pass in array or string. |  |
**sis_user_id** | Option<[**Vec<String>**](String.md)> | [String] Returns only enrollments for the specified SIS user ID(s). May pass in array or string. |  |
**created_for_sis_id** | Option<[**Vec<String>**](String.md)> | [Boolean] If sis_user_id is present and created_for_sis_id is true, Returns only enrollments for the specified SIS ID(s). If a user has two sis_id's, one enrollment may be created using one of the two ids. This would limit the enrollments returned from the endpoint to enrollments that were created from a sis_import with that sis_user_id |  |
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


## enrollments_api_last_attended

> String enrollments_api_last_attended(course_id, user_id, enrollments_api_last_attended_request)
Add last attended date

Add last attended date to student enrollment in course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |
**enrollments_api_last_attended_request** | Option<[**EnrollmentsApiLastAttendedRequest**](EnrollmentsApiLastAttendedRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/:course_id/user/:user_id/last_attended\"   -X PUT => date=\"Thu%20Dec%2021%202017%2000:00:00%20GMT-0700%20(MST) ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_reactivate

> String enrollments_api_reactivate(course_id, id)
Re-activate an enrollment

Activates an inactive enrollment

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_reject

> enrollments_api_reject(course_id, id)
Reject Course Invitation

rejects a pending course invitation for the current user

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_show

> String enrollments_api_show(account_id, id)
Enrollment by ID

Get an Enrollment object by Enrollment ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | The ID of the enrollment object | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## enrollments_api_show_temporary_enrollment_status

> enrollments_api_show_temporary_enrollment_status(user_id, account_id)
Show Temporary Enrollment recipient and provider status

Returns a JSON Object containing the temporary enrollment status for a user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**account_id** | Option<**String**> | The ID of the account to check for temporary enrollment status. Defaults to the domain root account if not provided. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

