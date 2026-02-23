# \ContentMigrationsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**content_migrations_asset_id_mapping**](ContentMigrationsApi.md#content_migrations_asset_id_mapping) | **GET** /courses/{course_id}/content_migrations/{id}/asset_id_mapping | Get asset id mapping
[**content_migrations_available_migrators_for_accounts**](ContentMigrationsApi.md#content_migrations_available_migrators_for_accounts) | **GET** /accounts/{account_id}/content_migrations/migrators | List Migration Systems
[**content_migrations_available_migrators_for_courses**](ContentMigrationsApi.md#content_migrations_available_migrators_for_courses) | **GET** /courses/{course_id}/content_migrations/migrators | List Migration Systems
[**content_migrations_available_migrators_for_groups**](ContentMigrationsApi.md#content_migrations_available_migrators_for_groups) | **GET** /groups/{group_id}/content_migrations/migrators | List Migration Systems
[**content_migrations_available_migrators_for_users**](ContentMigrationsApi.md#content_migrations_available_migrators_for_users) | **GET** /users/{user_id}/content_migrations/migrators | List Migration Systems
[**content_migrations_content_list_for_accounts**](ContentMigrationsApi.md#content_migrations_content_list_for_accounts) | **GET** /accounts/{account_id}/content_migrations/{id}/selective_data | List items for selective import
[**content_migrations_content_list_for_courses**](ContentMigrationsApi.md#content_migrations_content_list_for_courses) | **GET** /courses/{course_id}/content_migrations/{id}/selective_data | List items for selective import
[**content_migrations_content_list_for_groups**](ContentMigrationsApi.md#content_migrations_content_list_for_groups) | **GET** /groups/{group_id}/content_migrations/{id}/selective_data | List items for selective import
[**content_migrations_content_list_for_users**](ContentMigrationsApi.md#content_migrations_content_list_for_users) | **GET** /users/{user_id}/content_migrations/{id}/selective_data | List items for selective import
[**content_migrations_create_for_accounts**](ContentMigrationsApi.md#content_migrations_create_for_accounts) | **POST** /accounts/{account_id}/content_migrations | Create a content migration
[**content_migrations_create_for_courses**](ContentMigrationsApi.md#content_migrations_create_for_courses) | **POST** /courses/{course_id}/content_migrations | Create a content migration
[**content_migrations_create_for_groups**](ContentMigrationsApi.md#content_migrations_create_for_groups) | **POST** /groups/{group_id}/content_migrations | Create a content migration
[**content_migrations_create_for_users**](ContentMigrationsApi.md#content_migrations_create_for_users) | **POST** /users/{user_id}/content_migrations | Create a content migration
[**content_migrations_index_for_accounts**](ContentMigrationsApi.md#content_migrations_index_for_accounts) | **GET** /accounts/{account_id}/content_migrations | List content migrations
[**content_migrations_index_for_courses**](ContentMigrationsApi.md#content_migrations_index_for_courses) | **GET** /courses/{course_id}/content_migrations | List content migrations
[**content_migrations_index_for_groups**](ContentMigrationsApi.md#content_migrations_index_for_groups) | **GET** /groups/{group_id}/content_migrations | List content migrations
[**content_migrations_index_for_users**](ContentMigrationsApi.md#content_migrations_index_for_users) | **GET** /users/{user_id}/content_migrations | List content migrations
[**content_migrations_show_for_accounts**](ContentMigrationsApi.md#content_migrations_show_for_accounts) | **GET** /accounts/{account_id}/content_migrations/{id} | Get a content migration
[**content_migrations_show_for_courses**](ContentMigrationsApi.md#content_migrations_show_for_courses) | **GET** /courses/{course_id}/content_migrations/{id} | Get a content migration
[**content_migrations_show_for_groups**](ContentMigrationsApi.md#content_migrations_show_for_groups) | **GET** /groups/{group_id}/content_migrations/{id} | Get a content migration
[**content_migrations_show_for_users**](ContentMigrationsApi.md#content_migrations_show_for_users) | **GET** /users/{user_id}/content_migrations/{id} | Get a content migration
[**content_migrations_update_for_accounts**](ContentMigrationsApi.md#content_migrations_update_for_accounts) | **PUT** /accounts/{account_id}/content_migrations/{id} | Update a content migration
[**content_migrations_update_for_courses**](ContentMigrationsApi.md#content_migrations_update_for_courses) | **PUT** /courses/{course_id}/content_migrations/{id} | Update a content migration
[**content_migrations_update_for_groups**](ContentMigrationsApi.md#content_migrations_update_for_groups) | **PUT** /groups/{group_id}/content_migrations/{id} | Update a content migration
[**content_migrations_update_for_users**](ContentMigrationsApi.md#content_migrations_update_for_users) | **PUT** /users/{user_id}/content_migrations/{id} | Update a content migration



## content_migrations_asset_id_mapping

> content_migrations_asset_id_mapping(course_id, id)
Get asset id mapping

Given a complete course copy or blueprint import content migration, return a mapping of asset ids from the source course to the destination course that were copied in this migration or an earlier one with the same course pair and migration_type (course copy or blueprint).  The returned object's keys are asset types as they appear in API URLs (+announcements+, +assignments+, +discussion_topics+, +files+, +module_items+, +modules+, +pages+, and +quizzes+). The values are a mapping from id in source course to id in destination course for objects of this type.

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


## content_migrations_available_migrators_for_accounts

> serde_json::Value content_migrations_available_migrators_for_accounts(account_id)
List Migration Systems

Lists the currently available migration types. These values may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_available_migrators_for_courses

> serde_json::Value content_migrations_available_migrators_for_courses(course_id)
List Migration Systems

Lists the currently available migration types. These values may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_available_migrators_for_groups

> serde_json::Value content_migrations_available_migrators_for_groups(group_id)
List Migration Systems

Lists the currently available migration types. These values may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_available_migrators_for_users

> serde_json::Value content_migrations_available_migrators_for_users(user_id)
List Migration Systems

Lists the currently available migration types. These values may change.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_content_list_for_accounts

> String content_migrations_content_list_for_accounts(account_id, id, type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket)
List items for selective import

Enumerates the content available for selective import in a tree structure. Each node provides a +property+ copy argument that can be supplied to the {api:ContentMigrationsController#update Update endpoint} to selectively copy the content associated with that tree node and its children. Each node may also provide a +sub_items_url+ or an array of +sub_items+ which you can use to obtain copy parameters for a subset of the resources in a given node.  If no +type+ is sent you will get a list of the top-level sections in the content. It will look something like this:    [{     \"type\": \"course_settings\",     \"property\": \"copy[all_course_settings]\",     \"title\": \"Course Settings\"   },   {     \"type\": \"context_modules\",     \"property\": \"copy[all_context_modules]\",     \"title\": \"Modules\",     \"count\": 5,     \"sub_items_url\": \"http://example.com/api/v1/courses/22/content_migrations/77/selective_data?type=context_modules\"   },   {     \"type\": \"assignments\",     \"property\": \"copy[all_assignments]\",     \"title\": \"Assignments\",     \"count\": 2,     \"sub_items_url\": \"http://localhost:3000/api/v1/courses/22/content_migrations/77/selective_data?type=assignments\"   }]  When a +type+ is provided, nodes may be further divided via +sub_items+. For example, using +type=assignments+ results in a node for each assignment group and a sub_item for each assignment, like this:    [{     \"type\": \"assignment_groups\",     \"title\": \"An Assignment Group\",     \"property\": \"copy[assignment_groups][id_i855cf145e5acc7435e1bf1c6e2126e5f]\",     \"sub_items\": [{         \"type\": \"assignments\",         \"title\": \"Assignment 1\",         \"property\": \"copy[assignments][id_i2102a7fa93b29226774949298626719d]\"     }, {         \"type\": \"assignments\",         \"title\": \"Assignment 2\",         \"property\": \"copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]\"     }]   }]   To import the items corresponding to a particular tree node, use the +property+ as a parameter to the {api:ContentMigrationsController#update Update endpoint} and assign a value of 1, for example:    copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]=1  You can include multiple copy parameters to selectively import multiple items or groups of items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |
**type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket** | Option<**String**> | The type of content to enumerate. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_content_list_for_courses

> String content_migrations_content_list_for_courses(course_id, id, type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket)
List items for selective import

Enumerates the content available for selective import in a tree structure. Each node provides a +property+ copy argument that can be supplied to the {api:ContentMigrationsController#update Update endpoint} to selectively copy the content associated with that tree node and its children. Each node may also provide a +sub_items_url+ or an array of +sub_items+ which you can use to obtain copy parameters for a subset of the resources in a given node.  If no +type+ is sent you will get a list of the top-level sections in the content. It will look something like this:    [{     \"type\": \"course_settings\",     \"property\": \"copy[all_course_settings]\",     \"title\": \"Course Settings\"   },   {     \"type\": \"context_modules\",     \"property\": \"copy[all_context_modules]\",     \"title\": \"Modules\",     \"count\": 5,     \"sub_items_url\": \"http://example.com/api/v1/courses/22/content_migrations/77/selective_data?type=context_modules\"   },   {     \"type\": \"assignments\",     \"property\": \"copy[all_assignments]\",     \"title\": \"Assignments\",     \"count\": 2,     \"sub_items_url\": \"http://localhost:3000/api/v1/courses/22/content_migrations/77/selective_data?type=assignments\"   }]  When a +type+ is provided, nodes may be further divided via +sub_items+. For example, using +type=assignments+ results in a node for each assignment group and a sub_item for each assignment, like this:    [{     \"type\": \"assignment_groups\",     \"title\": \"An Assignment Group\",     \"property\": \"copy[assignment_groups][id_i855cf145e5acc7435e1bf1c6e2126e5f]\",     \"sub_items\": [{         \"type\": \"assignments\",         \"title\": \"Assignment 1\",         \"property\": \"copy[assignments][id_i2102a7fa93b29226774949298626719d]\"     }, {         \"type\": \"assignments\",         \"title\": \"Assignment 2\",         \"property\": \"copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]\"     }]   }]   To import the items corresponding to a particular tree node, use the +property+ as a parameter to the {api:ContentMigrationsController#update Update endpoint} and assign a value of 1, for example:    copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]=1  You can include multiple copy parameters to selectively import multiple items or groups of items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**id** | **String** | Scope response to id | [required] |
**type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket** | Option<**String**> | The type of content to enumerate. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_content_list_for_groups

> String content_migrations_content_list_for_groups(group_id, id, type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket)
List items for selective import

Enumerates the content available for selective import in a tree structure. Each node provides a +property+ copy argument that can be supplied to the {api:ContentMigrationsController#update Update endpoint} to selectively copy the content associated with that tree node and its children. Each node may also provide a +sub_items_url+ or an array of +sub_items+ which you can use to obtain copy parameters for a subset of the resources in a given node.  If no +type+ is sent you will get a list of the top-level sections in the content. It will look something like this:    [{     \"type\": \"course_settings\",     \"property\": \"copy[all_course_settings]\",     \"title\": \"Course Settings\"   },   {     \"type\": \"context_modules\",     \"property\": \"copy[all_context_modules]\",     \"title\": \"Modules\",     \"count\": 5,     \"sub_items_url\": \"http://example.com/api/v1/courses/22/content_migrations/77/selective_data?type=context_modules\"   },   {     \"type\": \"assignments\",     \"property\": \"copy[all_assignments]\",     \"title\": \"Assignments\",     \"count\": 2,     \"sub_items_url\": \"http://localhost:3000/api/v1/courses/22/content_migrations/77/selective_data?type=assignments\"   }]  When a +type+ is provided, nodes may be further divided via +sub_items+. For example, using +type=assignments+ results in a node for each assignment group and a sub_item for each assignment, like this:    [{     \"type\": \"assignment_groups\",     \"title\": \"An Assignment Group\",     \"property\": \"copy[assignment_groups][id_i855cf145e5acc7435e1bf1c6e2126e5f]\",     \"sub_items\": [{         \"type\": \"assignments\",         \"title\": \"Assignment 1\",         \"property\": \"copy[assignments][id_i2102a7fa93b29226774949298626719d]\"     }, {         \"type\": \"assignments\",         \"title\": \"Assignment 2\",         \"property\": \"copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]\"     }]   }]   To import the items corresponding to a particular tree node, use the +property+ as a parameter to the {api:ContentMigrationsController#update Update endpoint} and assign a value of 1, for example:    copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]=1  You can include multiple copy parameters to selectively import multiple items or groups of items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |
**type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket** | Option<**String**> | The type of content to enumerate. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_content_list_for_users

> String content_migrations_content_list_for_users(user_id, id, type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket)
List items for selective import

Enumerates the content available for selective import in a tree structure. Each node provides a +property+ copy argument that can be supplied to the {api:ContentMigrationsController#update Update endpoint} to selectively copy the content associated with that tree node and its children. Each node may also provide a +sub_items_url+ or an array of +sub_items+ which you can use to obtain copy parameters for a subset of the resources in a given node.  If no +type+ is sent you will get a list of the top-level sections in the content. It will look something like this:    [{     \"type\": \"course_settings\",     \"property\": \"copy[all_course_settings]\",     \"title\": \"Course Settings\"   },   {     \"type\": \"context_modules\",     \"property\": \"copy[all_context_modules]\",     \"title\": \"Modules\",     \"count\": 5,     \"sub_items_url\": \"http://example.com/api/v1/courses/22/content_migrations/77/selective_data?type=context_modules\"   },   {     \"type\": \"assignments\",     \"property\": \"copy[all_assignments]\",     \"title\": \"Assignments\",     \"count\": 2,     \"sub_items_url\": \"http://localhost:3000/api/v1/courses/22/content_migrations/77/selective_data?type=assignments\"   }]  When a +type+ is provided, nodes may be further divided via +sub_items+. For example, using +type=assignments+ results in a node for each assignment group and a sub_item for each assignment, like this:    [{     \"type\": \"assignment_groups\",     \"title\": \"An Assignment Group\",     \"property\": \"copy[assignment_groups][id_i855cf145e5acc7435e1bf1c6e2126e5f]\",     \"sub_items\": [{         \"type\": \"assignments\",         \"title\": \"Assignment 1\",         \"property\": \"copy[assignments][id_i2102a7fa93b29226774949298626719d]\"     }, {         \"type\": \"assignments\",         \"title\": \"Assignment 2\",         \"property\": \"copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]\"     }]   }]   To import the items corresponding to a particular tree node, use the +property+ as a parameter to the {api:ContentMigrationsController#update Update endpoint} and assign a value of 1, for example:    copy[assignments][id_i310cba275dc3f4aa8a3306bbbe380979]=1  You can include multiple copy parameters to selectively import multiple items or groups of items.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**id** | **String** | Scope response to id | [required] |
**type_left_square_bracket_double_quote_context_modules_double_quote_pipe_double_quote_assignments_double_quote_pipe_double_quote_quizzes_double_quote_pipe_double_quote_assessment_question_banks_double_quote_pipe_double_quote_discussion_topics_double_quote_pipe_double_quote_wiki_pages_double_quote_pipe_double_quote_context_external_tools_double_quote_pipe_double_quote_tool_profiles_double_quote_pipe_double_quote_announcements_double_quote_pipe_double_quote_calendar_events_double_quote_pipe_double_quote_rubrics_double_quote_pipe_double_quote_groups_double_quote_pipe_double_quote_learning_outcomes_double_quote_pipe_double_quote_attachments_double_quote_right_square_bracket** | Option<**String**> | The type of content to enumerate. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_create_for_accounts

> String content_migrations_create_for_accounts(account_id, content_migrations_create_for_accounts_request)
Create a content migration

Create a content migration. If the migration requires a file to be uploaded the actual processing of the file will start once the file upload process is completed. File uploading works as described in the {file:file.file_uploads.html File Upload Documentation} except that the values are set on a *pre_attachment* sub-hash.  For migrations that don't require a file to be uploaded, like course copy, the processing will begin as soon as the migration is created.  You can use the {api:ProgressController#show Progress API} to track the progress of the migration. The migration's progress is linked to with the _progress_url_ value.  The two general workflows are:  If no file upload is needed:  1. POST to create 2. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress  For file uploading:  1. POST to create with file info in *pre_attachment* 2. Do {file:file.file_uploads.html file upload processing} using the data in the *pre_attachment* data 3. {api:ContentMigrationsController#show GET} the ContentMigration 4. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress   (required if doing .zip file upload)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**content_migrations_create_for_accounts_request** | Option<[**ContentMigrationsCreateForAccountsRequest**](ContentMigrationsCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_create_for_courses

> String content_migrations_create_for_courses(course_id, content_migrations_create_for_accounts_request)
Create a content migration

Create a content migration. If the migration requires a file to be uploaded the actual processing of the file will start once the file upload process is completed. File uploading works as described in the {file:file.file_uploads.html File Upload Documentation} except that the values are set on a *pre_attachment* sub-hash.  For migrations that don't require a file to be uploaded, like course copy, the processing will begin as soon as the migration is created.  You can use the {api:ProgressController#show Progress API} to track the progress of the migration. The migration's progress is linked to with the _progress_url_ value.  The two general workflows are:  If no file upload is needed:  1. POST to create 2. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress  For file uploading:  1. POST to create with file info in *pre_attachment* 2. Do {file:file.file_uploads.html file upload processing} using the data in the *pre_attachment* data 3. {api:ContentMigrationsController#show GET} the ContentMigration 4. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress   (required if doing .zip file upload)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | ID of the course | [required] |
**content_migrations_create_for_accounts_request** | Option<[**ContentMigrationsCreateForAccountsRequest**](ContentMigrationsCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_create_for_groups

> String content_migrations_create_for_groups(group_id, content_migrations_create_for_accounts_request)
Create a content migration

Create a content migration. If the migration requires a file to be uploaded the actual processing of the file will start once the file upload process is completed. File uploading works as described in the {file:file.file_uploads.html File Upload Documentation} except that the values are set on a *pre_attachment* sub-hash.  For migrations that don't require a file to be uploaded, like course copy, the processing will begin as soon as the migration is created.  You can use the {api:ProgressController#show Progress API} to track the progress of the migration. The migration's progress is linked to with the _progress_url_ value.  The two general workflows are:  If no file upload is needed:  1. POST to create 2. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress  For file uploading:  1. POST to create with file info in *pre_attachment* 2. Do {file:file.file_uploads.html file upload processing} using the data in the *pre_attachment* data 3. {api:ContentMigrationsController#show GET} the ContentMigration 4. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress   (required if doing .zip file upload)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**content_migrations_create_for_accounts_request** | Option<[**ContentMigrationsCreateForAccountsRequest**](ContentMigrationsCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_create_for_users

> String content_migrations_create_for_users(user_id, content_migrations_create_for_accounts_request)
Create a content migration

Create a content migration. If the migration requires a file to be uploaded the actual processing of the file will start once the file upload process is completed. File uploading works as described in the {file:file.file_uploads.html File Upload Documentation} except that the values are set on a *pre_attachment* sub-hash.  For migrations that don't require a file to be uploaded, like course copy, the processing will begin as soon as the migration is created.  You can use the {api:ProgressController#show Progress API} to track the progress of the migration. The migration's progress is linked to with the _progress_url_ value.  The two general workflows are:  If no file upload is needed:  1. POST to create 2. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress  For file uploading:  1. POST to create with file info in *pre_attachment* 2. Do {file:file.file_uploads.html file upload processing} using the data in the *pre_attachment* data 3. {api:ContentMigrationsController#show GET} the ContentMigration 4. Use the {api:ProgressController#show Progress} specified in _progress_url_ to monitor progress   (required if doing .zip file upload)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**content_migrations_create_for_accounts_request** | Option<[**ContentMigrationsCreateForAccountsRequest**](ContentMigrationsCreateForAccountsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_index_for_accounts

> serde_json::Value content_migrations_index_for_accounts(account_id, page, per_page)
List content migrations

Returns paginated content migrations

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


## content_migrations_index_for_courses

> serde_json::Value content_migrations_index_for_courses(course_id, page, per_page)
List content migrations

Returns paginated content migrations

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


## content_migrations_index_for_groups

> serde_json::Value content_migrations_index_for_groups(group_id, page, per_page)
List content migrations

Returns paginated content migrations

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


## content_migrations_index_for_users

> serde_json::Value content_migrations_index_for_users(user_id, page, per_page)
List content migrations

Returns paginated content migrations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
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


## content_migrations_show_for_accounts

> String content_migrations_show_for_accounts(account_id, id)
Get a content migration

Returns data on an individual content migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_show_for_courses

> String content_migrations_show_for_courses(course_id, id)
Get a content migration

Returns data on an individual content migration

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


## content_migrations_show_for_groups

> String content_migrations_show_for_groups(group_id, id)
Get a content migration

Returns data on an individual content migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_show_for_users

> String content_migrations_show_for_users(user_id, id)
Get a content migration

Returns data on an individual content migration

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_update_for_accounts

> String content_migrations_update_for_accounts(account_id, id)
Update a content migration

Update a content migration. Takes same arguments as {api:ContentMigrationsController#create create} except that you can't change the migration type. However, changing most settings after the migration process has started will not do anything. Generally updating the content migration will be used when there is a file upload problem, or when importing content selectively. If the first upload has a problem you can supply new _pre_attachment_ values to start the process again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | Scope response to account_id | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_update_for_courses

> String content_migrations_update_for_courses(course_id, id)
Update a content migration

Update a content migration. Takes same arguments as {api:ContentMigrationsController#create create} except that you can't change the migration type. However, changing most settings after the migration process has started will not do anything. Generally updating the content migration will be used when there is a file upload problem, or when importing content selectively. If the first upload has a problem you can supply new _pre_attachment_ values to start the process again.

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


## content_migrations_update_for_groups

> String content_migrations_update_for_groups(group_id, id)
Update a content migration

Update a content migration. Takes same arguments as {api:ContentMigrationsController#create create} except that you can't change the migration type. However, changing most settings after the migration process has started will not do anything. Generally updating the content migration will be used when there is a file upload problem, or when importing content selectively. If the first upload has a problem you can supply new _pre_attachment_ values to start the process again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**group_id** | **String** | ID of the group | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## content_migrations_update_for_users

> String content_migrations_update_for_users(user_id, id)
Update a content migration

Update a content migration. Takes same arguments as {api:ContentMigrationsController#create create} except that you can't change the migration type. However, changing most settings after the migration process has started will not do anything. Generally updating the content migration will be used when there is a file upload problem, or when importing content selectively. If the first upload has a problem you can supply new _pre_attachment_ values to start the process again.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**user_id** | **String** | ID of the user | [required] |
**id** | **String** | Scope response to id | [required] |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

