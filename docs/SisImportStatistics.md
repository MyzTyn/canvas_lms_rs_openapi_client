# SisImportStatistics

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**total_state_changes** | Option<**i32**> | This is the total number of items that were changed in the sis import. There are a few caveats that can cause this number to not add up to the individual counts. There are some state changes that happen that have no impact to the object. An example would be changing a course from 'created' to 'claimed'. Both of these would be considered an active course, but would increment this counter. In this example the course would not increment the created or restored counters for course statistic. | [optional]
**account** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**enrollment_term** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**communication_channel** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**abstract_course** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**course** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**course_section** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**enrollment** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**group_category** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**group** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**group_membership** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**pseudonym** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**user_observer** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]
**account_user** | Option<[**models::SisImportStatistic**](SisImportStatistic.md)> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


