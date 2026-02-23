# SubmissionsApiUpdateAnonymousForCoursesRequestSubmission

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**posted_grade** | Option<**String**> | [String] Assign a score to the submission, updating both the \"score\" and \"grade\" fields on the submission record. This parameter can be passed in a few different formats:  points:: A floating point or integral value, such as \"13.5\". The grade   will be interpreted directly as the score of the assignment.   Values above assignment.points_possible are allowed, for awarding   extra credit. percentage:: A floating point value appended with a percent sign, such as    \"40%\". The grade will be interpreted as a percentage score on the    assignment, where 100% == assignment.points_possible. Values above 100%    are allowed, for awarding extra credit. letter grade:: A letter grade, following the assignment's defined letter    grading scheme. For example, \"A-\". The resulting score will be the high    end of the defined range for the letter grade. For instance, if \"B\" is    defined as 86% to 84%, a letter grade of \"B\" will be worth 86%. The    letter grade will be rejected if the assignment does not have a defined    letter grading scheme. For more fine-grained control of scores, pass in    points or percentage rather than the letter grade. \"pass/complete/fail/incomplete\":: A string value of \"pass\" or \"complete\"    will give a score of 100%. \"fail\" or \"incomplete\" will give a score of    0.  Note that assignments with grading_type of \"pass_fail\" can only be assigned a score of 0 or assignment.points_possible, nothing inbetween. If a posted_grade in the \"points\" or \"percentage\" format is sent, the grade will only be accepted if the grade equals one of those two values. | [optional]
**excuse** | Option<**bool**> | [Boolean] Sets the \"excused\" status of an assignment. | [optional]
**late_policy_status** | Option<**String**> | [String] Sets the late policy status to either \"late\", \"missing\", \"extended\", \"none\", or null.   NB: \"extended\" values can only be set in the UI when the \"UI features for 'extended' Submissions\" Account Feature is on | [optional]
**seconds_late_override** | Option<**i32**> | [Integer] Sets the seconds late if late policy status is \"late\" | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


