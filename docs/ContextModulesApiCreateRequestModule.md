# ContextModulesApiCreateRequestModule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**name** | **String** | [Required, String] The name of the module | 
**unlock_at** | Option<**String**> | [DateTime] The date the module will unlock | [optional]
**position** | Option<**i32**> | [Integer] The position of this module in the course (1-based) | [optional]
**require_sequential_progress** | Option<**bool**> | [Boolean] Whether module items must be unlocked in order | [optional]
**prerequisite_module_ids** | Option<**String**> | [] [String] IDs of Modules that must be completed before this one is unlocked. Prerequisite modules must precede this module (i.e. have a lower position value), otherwise they will be ignored | [optional]
**publish_final_grade** | Option<**bool**> | [Boolean] Whether to publish the student's final grade for the course upon completion of this module. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


