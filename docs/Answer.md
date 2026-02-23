# Answer

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i64**> | The unique identifier for the answer.  Do not supply if this answer is part of a new question | [optional]
**answer_text** | **String** | The text of the answer. | 
**answer_weight** | **i64** | An integer to determine correctness of the answer. Incorrect answers should be 0, correct answers should be 100. | 
**answer_comments** | Option<**String**> | Specific contextual comments for a particular answer. | [optional]
**text_after_answers** | Option<**String**> | Used in missing word questions.  The text to follow the missing word | [optional]
**answer_match_left** | Option<**String**> | Used in matching questions.  The static value of the answer that will be displayed on the left for students to match for. | [optional]
**answer_match_right** | Option<**String**> | Used in matching questions. The correct match for the value given in answer_match_left.  Will be displayed in a dropdown with the other answer_match_right values.. | [optional]
**matching_answer_incorrect_matches** | Option<**String**> | Used in matching questions. A list of distractors, delimited by new lines ( ) that will be seeded with all the answer_match_right values. | [optional]
**numerical_answer_type** | Option<**String**> | Used in numerical questions.  Values can be 'exact_answer', 'range_answer', or 'precision_answer'. | [optional]
**exact** | Option<**i64**> | Used in numerical questions of type 'exact_answer'.  The value the answer should equal. | [optional]
**margin** | Option<**i64**> | Used in numerical questions of type 'exact_answer'. The margin of error allowed for the student's answer. | [optional]
**approximate** | Option<**f64**> | Used in numerical questions of type 'precision_answer'.  The value the answer should equal. | [optional]
**precision** | Option<**i64**> | Used in numerical questions of type 'precision_answer'. The numerical precision that will be used when comparing the student's answer. | [optional]
**start** | Option<**i64**> | Used in numerical questions of type 'range_answer'. The start of the allowed range (inclusive). | [optional]
**end** | Option<**i64**> | Used in numerical questions of type 'range_answer'. The end of the allowed range (inclusive). | [optional]
**blank_id** | Option<**i64**> | Used in fill in multiple blank and multiple dropdowns questions. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


