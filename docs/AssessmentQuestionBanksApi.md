# \AssessmentQuestionBanksApi

All URIs are relative to *https://canvas.instructure.com/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**assessment_question_banks_index**](AssessmentQuestionBanksApi.md#assessment_question_banks_index) | **GET** /question_banks | List question banks
[**assessment_question_banks_questions**](AssessmentQuestionBanksApi.md#assessment_question_banks_questions) | **GET** /question_banks/{id}/questions | List assessment questions for a question bank
[**assessment_question_banks_show**](AssessmentQuestionBanksApi.md#assessment_question_banks_show) | **GET** /question_banks/{id} | Get a single question bank



## assessment_question_banks_index

> models::AssessmentQuestionBank assessment_question_banks_index(context_type, context_id, include_question_count)
List question banks

Returns the paginated list of question banks for a given context.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**context_type** | **String** | The type of context. Must be either \"Course\" or \"Account\". | [required] |
**context_id** | **i32** | The id of the context. | [required] |
**include_question_count** | Option<**bool**> | Whether to include the number of questions in each bank. |  |

### Return type

[**models::AssessmentQuestionBank**](AssessmentQuestionBank.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assessment_question_banks_questions

> models::AssessmentQuestion assessment_question_banks_questions(id, page, per_page)
List assessment questions for a question bank

Returns the paginated list of assessment questions in this bank.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The question bank unique identifier. | [required] |
**page** | Option<**i32**> | The page number to return. Defaults to 1. |  |
**per_page** | Option<**i32**> | The number of items to return per page. Defaults to 10. Maximum is 100. |  |

### Return type

[**models::AssessmentQuestion**](AssessmentQuestion.md)

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## assessment_question_banks_show

> String assessment_question_banks_show(id, include_question_count)
Get a single question bank

Returns the question bank with the given id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | The question bank unique identifier. | [required] |
**include_question_count** | Option<**bool**> | Whether to include the number of questions in the bank. |  |

### Return type

**String**

### Authorization

[bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

