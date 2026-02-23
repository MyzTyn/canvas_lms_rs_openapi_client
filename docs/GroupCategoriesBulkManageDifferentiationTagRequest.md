# GroupCategoriesBulkManageDifferentiationTagRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operations** | **String** | A hash containing arrays of create/update/delete operations: {   \"create\": [     { \"name\": \"New Group A\" },     { \"name\": \"New Group B\" }   ],   \"update\": [     { \"id\": 123, \"name\": \"Updated Group Name A\" },     { \"id\": 456, \"name\": \"Updated Group Name B\" }   ],   \"delete\": [     { \"id\": 789 },     { \"id\": 101 }   ] } | 
**group_category** | **String** | Attributes for the GroupCategory. May include:   - id [Optional, Integer]: The ID of an existing GroupCategory.   - name [Optional, String]: A new name for the GroupCategory. If provided with an ID, the category name will be updated. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


