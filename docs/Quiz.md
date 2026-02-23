# Quiz

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | the ID of the quiz | [optional]
**title** | Option<**String**> | the title of the quiz | [optional]
**html_url** | Option<**String**> | the HTTP/HTTPS URL to the quiz | [optional]
**mobile_url** | Option<**String**> | a url suitable for loading the quiz in a mobile webview.  it will persiste the headless session and, for quizzes in public courses, will force the user to login | [optional]
**preview_url** | Option<**String**> | A url that can be visited in the browser with a POST request to preview a quiz as the teacher. Only present when the user may grade | [optional]
**description** | Option<**String**> | the description of the quiz | [optional]
**quiz_type** | Option<**QuizType**> | type of quiz possible values: 'practice_quiz', 'assignment', 'graded_survey', 'survey' (enum: practice_quiz, assignment, graded_survey, survey) | [optional]
**assignment_group_id** | Option<**i32**> | the ID of the quiz's assignment group: | [optional]
**time_limit** | Option<**i32**> | quiz time limit in minutes | [optional]
**shuffle_answers** | Option<**bool**> | shuffle answers for students? | [optional]
**hide_results** | Option<**HideResults**> | let students see their quiz responses? possible values: null, 'always', 'until_after_last_attempt' (enum: always, until_after_last_attempt) | [optional]
**show_correct_answers** | Option<**bool**> | show which answers were correct when results are shown? only valid if hide_results=null | [optional]
**show_correct_answers_last_attempt** | Option<**bool**> | restrict the show_correct_answers option above to apply only to the last submitted attempt of a quiz that allows multiple attempts. only valid if show_correct_answers=true and allowed_attempts > 1 | [optional]
**show_correct_answers_at** | Option<**String**> | when should the correct answers be visible by students? only valid if show_correct_answers=true | [optional]
**hide_correct_answers_at** | Option<**String**> | prevent the students from seeing correct answers after the specified date has passed. only valid if show_correct_answers=true | [optional]
**one_time_results** | Option<**bool**> | prevent the students from seeing their results more than once (right after they submit the quiz) | [optional]
**scoring_policy** | Option<**ScoringPolicy**> | which quiz score to keep (only if allowed_attempts != 1) possible values: 'keep_highest', 'keep_latest' (enum: keep_highest, keep_latest) | [optional]
**allowed_attempts** | Option<**i32**> | how many times a student can take the quiz -1 = unlimited attempts | [optional]
**one_question_at_a_time** | Option<**bool**> | show one question at a time? | [optional]
**question_count** | Option<**i32**> | the number of questions in the quiz | [optional]
**points_possible** | Option<**i32**> | The total point value given to the quiz | [optional]
**cant_go_back** | Option<**bool**> | lock questions after answering? only valid if one_question_at_a_time=true | [optional]
**access_code** | Option<**String**> | access code to restrict quiz access | [optional]
**ip_filter** | Option<**String**> | IP address or range that quiz access is limited to | [optional]
**due_at** | Option<**String**> | when the quiz is due | [optional]
**lock_at** | Option<**String**> | when to lock the quiz | [optional]
**unlock_at** | Option<**String**> | when to unlock the quiz | [optional]
**published** | Option<**bool**> | whether the quiz has a published or unpublished draft state. | [optional]
**unpublishable** | Option<**bool**> | Whether the assignment's 'published' state can be changed to false. Will be false if there are student submissions for the quiz. | [optional]
**locked_for_user** | Option<**bool**> | Whether or not this is locked for the user. | [optional]
**lock_info** | Option<[**models::LockInfo**](LockInfo.md)> |  | [optional]
**lock_explanation** | Option<**String**> | (Optional) An explanation of why this is locked for the user. Present when locked_for_user is true. | [optional]
**speedgrader_url** | Option<**String**> | Link to SpeedGrader for this quiz. Will not be present if quiz is unpublished | [optional]
**quiz_extensions_url** | Option<**String**> | Link to endpoint to send extensions for this quiz. | [optional]
**permissions** | Option<[**models::QuizPermissions**](QuizPermissions.md)> |  | [optional]
**all_dates** | Option<[**Vec<models::AssignmentDate>**](AssignmentDate.md)> | list of due dates for the quiz | [optional]
**version_number** | Option<**i32**> | Current version number of the quiz | [optional]
**question_types** | Option<**Vec<String>**> | List of question types in the quiz | [optional]
**anonymous_submissions** | Option<**bool**> | Whether survey submissions will be kept anonymous (only applicable to 'graded_survey', 'survey' quiz types) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


