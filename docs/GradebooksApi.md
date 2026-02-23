# \GradebooksApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gradebooks_apply_score_to_ungraded_submissions**](GradebooksApi.md#gradebooks_apply_score_to_ungraded_submissions) | **PUT** /courses/{course_id}/apply_score_to_ungraded_submissions | Apply score to ungraded submissions
[**gradebooks_update_final_grade_overrides**](GradebooksApi.md#gradebooks_update_final_grade_overrides) | **PUT** /courses/{course_id}/update_final_grade_overrides | Bulk update final grade overrides



## gradebooks_apply_score_to_ungraded_submissions

> String gradebooks_apply_score_to_ungraded_submissions(course_id, gradebooks_apply_score_to_ungraded_submissions_request)
Apply score to ungraded submissions

Perform a bulk scoring of ungraded submissions for a course, or mark ungraded submissions as excused. The course's account must have the \"Apply Score to Ungraded\" feature enabled, and the caller must have permission to manage grades. By default, will apply scores to all ungraded submissions in the course, but the scope may be restricted using the parameters below.  {   \"percent\": \"50.0\",   \"mark_as_missing\": true,   \"only_past_due\": true,   \"assignment_ids\": [\"1\", \"2\", \"3\"],   \"student_ids\": [\"11\", \"22\"] }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**gradebooks_apply_score_to_ungraded_submissions_request** | Option<[**GradebooksApplyScoreToUngradedSubmissionsRequest**](GradebooksApplyScoreToUngradedSubmissionsRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gradebooks_update_final_grade_overrides

> String gradebooks_update_final_grade_overrides(course_id, gradebooks_update_final_grade_overrides_request)
Bulk update final grade overrides

Set multiple final grade override scores for a course. The course must have final grade override enabled, and the caller must have permission to manage grades. Additionally, the \"Import Override Scores in Gradebook\" feature flag must be enabled.  {   \"grading_period_id\": \"10\",   \"override_scores\": [     {       \"student_id\": \"124\",       \"override_score\": \"80.0\"     },     {       \"student_id\": \"126\",       \"override_score\": \"70.0\"     }   ] }

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**course_id** | **String** | Scope response to course_id | [required] |
**gradebooks_update_final_grade_overrides_request** | Option<[**GradebooksUpdateFinalGradeOverridesRequest**](GradebooksUpdateFinalGradeOverridesRequest.md)> | Request body parameters |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

