# \CoursesApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**courses_activity_stream**](CoursesApi.md#courses_activity_stream) | **GET** /courses/{course_id}/activity_stream | Course activity stream
[**courses_activity_stream_summary**](CoursesApi.md#courses_activity_stream_summary) | **GET** /courses/{course_id}/activity_stream/summary | Course activity stream summary
[**courses_api_settings**](CoursesApi.md#courses_api_settings) | **GET** /courses/{course_id}/settings | Get course settings
[**courses_batch_update**](CoursesApi.md#courses_batch_update) | **PUT** /accounts/{account_id}/courses | Update courses
[**courses_bulk_user_progress**](CoursesApi.md#courses_bulk_user_progress) | **GET** /courses/{course_id}/bulk_user_progress | Get bulk user progress
[**courses_content_share_users**](CoursesApi.md#courses_content_share_users) | **GET** /courses/{course_id}/content_share_users | Search for content share users
[**courses_create**](CoursesApi.md#courses_create) | **POST** /accounts/{account_id}/courses | Create a new course
[**courses_create_file**](CoursesApi.md#courses_create_file) | **POST** /courses/{course_id}/files | Upload a file
[**courses_destroy**](CoursesApi.md#courses_destroy) | **DELETE** /courses/{id} | Delete/Conclude a course
[**courses_dismiss_migration_limitation_msg**](CoursesApi.md#courses_dismiss_migration_limitation_msg) | **POST** /courses/{id}/dismiss_migration_limitation_message | Remove quiz migration alert
[**courses_effective_due_dates**](CoursesApi.md#courses_effective_due_dates) | **GET** /courses/{course_id}/effective_due_dates | Get effective due dates
[**courses_index**](CoursesApi.md#courses_index) | **GET** /courses | List your courses
[**courses_permissions**](CoursesApi.md#courses_permissions) | **GET** /courses/{course_id}/permissions | Permissions
[**courses_preview_html**](CoursesApi.md#courses_preview_html) | **POST** /courses/{course_id}/preview_html | Preview processed html
[**courses_recent_students**](CoursesApi.md#courses_recent_students) | **GET** /courses/{course_id}/recent_students | List recently logged in students
[**courses_reset_content**](CoursesApi.md#courses_reset_content) | **POST** /courses/{course_id}/reset_content | Reset a course
[**courses_restore_version**](CoursesApi.md#courses_restore_version) | **POST** /courses/{course_id}/restore/{version_id} | Restore course version
[**courses_show_for_accounts**](CoursesApi.md#courses_show_for_accounts) | **GET** /accounts/{account_id}/courses/{id} | Get a single course
[**courses_show_other**](CoursesApi.md#courses_show_other) | **GET** /courses/{id} | Get a single course
[**courses_student_view_student**](CoursesApi.md#courses_student_view_student) | **GET** /courses/{course_id}/student_view_student | Return test student for course
[**courses_students**](CoursesApi.md#courses_students) | **GET** /courses/{course_id}/students | List students
[**courses_todo_items**](CoursesApi.md#courses_todo_items) | **GET** /courses/{course_id}/todo | Course TODO items
[**courses_update**](CoursesApi.md#courses_update) | **PUT** /courses/{id} | Update a course
[**courses_update_settings**](CoursesApi.md#courses_update_settings) | **PUT** /courses/{course_id}/settings | Update course settings
[**courses_user**](CoursesApi.md#courses_user) | **GET** /courses/{course_id}/users/{id} | Get single user
[**courses_user_index**](CoursesApi.md#courses_user_index) | **GET** /users/{user_id}/courses | List courses for a user
[**courses_user_progress**](CoursesApi.md#courses_user_progress) | **GET** /courses/{course_id}/users/{user_id}/progress | Get user progress
[**courses_users_for_courses**](CoursesApi.md#courses_users_for_courses) | **GET** /courses/{course_id}/users | List users in course
[**courses_users_for_courses2**](CoursesApi.md#courses_users_for_courses2) | **GET** /courses/{course_id}/search_users | List users in course



## courses_activity_stream

> courses_activity_stream(course_id)
Course activity stream

Returns the current user's course-specific activity stream, paginated.  For full documentation, see the API documentation for the user activity stream, in the user api.

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


## courses_activity_stream_summary

> courses_activity_stream_summary(course_id)
Course activity stream summary

Returns a summary of the current user's course-specific activity stream.  For full documentation, see the API documentation for the user activity stream summary, in the user api.

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


## courses_api_settings

> courses_api_settings(course_id)
Get course settings

Returns some of a course's settings.

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
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_batch_update

> String courses_batch_update(account_id, courses_batch_update_request)
Update courses

Update multiple courses in an account.  Operates asynchronously; use the {api:ProgressController#show progress endpoint} to query the status of an operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**courses_batch_update_request** | Option<[**CoursesBatchUpdateRequest**](CoursesBatchUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/accounts/<account_id>/courses \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'event=offer' \\   -d 'course_ids[]=1' \\   -d 'course_ids[]=2' ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_bulk_user_progress

> courses_bulk_user_progress(course_id, page, per_page)
Get bulk user progress

Returns progress information for all users enrolled in the given course.  You must be a user who has permission to view all grades in the course (such as a teacher or administrator).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_content_share_users

> models::User courses_content_share_users(course_id, search_term, page, per_page)
Search for content share users

Returns a paginated list of users you can share content with.  Requires the content share feature and the user must have the manage content permission for the course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**search_term** | **String** | Term used to find users.  Will search available share users with the search term in their name. | [required] |
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


## courses_create

> String courses_create(account_id, courses_create_request)
Create a new course

Create a new course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**courses_create_request** | Option<[**CoursesCreateRequest**](CoursesCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_create_file

> courses_create_file(course_id)
Upload a file

Upload a file to the course.  This API endpoint is the first step in uploading a file to a course. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  Only those with the \"Manage Files\" permission on a course can upload files to the course. By default, this is Teachers, TAs and Designers.

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


## courses_destroy

> courses_destroy(id, event)
Delete/Conclude a course

Delete or conclude an existing course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**event** | **String** | The action to take on the course. | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_dismiss_migration_limitation_msg

> courses_dismiss_migration_limitation_msg(id)
Remove quiz migration alert

Remove alert about the limitations of quiz migrations that is displayed to a user in a course  you must be logged in to use this endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_effective_due_dates

> courses_effective_due_dates(course_id, assignment_ids)
Get effective due dates

For each assignment in the course, returns each assigned student's ID and their corresponding due date along with some grading period data. Returns a collection with keys representing assignment IDs and values as a collection containing keys representing student IDs and values representing the student's effective due_at, the grading_period_id of which the due_at falls in, and whether or not the grading period is closed (in_closed_grading_period)  The list of assignment IDs for which effective student due dates are requested. If not provided, all assignments in the course will be used.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**assignment_ids** | Option<[**Vec<String>**](String.md)> | [Optional, String] |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_index

> models::Course courses_index(enrollment_type, enrollment_role, enrollment_role_id, enrollment_state, exclude_blueprint_courses, include, state, page, per_page)
List your courses

Returns the paginated list of active courses for the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enrollment_type** | Option<**String**> | When set, only return courses where the user is enrolled as this type. For example, set to \"teacher\" to return only courses where the user is enrolled as a Teacher.  This argument is ignored if enrollment_role is given. |  |
**enrollment_role** | Option<**String**> | Deprecated When set, only return courses where the user is enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a base role type of 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**enrollment_role_id** | Option<**i32**> | When set, only return courses where the user is enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a built_in role type of 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**enrollment_state** | Option<**String**> | When set, only return courses where the user has an enrollment with the given state. This will respect section/course/term date overrides. |  |
**exclude_blueprint_courses** | Option<**bool**> | When set, only return courses that are not configured as blueprint courses. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"needs_grading_count\"|\"syllabus_body\"|\"public_description\"|\"total_scores\"|\"current_grading_period_scores\"|\"grading_periods\"|\"term\"|\"account\"|\"course_progress\"|\"sections\"|\"storage_quota_used_mb\"|\"total_students\"|\"passback_status\"|\"favorites\"|\"teachers\"|\"observed_users\"|\"course_image\"|\"banner_image\"|\"concluded\"|\"post_manually\"] - \"needs_grading_count\": Optional information to include with each Course.   When needs_grading_count is given, and the current user has grading   rights, the total number of submissions needing grading for all   assignments is returned. - \"syllabus_body\": Optional information to include with each Course.   When syllabus_body is given the user-generated html for the course   syllabus is returned. - \"public_description\": Optional information to include with each Course.   When public_description is given the user-generated text for the course   public description is returned. - \"total_scores\": Optional information to include with each Course.   When total_scores is given, any student enrollments will also   include the fields 'computed_current_score', 'computed_final_score',   'computed_current_grade', and 'computed_final_grade', as well as (if   the user has permission) 'unposted_current_score',   'unposted_final_score', 'unposted_current_grade', and   'unposted_final_grade' (see Enrollment documentation for more   information on these fields). This argument is ignored if the course is   configured to hide final grades. - \"current_grading_period_scores\": Optional information to include with   each Course. When current_grading_period_scores is given and total_scores   is given, any student enrollments will also include the fields   'has_grading_periods',   'totals_for_all_grading_periods_option', 'current_grading_period_title',   'current_grading_period_id', current_period_computed_current_score',   'current_period_computed_final_score',   'current_period_computed_current_grade', and   'current_period_computed_final_grade', as well as (if the user has permission)   'current_period_unposted_current_score',   'current_period_unposted_final_score',   'current_period_unposted_current_grade', and   'current_period_unposted_final_grade' (see Enrollment documentation for   more information on these fields). In addition, when this argument is   passed, the course will have a 'has_grading_periods' attribute   on it. This argument is ignored if the total_scores argument is not   included. If the course is configured to hide final grades, the   following fields are not returned:   'totals_for_all_grading_periods_option',   'current_period_computed_current_score',   'current_period_computed_final_score',   'current_period_computed_current_grade',   'current_period_computed_final_grade',   'current_period_unposted_current_score',   'current_period_unposted_final_score',   'current_period_unposted_current_grade', and   'current_period_unposted_final_grade' - \"grading_periods\": Optional information to include with each Course. When   grading_periods is given, a list of the grading periods associated with   each course is returned. - \"term\": Optional information to include with each Course. When   term is given, the information for the enrollment term for each course   is returned. - \"account\": Optional information to include with each Course. When   account is given, the account json for each course is returned. - \"course_progress\": Optional information to include with each Course.   When course_progress is given, each course will include a   'course_progress' object with the fields: 'requirement_count', an integer   specifying the total number of requirements in the course,   'requirement_completed_count', an integer specifying the total number of   requirements in this course that have been completed, and   'next_requirement_url', a string url to the next requirement item, and   'completed_at', the date the course was completed (null if incomplete).   'next_requirement_url' will be null if all requirements have been   completed or the current module does not require sequential progress.   \"course_progress\" will return an error message if the course is not   module based or the user is not enrolled as a student in the course. - \"sections\": Section enrollment information to include with each Course.   Returns an array of hashes containing the section ID (id), section name   (name), start and end dates (start_at, end_at), as well as the enrollment   type (enrollment_role, e.g. 'StudentEnrollment'). - \"storage_quota_used_mb\": The amount of storage space used by the files in this course - \"total_students\": Optional information to include with each Course.   Returns an integer for the total amount of active and invited students. - \"passback_status\": Include the grade passback_status - \"favorites\": Optional information to include with each Course.   Indicates if the user has marked the course as a favorite course. - \"teachers\": Teacher information to include with each Course.   Returns an array of hashes containing the {api:Users:UserDisplay UserDisplay} information   for each teacher in the course. - \"observed_users\": Optional information to include with each Course.   Will include data for observed users if the current user has an   observer enrollment. - \"tabs\": Optional information to include with each Course.   Will include the list of tabs configured for each course.  See the   {api:TabsController#index List available tabs API} for more information. - \"course_image\": Optional information to include with each Course. Returns course   image url if a course image has been set. - \"banner_image\": Optional information to include with each Course. Returns course   banner image url if the course is a Canvas for Elementary subject and a banner   image has been set. - \"concluded\": Optional information to include with each Course. Indicates whether   the course has been concluded, taking course and term dates into account. - \"post_manually\": Optional information to include with each Course. Returns true if   the course post policy is set to Manually post grades. Returns false if the the course   post policy is set to Automatically post grades. |  |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"unpublished\"|\"available\"|\"completed\"|\"deleted\"] If set, only return courses that are in the given state(s). By default, \"available\" is returned for students and observers, and anything except \"deleted\", for all other enrollment types |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Course**](Course.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_permissions

> courses_permissions(course_id, permissions)
Permissions

Returns permission information for the calling user in the given course. See also the {api:AccountsController#permissions Account} and {api:GroupsController#permissions Group} counterparts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**permissions** | Option<[**Vec<String>**](String.md)> | [String] List of permissions to check against the authenticated user. Permission names are documented in the {api:RoleOverridesController#manageable_permissions List assignable permissions} endpoint. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_preview_html

> courses_preview_html(course_id, courses_preview_html_request)
Preview processed html

Preview html content processed for this course

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**courses_preview_html_request** | Option<[**CoursesPreviewHtmlRequest**](CoursesPreviewHtmlRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/preview_html \\      -F 'html=<p><badhtml></badhtml>processed html</p>' \\      -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_recent_students

> models::User courses_recent_students(course_id, page, per_page)
List recently logged in students

Returns the paginated list of users in this course, ordered by how recently they have logged in. The records include the 'last_login' field which contains a timestamp of the last time that user logged into canvas.  The querying user must have the 'View usage reports' permission.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
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


## courses_reset_content

> String courses_reset_content(course_id)
Reset a course

Deletes the current course, and creates a new equivalent course with no content, but all sections and users moved over.

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


## courses_restore_version

> String courses_restore_version(course_id, version_id)
Restore course version

Restore a course to a prior version.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**version_id** | **String** | The version to restore to (use the syllabus_versions include parameter in the course show API to see available versions) | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_show_for_accounts

> String courses_show_for_accounts(account_id, id, include, teacher_limit)
Get a single course

Return information on a single course.  Accepts the same include[] parameters as the list action plus:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | ID of the account | [required] |
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"needs_grading_count\"|\"syllabus_body\"|\"public_description\"|\"total_scores\"|\"current_grading_period_scores\"|\"term\"|\"account\"|\"course_progress\"|\"sections\"|\"storage_quota_used_mb\"|\"total_students\"|\"passback_status\"|\"favorites\"|\"teachers\"|\"observed_users\"|\"all_courses\"|\"permissions\"|\"course_image\"|\"banner_image\"|\"concluded\"|\"lti_context_id\"|\"post_manually\"] - \"all_courses\": Also search recently deleted courses. - \"permissions\": Include permissions the current user has   for the course. - \"observed_users\": Include observed users in the enrollments - \"course_image\": Include course image url if a course image has been set - \"banner_image\": Include course banner image url if the course is a Canvas for   Elementary subject and a banner image has been set - \"concluded\": Optional information to include with Course. Indicates whether   the course has been concluded, taking course and term dates into account. - \"lti_context_id\": Include course LTI tool id. - \"post_manually\": Include course post policy. If the post policy is manually post grades,   the value will be true. If the post policy is automatically post grades, the value will be false. |  |
**teacher_limit** | Option<**i32**> | The maximum number of teacher enrollments to show. If the course contains more teachers than this, instead of giving the teacher enrollments, the count of teachers will be given under a _teacher_count_ key. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_show_other

> String courses_show_other(id, include, teacher_limit)
Get a single course

Return information on a single course.  Accepts the same include[] parameters as the list action plus:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"needs_grading_count\"|\"syllabus_body\"|\"public_description\"|\"total_scores\"|\"current_grading_period_scores\"|\"term\"|\"account\"|\"course_progress\"|\"sections\"|\"storage_quota_used_mb\"|\"total_students\"|\"passback_status\"|\"favorites\"|\"teachers\"|\"observed_users\"|\"all_courses\"|\"permissions\"|\"course_image\"|\"banner_image\"|\"concluded\"|\"lti_context_id\"|\"post_manually\"] - \"all_courses\": Also search recently deleted courses. - \"permissions\": Include permissions the current user has   for the course. - \"observed_users\": Include observed users in the enrollments - \"course_image\": Include course image url if a course image has been set - \"banner_image\": Include course banner image url if the course is a Canvas for   Elementary subject and a banner image has been set - \"concluded\": Optional information to include with Course. Indicates whether   the course has been concluded, taking course and term dates into account. - \"lti_context_id\": Include course LTI tool id. - \"post_manually\": Include course post policy. If the post policy is manually post grades,   the value will be true. If the post policy is automatically post grades, the value will be false. |  |
**teacher_limit** | Option<**i32**> | The maximum number of teacher enrollments to show. If the course contains more teachers than this, instead of giving the teacher enrollments, the count of teachers will be given under a _teacher_count_ key. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_student_view_student

> String courses_student_view_student(course_id)
Return test student for course

Returns information for a test student in this course. Creates a test student if one does not already exist for the course. The caller must have permission to access the course's student view.

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


## courses_students

> models::User courses_students(course_id)
List students

Returns the paginated list of students enrolled in this course.  DEPRECATED: Please use the {api:CoursesController#users course users} endpoint and pass \"student\" as the enrollment_type.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_todo_items

> courses_todo_items(course_id, page, per_page)
Course TODO items

Returns the current user's course-specific todo items.  For full documentation, see the API documentation for the user todo items, in the user api.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_update

> courses_update(id, courses_update_request)
Update a course

Update an existing course.  Arguments are the same as Courses#create, with a few exceptions (enroll_me).  If a user has content management rights, but not full course editing rights, the only attribute editable through this endpoint will be \"syllabus_body\"  If an account has set prevent_course_availability_editing_by_teachers, a teacher cannot change +course[start_at]+, +course[conclude_at]+, or +course[restrict_enrollments_to_course_dates]+ here.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**courses_update_request** | Option<[**CoursesUpdateRequest**](CoursesUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id> \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'course[name]=New course name' \\   -d 'course[start_at]=2012-05-05T00:00:00Z' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_update_settings

> courses_update_settings(course_id, courses_update_settings_request)
Update course settings

Can update the following course settings:

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**courses_update_settings_request** | Option<[**CoursesUpdateSettingsRequest**](CoursesUpdateSettingsRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/courses/<course_id>/settings \\   -X PUT \\   -H 'Authorization: Bearer <token>' \\   -d 'allow_student_discussion_topics=false' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_user

> String courses_user(course_id, id)
Get single user

Return information on a single user.  Accepts the same include[] parameters as the :users: action, and returns a single user with the same fields as that action.

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


## courses_user_index

> models::Course courses_user_index(user_id, include, state, enrollment_state, homeroom, account_id, page, per_page)
List courses for a user

Returns a paginated list of active courses for this user. To view the course list for a user other than yourself, you must be either an observer of that user or an administrator.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"needs_grading_count\"|\"syllabus_body\"|\"public_description\"|\"total_scores\"|\"current_grading_period_scores\"|\"grading_periods\"|term\"|\"account\"|\"course_progress\"|\"sections\"|\"storage_quota_used_mb\"|\"total_students\"|\"passback_status\"|\"favorites\"|\"teachers\"|\"observed_users\"|\"course_image\"|\"banner_image\"|\"concluded\"|\"post_manually\"] - \"needs_grading_count\": Optional information to include with each Course.   When needs_grading_count is given, and the current user has grading   rights, the total number of submissions needing grading for all   assignments is returned. - \"syllabus_body\": Optional information to include with each Course.   When syllabus_body is given the user-generated html for the course   syllabus is returned. - \"public_description\": Optional information to include with each Course.   When public_description is given the user-generated text for the course   public description is returned. - \"total_scores\": Optional information to include with each Course.   When total_scores is given, any student enrollments will also   include the fields 'computed_current_score', 'computed_final_score',   'computed_current_grade', and 'computed_final_grade' (see Enrollment   documentation for more information on these fields). This argument   is ignored if the course is configured to hide final grades. - \"current_grading_period_scores\": Optional information to include with   each Course. When current_grading_period_scores is given and total_scores   is given, any student enrollments will also include the fields   'has_grading_periods',   'totals_for_all_grading_periods_option', 'current_grading_period_title',   'current_grading_period_id', current_period_computed_current_score',   'current_period_computed_final_score',   'current_period_computed_current_grade', and   'current_period_computed_final_grade', as well as (if the user has permission)   'current_period_unposted_current_score',   'current_period_unposted_final_score',   'current_period_unposted_current_grade', and   'current_period_unposted_final_grade' (see Enrollment documentation for   more information on these fields). In addition, when this argument is   passed, the course will have a 'has_grading_periods' attribute   on it. This argument is ignored if the course is configured to hide final   grades or if the total_scores argument is not included. - \"grading_periods\": Optional information to include with each Course. When   grading_periods is given, a list of the grading periods associated with   each course is returned. - \"term\": Optional information to include with each Course. When   term is given, the information for the enrollment term for each course   is returned. - \"account\": Optional information to include with each Course. When   account is given, the account json for each course is returned. - \"course_progress\": Optional information to include with each Course.   When course_progress is given, each course will include a   'course_progress' object with the fields: 'requirement_count', an integer   specifying the total number of requirements in the course,   'requirement_completed_count', an integer specifying the total number of   requirements in this course that have been completed, and   'next_requirement_url', a string url to the next requirement item, and   'completed_at', the date the course was completed (null if incomplete).   'next_requirement_url' will be null if all requirements have been   completed or the current module does not require sequential progress.   \"course_progress\" will return an error message if the course is not   module based or the user is not enrolled as a student in the course. - \"sections\": Section enrollment information to include with each Course.   Returns an array of hashes containing the section ID (id), section name   (name), start and end dates (start_at, end_at), as well as the enrollment   type (enrollment_role, e.g. 'StudentEnrollment'). - \"storage_quota_used_mb\": The amount of storage space used by the files in this course - \"total_students\": Optional information to include with each Course.   Returns an integer for the total amount of active and invited students. - \"passback_status\": Include the grade passback_status - \"favorites\": Optional information to include with each Course.   Indicates if the user has marked the course as a favorite course. - \"teachers\": Teacher information to include with each Course.   Returns an array of hashes containing the {api:Users:UserDisplay UserDisplay} information   for each teacher in the course. - \"observed_users\": Optional information to include with each Course.   Will include data for observed users if the current user has an   observer enrollment. - \"tabs\": Optional information to include with each Course.   Will include the list of tabs configured for each course.  See the   {api:TabsController#index List available tabs API} for more information. - \"course_image\": Optional information to include with each Course. Returns course   image url if a course image has been set. - \"banner_image\": Optional information to include with each Course. Returns course   banner image url if the course is a Canvas for Elementary subject and a banner   image has been set. - \"concluded\": Optional information to include with each Course. Indicates whether   the course has been concluded, taking course and term dates into account. - \"post_manually\": Optional information to include with each Course. Returns true if   the course post policy is set to \"Manually\". Returns false if the the course post   policy is set to \"Automatically\". |  |
**state** | Option<[**Vec<String>**](String.md)> | [String, \"unpublished\"|\"available\"|\"completed\"|\"deleted\"] If set, only return courses that are in the given state(s). By default, \"available\" is returned for students and observers, and anything except \"deleted\", for all other enrollment types |  |
**enrollment_state** | Option<**String**> | When set, only return courses where the user has an enrollment with the given state. This will respect section/course/term date overrides. |  |
**homeroom** | Option<**bool**> | If set, only return homeroom courses. |  |
**account_id** | Option<**String**> | If set, only include courses associated with this account |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Course**](Course.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_user_progress

> String courses_user_progress(course_id, user_id)
Get user progress

Return progress information for the user and course  You can supply +self+ as the user_id to query your own progress in a course. To query another user's progress, you must be a teacher in the course, an administrator, or a linked observer of the user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## courses_users_for_courses

> serde_json::Value courses_users_for_courses(course_id, search_term, sort, enrollment_type, enrollment_role, enrollment_role_id, section_ids, include, user_id, user_ids, enrollment_state, page, per_page)
List users in course

Returns the paginated list of users in this course. And optionally the user's enrollments in the course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. |  |
**sort** | Option<**String**> | When set, sort the results of the search based on the given field. |  |
**enrollment_type** | Option<[**Vec<String>**](String.md)> | [String, \"teacher\"|\"student\"|\"student_view\"|\"ta\"|\"observer\"|\"designer\"] When set, only return users where the user is enrolled as this type. \"student_view\" implies include[]=test_student. This argument is ignored if enrollment_role is given. |  |
**enrollment_role** | Option<**String**> | Deprecated When set, only return users enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a base role type of 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**enrollment_role_id** | Option<**i32**> | When set, only return courses where the user is enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a built_in role id with type 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**section_ids** | Option<[**Vec<String>**](String.md)> | [Integer] When set, only return users who are enrolled in the given section(s). |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"enrollments\"|\"locked\"|\"avatar_url\"|\"test_student\"|\"bio\"|\"custom_links\"|\"current_grading_period_scores\"|\"uuid\"] - \"enrollments\": Optionally include with each Course the user's current and invited enrollments. If the user is enrolled as a student, and the account has permission to manage or view all grades, each enrollment will include a 'grades' key with 'current_score', 'final_score', 'current_grade' and 'final_grade' values. - \"locked\": Optionally include whether an enrollment is locked. - \"avatar_url\": Optionally include avatar_url. - \"bio\": Optionally include each user's bio. - \"test_student\": Optionally include the course's Test Student, if present. Default is to not include Test Student. - \"custom_links\": Optionally include plugin-supplied custom links for each student, such as analytics information - \"current_grading_period_scores\": if enrollments is included as well as this directive, the scores returned in the enrollment will be for the current grading period if there is one. A 'grading_period_id' value will also be included with the scores. if grading_period_id is nil there is no current grading period and the score is a total score. - \"uuid\": Optionally include the users uuid |  |
**user_id** | Option<**String**> | If this parameter is given and it corresponds to a user in the course, the +page+ parameter will be ignored and the page containing the specified user will be returned instead. |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If included, the course users set will only include users with IDs specified by the param. Note: this will not work in conjunction with the \"user_id\" argument but multiple user_ids can be included. |  |
**enrollment_state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"invited\"|\"rejected\"|\"completed\"|\"inactive\"] When set, only return users where the enrollment workflow state is of one of the given types. \"active\" and \"invited\" enrollments are returned by default. |  |
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


## courses_users_for_courses2

> serde_json::Value courses_users_for_courses2(course_id, search_term, sort, enrollment_type, enrollment_role, enrollment_role_id, section_ids, include, user_id, user_ids, enrollment_state, page, per_page)
List users in course

Returns the paginated list of users in this course. And optionally the user's enrollments in the course.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. |  |
**sort** | Option<**String**> | When set, sort the results of the search based on the given field. |  |
**enrollment_type** | Option<[**Vec<String>**](String.md)> | [String, \"teacher\"|\"student\"|\"student_view\"|\"ta\"|\"observer\"|\"designer\"] When set, only return users where the user is enrolled as this type. \"student_view\" implies include[]=test_student. This argument is ignored if enrollment_role is given. |  |
**enrollment_role** | Option<**String**> | Deprecated When set, only return users enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a base role type of 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**enrollment_role_id** | Option<**i32**> | When set, only return courses where the user is enrolled with the specified course-level role.  This can be a role created with the {api:RoleOverridesController#add_role Add Role API} or a built_in role id with type 'StudentEnrollment', 'TeacherEnrollment', 'TaEnrollment', 'ObserverEnrollment', or 'DesignerEnrollment'. |  |
**section_ids** | Option<[**Vec<String>**](String.md)> | [Integer] When set, only return users who are enrolled in the given section(s). |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"enrollments\"|\"locked\"|\"avatar_url\"|\"test_student\"|\"bio\"|\"custom_links\"|\"current_grading_period_scores\"|\"uuid\"] - \"enrollments\": Optionally include with each Course the user's current and invited enrollments. If the user is enrolled as a student, and the account has permission to manage or view all grades, each enrollment will include a 'grades' key with 'current_score', 'final_score', 'current_grade' and 'final_grade' values. - \"locked\": Optionally include whether an enrollment is locked. - \"avatar_url\": Optionally include avatar_url. - \"bio\": Optionally include each user's bio. - \"test_student\": Optionally include the course's Test Student, if present. Default is to not include Test Student. - \"custom_links\": Optionally include plugin-supplied custom links for each student, such as analytics information - \"current_grading_period_scores\": if enrollments is included as well as this directive, the scores returned in the enrollment will be for the current grading period if there is one. A 'grading_period_id' value will also be included with the scores. if grading_period_id is nil there is no current grading period and the score is a total score. - \"uuid\": Optionally include the users uuid |  |
**user_id** | Option<**String**> | If this parameter is given and it corresponds to a user in the course, the +page+ parameter will be ignored and the page containing the specified user will be returned instead. |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If included, the course users set will only include users with IDs specified by the param. Note: this will not work in conjunction with the \"user_id\" argument but multiple user_ids can be included. |  |
**enrollment_state** | Option<[**Vec<String>**](String.md)> | [String, \"active\"|\"invited\"|\"rejected\"|\"completed\"|\"inactive\"] When set, only return users where the enrollment workflow state is of one of the given types. \"active\" and \"invited\" enrollments are returned by default. |  |
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

