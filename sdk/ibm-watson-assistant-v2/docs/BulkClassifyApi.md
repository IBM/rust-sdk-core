# \BulkClassifyApi

All URIs are relative to *https://api.us-south.assistant.watson.cloud.ibm.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**bulk_classify**](BulkClassifyApi.md#bulk_classify) | **post** /v2/skills/{skill_id}/workspace/bulk_classify | Identify intents and entities in multiple user utterances



## bulk_classify

> crate::models::BulkClassifyResponse bulk_classify(skill_id, version, request)
Identify intents and entities in multiple user utterances

Send multiple user inputs to a dialog skill in a single request and receive information about the intents and entities recognized in each input. This method is useful for testing and comparing the performance of different skills or skill versions.  This method is available only with Enterprise with Data Isolation plans.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**skill_id** | **String** | Unique identifier of the skill. To find the skill ID in the Watson Assistant user interface, open the skill settings and click **API Details**. | [required] |
**version** | **String** | Release date of the API version you want to use. Specify dates in YYYY-MM-DD format. The current version is `2020-09-24`. | [required] |
**request** | Option<[**BulkClassifyInput**](BulkClassifyInput.md)> | An input object that includes the text to classify. |  |

### Return type

[**crate::models::BulkClassifyResponse**](BulkClassifyResponse.md)

### Authorization

[IAM](../README.md#IAM)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

