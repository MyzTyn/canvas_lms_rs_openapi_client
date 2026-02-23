# Page

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**page_id** | Option<**i32**> | the ID of the page | [optional]
**url** | Option<**String**> | the unique locator for the page | [optional]
**title** | Option<**String**> | the title of the page | [optional]
**created_at** | Option<**String**> | the creation date for the page | [optional]
**updated_at** | Option<**String**> | the date the page was last updated | [optional]
**hide_from_students** | Option<**bool**> | (DEPRECATED) whether this page is hidden from students (note: this is always reflected as the inverse of the published value) | [optional]
**editing_roles** | Option<**String**> | roles allowed to edit the page; comma-separated list comprising a combination of 'teachers', 'students', 'members', and/or 'public' if not supplied, course defaults are used | [optional]
**last_edited_by** | Option<[**models::User**](User.md)> |  | [optional]
**body** | Option<**String**> | the page content, in HTML (present when requesting a single page; optionally included when listing pages) | [optional]
**published** | Option<**bool**> | whether the page is published (true) or draft state (false). | [optional]
**publish_at** | Option<**String**> | scheduled publication date for this page | [optional]
**front_page** | Option<**bool**> | whether this page is the front page for the wiki | [optional]
**locked_for_user** | Option<**bool**> | Whether or not this is locked for the user. | [optional]
**lock_info** | Option<[**models::LockInfo**](LockInfo.md)> |  | [optional]
**lock_explanation** | Option<**String**> | (Optional) An explanation of why this is locked for the user. Present when locked_for_user is true. | [optional]
**editor** | Option<**Editor**> | The editor used to create and edit this page. May be one of 'rce' or 'block_editor'. (enum: rce, block_editor) | [optional]
**block_editor_attributes** | Option<**serde_json::Value**> | The block editor attributes for this page. (optionally included, and only if this is a block editor created page) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


