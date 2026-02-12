# \WebhooksApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_by_org_by_repo_webhooks_by_webhook_id**](WebhooksApi.md#delete_by_org_by_repo_webhooks_by_webhook_id) | **DELETE** /{org}/{repo}/webhooks/{webhookId} | Delete webhook
[**get_by_org_by_repo_webhooks**](WebhooksApi.md#get_by_org_by_repo_webhooks) | **GET** /{org}/{repo}/webhooks | List webhooks
[**post_by_org_by_repo_webhooks**](WebhooksApi.md#post_by_org_by_repo_webhooks) | **POST** /{org}/{repo}/webhooks | Create webhook



## delete_by_org_by_repo_webhooks_by_webhook_id

> models::DeleteByOrgApiKeysById200Response delete_by_org_by_repo_webhooks_by_webhook_id(org, repo, webhook_id)
Delete webhook

Delete a webhook from a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**webhook_id** | Option<**String**> |  | [required] |

### Return type

[**models::DeleteByOrgApiKeysById200Response**](deleteByOrgApiKeysById_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo_webhooks

> models::GetByOrgByRepoWebhooks200Response get_by_org_by_repo_webhooks(org, repo)
List webhooks

List webhooks for a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::GetByOrgByRepoWebhooks200Response**](getByOrgByRepoWebhooks_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_webhooks

> models::PostByOrgByRepoWebhooks201Response post_by_org_by_repo_webhooks(org, repo, post_by_org_by_repo_webhooks_request)
Create webhook

Create a webhook for a repository

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**post_by_org_by_repo_webhooks_request** | Option<[**PostByOrgByRepoWebhooksRequest**](PostByOrgByRepoWebhooksRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoWebhooks201Response**](postByOrgByRepoWebhooks_201_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

