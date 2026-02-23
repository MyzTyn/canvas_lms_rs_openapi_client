# DiscussionTopic

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**id** | Option<**i32**> | The ID of this topic. | [optional]
**title** | Option<**String**> | The topic title. | [optional]
**message** | Option<**String**> | The HTML content of the message body. | [optional]
**html_url** | Option<**String**> | The URL to the discussion topic in canvas. | [optional]
**posted_at** | Option<**String**> | The datetime the topic was posted. If it is null it hasn't been posted yet. (see delayed_post_at) | [optional]
**last_reply_at** | Option<**String**> | The datetime for when the last reply was in the topic. | [optional]
**require_initial_post** | Option<**bool**> | If true then a user may not respond to other replies until that user has made an initial reply. Defaults to false. | [optional]
**user_can_see_posts** | Option<**bool**> | Whether or not posts in this topic are visible to the user. | [optional]
**discussion_subentry_count** | Option<**i32**> | The count of entries in the topic. | [optional]
**read_state** | Option<**ReadState**> | The read_state of the topic for the current user, 'read' or 'unread'. (enum: read, unread) | [optional]
**unread_count** | Option<**i32**> | The count of unread entries of this topic for the current user. | [optional]
**subscribed** | Option<**bool**> | Whether or not the current user is subscribed to this topic. | [optional]
**subscription_hold** | Option<**SubscriptionHold**> | (Optional) Why the user cannot subscribe to this topic. Only one reason will be returned even if multiple apply. Can be one of: 'initial_post_required': The user must post a reply first; 'not_in_group_set': The user is not in the group set for this graded group discussion; 'not_in_group': The user is not in this topic's group; 'topic_is_announcement': This topic is an announcement (enum: initial_post_required, not_in_group_set, not_in_group, topic_is_announcement) | [optional]
**assignment_id** | Option<**i32**> | The unique identifier of the assignment if the topic is for grading, otherwise null. | [optional]
**delayed_post_at** | Option<**String**> | The datetime to publish the topic (if not right away). | [optional]
**published** | Option<**bool**> | Whether this discussion topic is published (true) or draft state (false) | [optional]
**lock_at** | Option<**String**> | The datetime to lock the topic (if ever). | [optional]
**locked** | Option<**bool**> | Whether or not the discussion is 'closed for comments'. | [optional]
**pinned** | Option<**bool**> | Whether or not the discussion has been 'pinned' by an instructor | [optional]
**locked_for_user** | Option<**bool**> | Whether or not this is locked for the user. | [optional]
**lock_info** | Option<[**models::LockInfo**](LockInfo.md)> |  | [optional]
**lock_explanation** | Option<**String**> | (Optional) An explanation of why this is locked for the user. Present when locked_for_user is true. | [optional]
**user_name** | Option<**String**> | The username of the topic creator. | [optional]
**topic_children** | Option<**Vec<i32>**> | DEPRECATED An array of topic_ids for the group discussions the user is a part of. | [optional]
**group_topic_children** | Option<**Vec<serde_json::Value>**> | An array of group discussions the user is a part of. Fields include: id, group_id | [optional]
**root_topic_id** | Option<**i32**> | If the topic is for grading and a group assignment this will point to the original topic in the course. | [optional]
**podcast_url** | Option<**String**> | If the topic is a podcast topic this is the feed url for the current user. | [optional]
**discussion_type** | Option<**DiscussionType**> | The type of discussion. Values are 'side_comment' or 'not_threaded', for discussions that only allow one level of nested comments, and 'threaded' for fully threaded discussions. (enum: side_comment, not_threaded, threaded) | [optional]
**group_category_id** | Option<**i32**> | The unique identifier of the group category if the topic is a group discussion, otherwise null. | [optional]
**attachments** | Option<[**Vec<models::FileAttachment>**](FileAttachment.md)> | Array of file attachments. | [optional]
**permissions** | Option<**serde_json::Value**> | The current user's permissions on this topic. | [optional]
**allow_rating** | Option<**bool**> | Whether or not users can rate entries in this topic. | [optional]
**only_graders_can_rate** | Option<**bool**> | Whether or not grade permissions are required to rate entries. | [optional]
**sort_by_rating** | Option<**bool**> | DEPRECATED, Whether or not entries should be sorted by rating. | [optional]
**sort_order** | Option<**SortOrder**> | How entries should be sorted by default. (enum: asc, desc) | [optional]
**sort_order_locked** | Option<**bool**> | Can users decide their preferred sort order. | [optional]
**expand** | Option<**bool**> | Threaded replies should be expanded by default. | [optional]
**expand_locked** | Option<**bool**> | Can users decide their preferred thread expand setting. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


