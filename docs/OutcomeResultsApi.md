# \OutcomeResultsApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**outcome_results_contributing_scores**](OutcomeResultsApi.md#outcome_results_contributing_scores) | **GET** /courses/{course_id}/outcomes/{outcome_id}/contributing_scores | Get contributing scores
[**outcome_results_enqueue_outcome_rollup_calculation**](OutcomeResultsApi.md#outcome_results_enqueue_outcome_rollup_calculation) | **POST** /enqueue_outcome_rollup_calculation | Enqueue a delayed Outcome Rollup Calculation Job
[**outcome_results_index**](OutcomeResultsApi.md#outcome_results_index) | **GET** /courses/{course_id}/outcome_results | Get outcome results
[**outcome_results_mastery_distribution**](OutcomeResultsApi.md#outcome_results_mastery_distribution) | **GET** /courses/{course_id}/outcome_mastery_distribution | Get mastery distribution
[**outcome_results_outcome_order**](OutcomeResultsApi.md#outcome_results_outcome_order) | **POST** /courses/{course_id}/assign_outcome_order | Set outcome ordering for LMGB
[**outcome_results_rollups**](OutcomeResultsApi.md#outcome_results_rollups) | **GET** /courses/{course_id}/outcome_rollups | Get outcome result rollups



## outcome_results_contributing_scores

> outcome_results_contributing_scores(course_id, outcome_id, user_ids, only_assignment_alignments, show_unpublished_assignments)
Get contributing scores

Gets the contributing scores for a specific outcome and set of users. Contributing scores are the individual assignment/quiz scores that contributed to the outcome score for each user.  Returns all alignments for the outcome in the course context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**outcome_id** | **String** | Scope response to outcome_id | [required] |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If specified, only the users whose ids are given will be included in the results. It is an error to specify an id for a user who is not a student in the context. |  |
**only_assignment_alignments** | Option<**bool**> | If specified, only assignment alignments will be included in the results. |  |
**show_unpublished_assignments** | Option<**bool**> | If true, unpublished assignments will be included in the results. Defaults to false. |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_results_enqueue_outcome_rollup_calculation

> String outcome_results_enqueue_outcome_rollup_calculation(outcome_results_enqueue_outcome_rollup_calculation_request)
Enqueue a delayed Outcome Rollup Calculation Job

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**outcome_results_enqueue_outcome_rollup_calculation_request** | Option<[**OutcomeResultsEnqueueOutcomeRollupCalculationRequest**](OutcomeResultsEnqueueOutcomeRollupCalculationRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_results_index

> outcome_results_index(course_id, user_ids, outcome_ids, include, include_hidden, page, per_page)
Get outcome results

Gets the outcome results for users and outcomes in the specified context.  used in sLMGB

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If specified, only the users whose ids are given will be included in the results. SIS ids can be used, prefixed by \"sis_user_id:\". It is an error to specify an id for a user who is not a student in the context. |  |
**outcome_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If specified, only the outcomes whose ids are given will be included in the results. it is an error to specify an id for an outcome which is not linked to the context. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"alignments\"|\"outcomes\"|\"outcomes.alignments\"|\"outcome_groups\"|\"outcome_links\"|\"outcome_paths\"|\"users\"] Specify additional collections to be side loaded with the result. \"alignments\" includes only the alignments referenced by the returned results. \"outcomes.alignments\" includes all alignments referenced by outcomes in the context. |  |
**include_hidden** | Option<**bool**> | If true, results that are hidden from the learning mastery gradebook and student rollup scores will be included |  |
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


## outcome_results_mastery_distribution

> String outcome_results_mastery_distribution(course_id, exclude, outcome_ids, student_ids, include, only_assignment_alignments, show_unpublished_assignments, add_defaults)
Get mastery distribution

Returns the distribution of student scores across mastery levels for all outcomes. This endpoint fetches data for ALL students (not paginated) to provide accurate distribution statistics for charts and analytics.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**exclude** | Option<[**Vec<String>**](String.md)> | [String] Optionally restrict which results are included: - \"missing_user_rollups\": exclude students without any scores - \"missing_outcome_results\": exclude outcomes without any results |  |
**outcome_ids** | Option<[**Vec<String>**](String.md)> | [String] Optionally restrict to specific outcome IDs |  |
**student_ids** | Option<[**Vec<String>**](String.md)> | [String] Optionally restrict to specific student IDs. If not provided, all students will be included. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String] Optionally include additional data: - \"alignment_distributions\": include contributing score distributions for alignments |  |
**only_assignment_alignments** | Option<**bool**> | If true and alignment_distributions is included, only include assignment alignments. Default: false. |  |
**show_unpublished_assignments** | Option<**bool**> | If true, include unpublished assignments in alignment distributions. Default: false. |  |
**add_defaults** | Option<**bool**> | If defaults are requested, then color and mastery level defaults will be added to outcome ratings in the result. This will only take effect if the Account Level Mastery Scales FF is DISABLED |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## outcome_results_outcome_order

> outcome_results_outcome_order(course_id)
Set outcome ordering for LMGB

Saves the ordering of outcomes in LMGB for a user

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


## outcome_results_rollups

> outcome_results_rollups(course_id, aggregate, aggregate_stat, user_ids, outcome_ids, include, exclude, sort_by, sort_outcome_id, sort_order, add_defaults, contributing_scores)
Get outcome result rollups

Gets the outcome rollups for the users and outcomes in the specified context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**aggregate** | Option<**String**> | If specified, instead of returning one rollup for each user, all the user rollups will be combined into one rollup for the course that will contain the average (or median, see below) rollup score for each outcome. |  |
**aggregate_stat** | Option<**String**> | If aggregate rollups requested, then this value determines what statistic is used for the aggregate. Defaults to \"mean\" if this value is not specified. |  |
**user_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If specified, only the users whose ids are given will be included in the results or used in an aggregate result. it is an error to specify an id for a user who is not a student in the context |  |
**outcome_ids** | Option<[**Vec<String>**](String.md)> | [Integer] If specified, only the outcomes whose ids are given will be included in the results. it is an error to specify an id for an outcome which is not linked to the context. |  |
**include** | Option<[**Vec<String>**](String.md)> | [String, \"courses\"|\"outcomes\"|\"outcomes.alignments\"|\"outcome_groups\"|\"outcome_links\"|\"outcome_paths\"|\"users\"] Specify additional collections to be side loaded with the result. |  |
**exclude** | Option<[**Vec<String>**](String.md)> | [String, \"missing_user_rollups\"|\"missing_outcome_results\"|\"] Specify additional values to exclude. \"missing_user_rollups\" excludes rollups for users without results. \"missing_outcome_results\" excludes outcomes without results. |  |
**sort_by** | Option<**String**> | If specified, sorts outcome result rollups. \"student\" sorting will sort by a user's sortable name. \"outcome\" sorting will sort by the given outcome's rollup score. The latter requires specifying the \"sort_outcome_id\" parameter. By default, the sort order is ascending. |  |
**sort_outcome_id** | Option<**i32**> | If outcome sorting requested, then this determines which outcome to use for rollup score sorting. |  |
**sort_order** | Option<**String**> | If sorting requested, then this allows changing the default sort order of ascending to descending. |  |
**add_defaults** | Option<**bool**> | If defaults are requested, then color and mastery level defaults will be added to outcome ratings in the rollup. This will only take effect if the Account Level Mastery Scales FF is DISABLED |  |
**contributing_scores** | Option<**bool**> | **DEPRECATED**: This parameter is deprecated. Use the separate GET /api/v1/courses/:course_id/outcomes/:outcome_id/contributing_scores endpoint instead to fetch contributing scores for a specific outcome. If contributing scores are requested, then each individual outcome score will also include all graded artifacts that contributed to the outcome score |  |

### Return type

 (empty response body)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

