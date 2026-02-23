# RubricsCreateRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The id of the rubric | [optional]
**rubric_association_id** | Option<**i32**> | The id of the rubric association object (not the course/assignment itself, but the join table record id). It can be used in place of +rubric_association[association_id]+ and +rubric_association[association_type]+ if desired. | [optional]
**rubric** | Option<[**models::RubricsCreateRequestRubric**](RubricsCreateRequestRubric.md)> |  | [optional]
**rubric_association** | Option<[**models::RubricsCreateRequestRubricAssociation**](RubricsCreateRequestRubricAssociation.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


