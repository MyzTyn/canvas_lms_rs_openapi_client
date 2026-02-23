# SubmissionsApiUpdateForCoursesRequestRubricAssessment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**rubric_assessment** | Option<**String**> | Assign a rubric assessment to this assignment submission. The sub-parameters here depend on the rubric for the assignment. The general format is, for each row in the rubric:  The points awarded for this row.   rubric_assessment[criterion_id][points]  The rating id for the row.   rubric_assessment[criterion_id][rating_id]  Comments to add for this row.   rubric_assessment[criterion_id][comments]  For example, if the assignment rubric is (in JSON format):   !!!javascript   [     {       'id': 'crit1',       'points': 10,       'description': 'Criterion 1',       'ratings':       [         { 'id': 'rat1', 'description': 'Good', 'points': 10 },         { 'id': 'rat2', 'description': 'Poor', 'points': 3 }       ]     },     {       'id': 'crit2',       'points': 5,       'description': 'Criterion 2',       'ratings':       [         { 'id': 'rat1', 'description': 'Exemplary', 'points': 5 },         { 'id': 'rat2', 'description': 'Complete', 'points': 5 },         { 'id': 'rat3', 'description': 'Incomplete', 'points': 0 }       ]     }   ]  Then a possible set of values for rubric_assessment would be:     rubric_assessment[crit1][points]=3&rubric_assessment[crit1][rating_id]=rat1&rubric_assessment[crit2][points]=5&rubric_assessment[crit2][rating_id]=rat2&rubric_assessment[crit2][comments]=Well%20Done. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


