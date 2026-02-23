# \ServiceCredentialsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**service_credentials_activity_stream_for_users**](ServiceCredentialsApi.md#service_credentials_activity_stream_for_users) | **GET** /users/self/activity_stream | List the activity stream
[**service_credentials_activity_stream_other**](ServiceCredentialsApi.md#service_credentials_activity_stream_other) | **GET** /users/activity_stream | List the activity stream
[**service_credentials_activity_stream_summary**](ServiceCredentialsApi.md#service_credentials_activity_stream_summary) | **GET** /users/self/activity_stream/summary | Activity stream summary
[**service_credentials_api_index**](ServiceCredentialsApi.md#service_credentials_api_index) | **GET** /accounts/{account_id}/users | List users in account
[**service_credentials_api_show**](ServiceCredentialsApi.md#service_credentials_api_show) | **GET** /users/{id} | Show user details
[**service_credentials_create**](ServiceCredentialsApi.md#service_credentials_create) | **POST** /accounts/{account_id}/users | Create a user
[**service_credentials_create_file**](ServiceCredentialsApi.md#service_credentials_create_file) | **POST** /users/{user_id}/files | Upload a file
[**service_credentials_create_self_registered_user**](ServiceCredentialsApi.md#service_credentials_create_self_registered_user) | **POST** /accounts/{account_id}/self_registration | [DEPRECATED] Self register a user
[**service_credentials_expire_mobile_sessions_for_users**](ServiceCredentialsApi.md#service_credentials_expire_mobile_sessions_for_users) | **DELETE** /users/{id}/mobile_sessions | Log users out of all mobile apps
[**service_credentials_expire_mobile_sessions_other**](ServiceCredentialsApi.md#service_credentials_expire_mobile_sessions_other) | **DELETE** /users/mobile_sessions | Log users out of all mobile apps
[**service_credentials_get_custom_color**](ServiceCredentialsApi.md#service_credentials_get_custom_color) | **GET** /users/{id}/colors/{asset_string} | Get custom color
[**service_credentials_get_custom_colors**](ServiceCredentialsApi.md#service_credentials_get_custom_colors) | **GET** /users/{id}/colors | Get custom colors
[**service_credentials_get_dashboard_positions**](ServiceCredentialsApi.md#service_credentials_get_dashboard_positions) | **GET** /users/{id}/dashboard_positions | Get dashboard positions
[**service_credentials_ignore_all_stream_items**](ServiceCredentialsApi.md#service_credentials_ignore_all_stream_items) | **DELETE** /users/self/activity_stream | Hide all stream items
[**service_credentials_ignore_stream_item**](ServiceCredentialsApi.md#service_credentials_ignore_stream_item) | **DELETE** /users/self/activity_stream/{id} | Hide a stream item
[**service_credentials_merge_into_for_users**](ServiceCredentialsApi.md#service_credentials_merge_into_for_users) | **PUT** /users/{id}/merge_into/{destination_user_id} | Merge user into another user
[**service_credentials_merge_into_for_users2**](ServiceCredentialsApi.md#service_credentials_merge_into_for_users2) | **PUT** /users/{id}/merge_into/accounts/{destination_account_id}/users/{destination_user_id} | Merge user into another user
[**service_credentials_missing_submissions**](ServiceCredentialsApi.md#service_credentials_missing_submissions) | **GET** /users/{user_id}/missing_submissions | List Missing Submissions
[**service_credentials_pandata_events_token**](ServiceCredentialsApi.md#service_credentials_pandata_events_token) | **POST** /users/self/pandata_events_token | Get a Pandata Events jwt token and its expiration date
[**service_credentials_set_custom_color**](ServiceCredentialsApi.md#service_credentials_set_custom_color) | **PUT** /users/{id}/colors/{asset_string} | Update custom color
[**service_credentials_set_dashboard_positions**](ServiceCredentialsApi.md#service_credentials_set_dashboard_positions) | **PUT** /users/{id}/dashboard_positions | Update dashboard positions
[**service_credentials_set_files_ui_version_preference**](ServiceCredentialsApi.md#service_credentials_set_files_ui_version_preference) | **PUT** /users/{id}/files_ui_version_preference | Update files UI version preference
[**service_credentials_set_text_editor_preference**](ServiceCredentialsApi.md#service_credentials_set_text_editor_preference) | **PUT** /users/{id}/text_editor_preference | Update text editor preference
[**service_credentials_settings**](ServiceCredentialsApi.md#service_credentials_settings) | **GET** /users/{id}/settings | Update user settings.
[**service_credentials_split**](ServiceCredentialsApi.md#service_credentials_split) | **POST** /users/{id}/split | Split merged users into separate users
[**service_credentials_terminate_sessions**](ServiceCredentialsApi.md#service_credentials_terminate_sessions) | **DELETE** /users/{id}/sessions | Terminate all user sessions
[**service_credentials_todo_item_count**](ServiceCredentialsApi.md#service_credentials_todo_item_count) | **GET** /users/self/todo_item_count | List counts for todo items
[**service_credentials_todo_items**](ServiceCredentialsApi.md#service_credentials_todo_items) | **GET** /users/self/todo | List the TODO items
[**service_credentials_upcoming_events**](ServiceCredentialsApi.md#service_credentials_upcoming_events) | **GET** /users/self/upcoming_events | List upcoming assignments, calendar events
[**service_credentials_update**](ServiceCredentialsApi.md#service_credentials_update) | **PUT** /users/{id} | Edit a user
[**service_credentials_user_graded_submissions**](ServiceCredentialsApi.md#service_credentials_user_graded_submissions) | **GET** /users/{id}/graded_submissions | Get a users most recently graded submissions



## service_credentials_activity_stream_for_users

> service_credentials_activity_stream_for_users(only_active_courses)
List the activity stream

Returns the current user's global activity stream, paginated.  There are many types of objects that can be returned in the activity stream. All object types have the same basic set of shared attributes:   !!!javascript   {     'created_at': '2011-07-13T09:12:00Z',     'updated_at': '2011-07-25T08:52:41Z',     'id': 1234,     'title': 'Stream Item Subject',     'message': 'This is the body text of the activity stream item. It is plain-text, and can be multiple paragraphs.',     'type': 'DiscussionTopic|Conversation|Message|Submission|Conference|Collaboration|AssessmentRequest...',     'read_state': false,     'context_type': 'course', // course|group     'course_id': 1,     'group_id': null,     'html_url': \"http://...\" // URL to the Canvas web UI for this stream item   }  In addition, each item type has its own set of attributes available.  DiscussionTopic:    !!!javascript   {     'type': 'DiscussionTopic',     'discussion_topic_id': 1234,     'total_root_discussion_entries': 5,     'require_initial_post': true,     'user_has_posted': true,     'root_discussion_entries': {       ...     }   }  For DiscussionTopic, the message is truncated at 4kb.  Announcement:    !!!javascript   {     'type': 'Announcement',     'announcement_id': 1234,     'total_root_discussion_entries': 5,     'require_initial_post': true,     'user_has_posted': null,     'root_discussion_entries': {       ...     }   }  For Announcement, the message is truncated at 4kb.  Conversation:    !!!javascript   {     'type': 'Conversation',     'conversation_id': 1234,     'private': false,     'participant_count': 3,   }  Message:    !!!javascript   {     'type': 'Message',     'message_id': 1234,     'notification_category': 'Assignment Graded'   }  Submission:  Returns an {api:Submissions:Submission Submission} with its Course and Assignment data.  Conference:    !!!javascript   {     'type': 'Conference',     'web_conference_id': 1234   }  Collaboration:    !!!javascript   {     'type': 'Collaboration',     'collaboration_id': 1234   }  AssessmentRequest:    !!!javascript   {     'type': 'AssessmentRequest',     'assessment_request_id': 1234   }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_active_courses** | Option<**bool**> | If true, will only return objects for courses the user is actively participating in |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_activity_stream_other

> service_credentials_activity_stream_other(only_active_courses)
List the activity stream

Returns the current user's global activity stream, paginated.  There are many types of objects that can be returned in the activity stream. All object types have the same basic set of shared attributes:   !!!javascript   {     'created_at': '2011-07-13T09:12:00Z',     'updated_at': '2011-07-25T08:52:41Z',     'id': 1234,     'title': 'Stream Item Subject',     'message': 'This is the body text of the activity stream item. It is plain-text, and can be multiple paragraphs.',     'type': 'DiscussionTopic|Conversation|Message|Submission|Conference|Collaboration|AssessmentRequest...',     'read_state': false,     'context_type': 'course', // course|group     'course_id': 1,     'group_id': null,     'html_url': \"http://...\" // URL to the Canvas web UI for this stream item   }  In addition, each item type has its own set of attributes available.  DiscussionTopic:    !!!javascript   {     'type': 'DiscussionTopic',     'discussion_topic_id': 1234,     'total_root_discussion_entries': 5,     'require_initial_post': true,     'user_has_posted': true,     'root_discussion_entries': {       ...     }   }  For DiscussionTopic, the message is truncated at 4kb.  Announcement:    !!!javascript   {     'type': 'Announcement',     'announcement_id': 1234,     'total_root_discussion_entries': 5,     'require_initial_post': true,     'user_has_posted': null,     'root_discussion_entries': {       ...     }   }  For Announcement, the message is truncated at 4kb.  Conversation:    !!!javascript   {     'type': 'Conversation',     'conversation_id': 1234,     'private': false,     'participant_count': 3,   }  Message:    !!!javascript   {     'type': 'Message',     'message_id': 1234,     'notification_category': 'Assignment Graded'   }  Submission:  Returns an {api:Submissions:Submission Submission} with its Course and Assignment data.  Conference:    !!!javascript   {     'type': 'Conference',     'web_conference_id': 1234   }  Collaboration:    !!!javascript   {     'type': 'Collaboration',     'collaboration_id': 1234   }  AssessmentRequest:    !!!javascript   {     'type': 'AssessmentRequest',     'assessment_request_id': 1234   }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_active_courses** | Option<**bool**> | If true, will only return objects for courses the user is actively participating in |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_activity_stream_summary

> service_credentials_activity_stream_summary(only_active_courses)
Activity stream summary

Returns a summary of the current user's global activity stream.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**only_active_courses** | Option<**bool**> | If true, will only return objects for courses the user is actively participating in |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_api_index

> serde_json::Value service_credentials_api_index(account_id, search_term, enrollment_type, sort, order, include_deleted_users, uuids, page, per_page)
List users in account

A paginated list of users associated with this account.   @example_request    curl https://<canvas>/api/v1/accounts/self/users?search_term=<search value> \\       -X GET \\       -H 'Authorization: Bearer <token>'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**search_term** | Option<**String**> | The partial name or full ID of the users to match and return in the results list. Must be at least 3 characters.  Note that the API will prefer matching on canonical user ID if the ID has a numeric form. It will only search against other fields if non-numeric in form, or if the numeric value doesn't yield any matches. Queries by administrative users will search on SIS ID, Integration ID, login ID, name, or email address |  |
**enrollment_type** | Option<**String**> | When set, only return users enrolled with the specified course-level base role. This can be a base role type of 'student', 'teacher', 'ta', 'observer', or 'designer'. |  |
**sort** | Option<**String**> | The column to sort results by. For efficiency, use +id+ if you intend to retrieve many pages of results. In the future, other sort options may be rate-limited after 50 pages. |  |
**order** | Option<**String**> | The order to sort the given column by. |  |
**include_deleted_users** | Option<**bool**> | When set to true and used with an account context, returns users who have deleted pseudonyms for the context |  |
**uuids** | Option<[**Vec<String>**](String.md)> | When set, only return users with the specified UUIDs. UUIDs after the first 100 are ignored. |  |
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


## service_credentials_api_show

> String service_credentials_api_show(id, include)
Show user details

Shows details for user.  Also includes an attribute \"permissions\", a non-comprehensive list of permissions for the user. Example:   !!!javascript   \"permissions\": {    \"can_update_name\": true, // Whether the user can update their name.    \"can_update_avatar\": false, // Whether the user can update their avatar.    \"limit_parent_app_web_access\": false // Whether the user can interact with Canvas web from the Canvas Parent app.   }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"uuid\", \"last_login\"] Array of additional information to include on the user record. \"locale\", \"avatar_url\", \"permissions\", \"email\", and \"effective_locale\" will always be returned |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_create

> String service_credentials_create(account_id, service_credentials_create_request)
Create a user

Create and return a new user and pseudonym for an account.  [DEPRECATED (for self-registration only)] If you don't have the \"Modify login details for users\" permission, but self-registration is enabled on the account, you can still use this endpoint to register new users. Certain fields will be required, and others will be ignored (see below).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**service_credentials_create_request** | Option<[**ServiceCredentialsCreateRequest**](ServiceCredentialsCreateRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_create_file

> service_credentials_create_file(user_id)
Upload a file

Upload a file to the user's personal files section.  This API endpoint is the first step in uploading a file to a user's files. See the {file:file.file_uploads.html File Upload Documentation} for details on the file upload workflow.  Note that typically users will only be able to upload files to their own files section. Passing a user_id of +self+ is an easy shortcut to specify the current user.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | Scope response to user_id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_create_self_registered_user

> String service_credentials_create_self_registered_user(account_id, service_credentials_create_self_registered_user_request)
[DEPRECATED] Self register a user

Self register and return a new user and pseudonym for an account.  If self-registration is enabled on the account, you can use this endpoint to self register new users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**service_credentials_create_self_registered_user_request** | Option<[**ServiceCredentialsCreateSelfRegisteredUserRequest**](ServiceCredentialsCreateSelfRegisteredUserRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_expire_mobile_sessions_for_users

> service_credentials_expire_mobile_sessions_for_users(id, skip_admins)
Log users out of all mobile apps

Permanently expires any active mobile sessions, forcing them to re-authorize.  The route that takes a user id will expire mobile sessions for that user. The route that doesn't take a user id will expire mobile sessions for *all* users in the institution (except for account administrators if +skip_admins+ is given).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the id | [required] |
**skip_admins** | Option<**bool**> | If true, will not expire mobile sessions for account administrators. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_expire_mobile_sessions_other

> service_credentials_expire_mobile_sessions_other(skip_admins)
Log users out of all mobile apps

Permanently expires any active mobile sessions, forcing them to re-authorize.  The route that takes a user id will expire mobile sessions for that user. The route that doesn't take a user id will expire mobile sessions for *all* users in the institution (except for account administrators if +skip_admins+ is given).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skip_admins** | Option<**bool**> | If true, will not expire mobile sessions for account administrators. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_get_custom_color

> service_credentials_get_custom_color(asset_string, id)
Get custom color

Returns the custom colors that have been saved for a user for a given context.  The asset_string parameter should be in the format 'context_id', for example 'course_42'.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_string** | **String** | Scope response to asset_string | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_get_custom_colors

> service_credentials_get_custom_colors(id)
Get custom colors

Returns all custom colors that have been saved for a user.

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


## service_credentials_get_dashboard_positions

> service_credentials_get_dashboard_positions(id)
Get dashboard positions

Returns all dashboard positions that have been saved for a user.

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


## service_credentials_ignore_all_stream_items

> service_credentials_ignore_all_stream_items()
Hide all stream items

Hide all stream items for the user

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


## service_credentials_ignore_stream_item

> service_credentials_ignore_stream_item(id)
Hide a stream item

Hide the given stream item.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_merge_into_for_users

> String service_credentials_merge_into_for_users(destination_user_id, id)
Merge user into another user

Merge a user into another user. To merge users, the caller must have permissions to manage both users. This should be considered irreversible. This will delete the user and move all the data into the destination user.  User merge details and caveats: The from_user is the user that was deleted in the user_merge process. The destination_user is the user that remains, that is being split.  Avatars: When both users have avatars, only the destination_users avatar will remain. When one user has an avatar, it will end up on the destination_user.  Terms of Use: If either user has accepted terms of use, it will be be left as accepted.  Communication Channels: All unique communication channels moved to the destination_user. All notification preferences are moved to the destination_user.  Enrollments: All unique enrollments are moved to the destination_user. When there is an enrollment that would end up making it so that a user would be observing themselves, the enrollment is not moved over. Everything that is tied to the from_user at the course level relating to the enrollment is also moved to the destination_user.  Submissions: All submissions are moved to the destination_user. If there are enrollments for both users in the same course, we prefer submissions that have grades then submissions that have work in them, and if there are no grades or no work, they are not moved.  Other notes: Access Tokens are moved on merge. Conversations are moved on merge. Favorites are moved on merge. Courses will commonly use LTI tools. LTI tools reference the user with IDs that are stored on a user object. Merging users deletes one user and moves all records from the deleted user to the destination_user. These IDs are kept for all enrollments, group_membership, and account_users for the from_user at the time of the merge. When the destination_user launches an LTI tool from a course that used to be the from_user's, it doesn't appear as a new user to the tool provider. Instead it will send the stored ids. The destination_user's LTI IDs remain as they were for the courses that they originally had. Future enrollments for the destination_user will use the IDs that are on the destination_user object. LTI IDs that are kept and tracked per context include lti_context_id, lti_id and uuid. APIs that return the LTI ids will return the one for the context that it is called for, except for the user uuid. The user UUID will display the destination_users uuid, and when getting the uuid from an api that is in a context that was recorded from a merge event, an additional attribute is added as past_uuid.  When finding users by SIS ids in different accounts the destination_account_id is required.  The account can also be identified by passing the domain in destination_account_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_user_id** | **String** | Scope response to destination_user_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_merge_into_for_users2

> String service_credentials_merge_into_for_users2(destination_account_id, destination_user_id, id)
Merge user into another user

Merge a user into another user. To merge users, the caller must have permissions to manage both users. This should be considered irreversible. This will delete the user and move all the data into the destination user.  User merge details and caveats: The from_user is the user that was deleted in the user_merge process. The destination_user is the user that remains, that is being split.  Avatars: When both users have avatars, only the destination_users avatar will remain. When one user has an avatar, it will end up on the destination_user.  Terms of Use: If either user has accepted terms of use, it will be be left as accepted.  Communication Channels: All unique communication channels moved to the destination_user. All notification preferences are moved to the destination_user.  Enrollments: All unique enrollments are moved to the destination_user. When there is an enrollment that would end up making it so that a user would be observing themselves, the enrollment is not moved over. Everything that is tied to the from_user at the course level relating to the enrollment is also moved to the destination_user.  Submissions: All submissions are moved to the destination_user. If there are enrollments for both users in the same course, we prefer submissions that have grades then submissions that have work in them, and if there are no grades or no work, they are not moved.  Other notes: Access Tokens are moved on merge. Conversations are moved on merge. Favorites are moved on merge. Courses will commonly use LTI tools. LTI tools reference the user with IDs that are stored on a user object. Merging users deletes one user and moves all records from the deleted user to the destination_user. These IDs are kept for all enrollments, group_membership, and account_users for the from_user at the time of the merge. When the destination_user launches an LTI tool from a course that used to be the from_user's, it doesn't appear as a new user to the tool provider. Instead it will send the stored ids. The destination_user's LTI IDs remain as they were for the courses that they originally had. Future enrollments for the destination_user will use the IDs that are on the destination_user object. LTI IDs that are kept and tracked per context include lti_context_id, lti_id and uuid. APIs that return the LTI ids will return the one for the context that it is called for, except for the user uuid. The user UUID will display the destination_users uuid, and when getting the uuid from an api that is in a context that was recorded from a merge event, an additional attribute is added as past_uuid.  When finding users by SIS ids in different accounts the destination_account_id is required.  The account can also be identified by passing the domain in destination_account_id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**destination_account_id** | **String** | ID of the destination account | [required] |
**destination_user_id** | **String** | Scope response to destination_user_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_missing_submissions

> serde_json::Value service_credentials_missing_submissions(user_id, course_ids, observed_user_id, include, filter, page, per_page)
List Missing Submissions

A paginated list of past-due assignments for which the student does not have a submission. The user sending the request must either be the student, an admin or a parent observer using the parent app

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | the student's ID | [required] |
**course_ids** | [**Vec<String>**](String.md) | [String] Optionally restricts the list of past-due assignments to only those associated with the specified course IDs. Required if observed_user_id is passed. | [required] |
**observed_user_id** | Option<**String**> | Return missing submissions for the given observed user. Must be accompanied by course_ids[]. The user making the request must be observing the observed user in all the courses specified by course_ids[]. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"planner_overrides\"|\"course\"] \"planner_overrides\":: Optionally include the assignment's associated planner override, if it exists, for the current user.                       These will be returned under a +planner_override+ key \"course\":: Optionally include the assignments' courses |  |
**filter** | Option<[**Vec<String>**](String.md)> | [String, \"submittable\"|\"current_grading_period\"] \"submittable\":: Only return assignments that the current user can submit (i.e. filter out locked assignments) \"current_grading_period\":: Only return missing assignments that are in the current grading period |  |
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


## service_credentials_pandata_events_token

> service_credentials_pandata_events_token(service_credentials_pandata_events_token_request)
Get a Pandata Events jwt token and its expiration date

Returns a jwt auth and props token that can be used to send events to Pandata.  NOTE: This is currently only available to the mobile developer keys.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**service_credentials_pandata_events_token_request** | Option<[**ServiceCredentialsPandataEventsTokenRequest**](ServiceCredentialsPandataEventsTokenRequest.md)> | Request body parameters  **Example Request:** ``` curl https://<canvas>/api/v1/users/self/pandata_events_token \\      -X POST \\      -H 'Authorization: Bearer <token>'      -F 'app_key=MOBILE_APPS_KEY' \\ ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_set_custom_color

> service_credentials_set_custom_color(asset_string, id, service_credentials_set_custom_color_request)
Update custom color

Updates a custom color for a user for a given context.  This allows colors for the calendar and elsewhere to be customized on a user basis.  The asset string parameter should be in the format 'context_id', for example 'course_42'

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**asset_string** | **String** | Scope response to asset_string | [required] |
**id** | **String** | Scope response to id | [required] |
**service_credentials_set_custom_color_request** | Option<[**ServiceCredentialsSetCustomColorRequest**](ServiceCredentialsSetCustomColorRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/<user_id>/colors/<asset_string> \\   -X PUT \\   -F 'hexcode=fffeee'   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_set_dashboard_positions

> service_credentials_set_dashboard_positions(id)
Update dashboard positions

Updates the dashboard positions for a user for a given context.  This allows positions for the dashboard cards and elsewhere to be customized on a per user basis.  The asset string parameter should be in the format 'context_id', for example 'course_42'

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


## service_credentials_set_files_ui_version_preference

> service_credentials_set_files_ui_version_preference(id, service_credentials_set_files_ui_version_preference_request)
Update files UI version preference

Updates a user's default choice for files UI version. This allows the files UI to preload the user's preference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**service_credentials_set_files_ui_version_preference_request** | Option<[**ServiceCredentialsSetFilesUiVersionPreferenceRequest**](ServiceCredentialsSetFilesUiVersionPreferenceRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/<user_id>/files_ui_version_preference \\   -X PUT \\   -F 'files_ui_version=v2'   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_set_text_editor_preference

> service_credentials_set_text_editor_preference(id, service_credentials_set_text_editor_preference_request)
Update text editor preference

Updates a user's default choice for text editor.  This allows the Choose an Editor propmts to preload the user's preference.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**service_credentials_set_text_editor_preference_request** | Option<[**ServiceCredentialsSetTextEditorPreferenceRequest**](ServiceCredentialsSetTextEditorPreferenceRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/<user_id>/prefered_editor \\   -X PUT \\   -F 'text_editor_preference=rce'   -H 'Authorization: Bearer <token>' ``` |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_settings

> service_credentials_settings(id, manual_mark_as_read, release_notes_badge_disabled, collapse_global_nav, collapse_course_nav, hide_dashcard_color_overlays, comment_library_suggestions_enabled, elementary_dashboard_disabled, widget_dashboard_user_preference)
Update user settings.

Update an existing user's settings.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**manual_mark_as_read** | Option<**bool**> | If true, require user to manually mark discussion posts as read (don't auto-mark as read). |  |
**release_notes_badge_disabled** | Option<**bool**> | If true, hide the badge for new release notes. |  |
**collapse_global_nav** | Option<**bool**> | If true, the user's page loads with the global navigation collapsed |  |
**collapse_course_nav** | Option<**bool**> | If true, the user's course pages will load with the course navigation collapsed. |  |
**hide_dashcard_color_overlays** | Option<**bool**> | If true, images on course cards will be presented without being tinted to match the course color. |  |
**comment_library_suggestions_enabled** | Option<**bool**> | If true, suggestions within the comment library will be shown. |  |
**elementary_dashboard_disabled** | Option<**bool**> | If true, will display the user's preferred class Canvas dashboard view instead of the canvas for elementary view. |  |
**widget_dashboard_user_preference** | Option<**bool**> | If true, enables the widget dashboard for the user. Only applies when the widget_dashboard feature is enabled at the account level. Defaults to true when the feature becomes available. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_split

> models::User service_credentials_split(id)
Split merged users into separate users

Merged users cannot be fully restored to their previous state, but this will attempt to split as much as possible to the previous state. To split a merged user, the caller must have permissions to manage all of the users logins. If there are multiple users that have been merged into one user it will split each merge into a separate user. A split can only happen within 180 days of a user merge. A user merge deletes the previous user and may be permanently deleted. In this scenario we create a new user object and proceed to move as much as possible to the new user. The user object will not have preserved the name or settings from the previous user. Some items may have been deleted during a user_merge that cannot be restored, and/or the data has become stale because of other changes to the objects since the time of the user_merge.  Split users details and caveats:  The from_user is the user that was deleted in the user_merge process. The destination_user is the user that remains, that is being split.  Avatars: When both users had avatars, both will be remain. When from_user had an avatar and destination_user did not have an avatar, the destination_user's avatar will be deleted if it still matches what was there are the time of the merge. If the destination_user's avatar was changed at anytime after the merge, it will remain on the destination user. If the from_user had an avatar it will be there after split.  Terms of Use: If from_user had not accepted terms of use, they will be prompted again to accept terms of use after the split. If the destination_user had not accepted terms of use, hey will be prompted again to accept terms of use after the split. If neither user had accepted the terms of use, but since the time of the merge had accepted, both will be prompted to accept terms of use. If both had accepted terms of use, this will remain.  Communication Channels: All communication channels are restored to what they were prior to the merge. If a communication channel was added after the merge, it will remain on the destination_user. Notification preferences remain with the communication channels.  Enrollments: All enrollments from the time of the merge will be moved back to where they were. Enrollments created since the time of the merge that were created by sis_import will go to the user that owns that sis_id used for the import. Other new enrollments will remain on the destination_user. Everything that is tied to the destination_user at the course level relating to an enrollment is moved to the from_user. When both users are in the same course prior to merge this can cause some unexpected items to move.  Submissions: Unlike other items tied to a course, submissions are explicitly recorded to avoid problems with grades. All submissions were moved are restored to the spot prior to merge. All submission that were created in a course that was moved in enrollments are moved over to the from_user.  Other notes: Access Tokens are moved back on split. Conversations are moved back on split. Favorites that existing at the time of merge are moved back on split. LTI ids are restored to how they were prior to merge.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |

### Return type

[**models::User**](User.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_terminate_sessions

> service_credentials_terminate_sessions(id)
Terminate all user sessions

Terminates all sessions for a user. This includes all browser-based sessions and all access tokens, including manually generated ones. The user can immediately re-authenticate to access Canvas again if they have the current credentials. All integrations will need to be re-authorized.

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
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_todo_item_count

> service_credentials_todo_item_count(include)
List counts for todo items

Counts of different todo items such as the number of assignments needing grading as well as the number of assignments needing submitting.  There is a limit to the number of todo items this endpoint will count. It will only look at the first 100 todo items for the user. If the user has more than 100 todo items this count may not be reliable. The largest reliable number for both counts is 100.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | [String, \"ungraded_quizzes\"] \"ungraded_quizzes\":: Optionally include ungraded quizzes (such as practice quizzes and surveys) in the list.                      These will be returned under a +quiz+ key instead of an +assignment+ key in response elements. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_todo_items

> service_credentials_todo_items(include, page, per_page)
List the TODO items

A paginated list of the current user's list of todo items.  There is a limit to the number of items returned.  The `ignore` and `ignore_permanently` URLs can be used to update the user's preferences on what items will be displayed. Performing a DELETE request against the `ignore` URL will hide that item from future todo item requests, until the item changes. Performing a DELETE request against the `ignore_permanently` URL will hide that item forever.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**include** | Option<[**Vec<String>**](String.md)> | [String, \"ungraded_quizzes\"] \"ungraded_quizzes\":: Optionally include ungraded quizzes (such as practice quizzes and surveys) in the list.                      These will be returned under a +quiz+ key instead of an +assignment+ key in response elements. |  |
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


## service_credentials_upcoming_events

> service_credentials_upcoming_events()
List upcoming assignments, calendar events

A paginated list of the current user's upcoming events.

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


## service_credentials_update

> String service_credentials_update(id, service_credentials_update_request)
Edit a user

Modify an existing user. To modify a user's login, see the documentation for logins.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**service_credentials_update_request** | Option<[**ServiceCredentialsUpdateRequest**](ServiceCredentialsUpdateRequest.md)> | Request body parameters  **Example Request:** ``` curl 'https://<canvas>/api/v1/users/133' \\      -X PUT \\      -F 'user[name]=Sheldon Cooper' \\      -F 'user[short_name]=Shelly' \\      -F 'user[time_zone]=Pacific Time (US & Canada)' \\      -F 'user[avatar][token]=<opaque_token>' \\      -H \"Authorization: Bearer <token>\" ``` |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## service_credentials_user_graded_submissions

> models::Submission service_credentials_user_graded_submissions(id, include, only_current_enrollments, only_published_assignments, page, per_page)
Get a users most recently graded submissions

Returns a list of the user's most recently graded submissions.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | Scope response to id | [required] |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"assignment\"] Associations to include with the group |  |
**only_current_enrollments** | Option<**bool**> | Returns submissions for only currently active enrollments |  |
**only_published_assignments** | Option<**bool**> | Returns submissions for only published assignments |  |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::Submission**](Submission.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

