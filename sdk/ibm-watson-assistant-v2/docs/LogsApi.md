# \LogsApi

All URIs are relative to *https://api.us-south.assistant.watson.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**list_logs**](LogsApi.md#list_logs) | **get** /v2/assistants/{assistant_id}/logs | List log events for an assistant



## list_logs

> crate::models::LogCollection list_logs(assistant_id, version, sort, filter, page_limit, cursor)
List log events for an assistant

List the events from the log of an assistant.  This method requires Manager access, and is available only with Enterprise plans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**assistant_id** | **String** | Unique identifier of the assistant. To find the assistant ID in the Watson Assistant user interface, open the assistant settings and click **API Details**. For information about creating assistants, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-assistant-add#assistant-add-task).  **Note:** Currently, the v2 API does not support creating assistants. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**sort** | Option<**String**> | How to sort the returned log events. You can sort by **request_timestamp**. To reverse the sort order, prefix the parameter value with a minus sign (`-`). |  |
**filter** | Option<**String**> | A cacheable parameter that limits the results to those matching the specified filter. For more information, see the [documentation](https://cloud.ibm.com/docs/assistant?topic=assistant-filter-reference#filter-reference). |  |
**page_limit** | Option<**i32**> | The number of records to return in each page of results. |  |[default to 100]
**cursor** | Option<**String**> | A token identifying the page of results to retrieve. |  |

### Return type

[**crate::models::LogCollection**](LogCollection.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

