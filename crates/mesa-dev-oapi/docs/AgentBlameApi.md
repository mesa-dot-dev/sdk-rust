# \AgentBlameApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_by_org_by_repo_agentblame**](AgentBlameApi.md#get_by_org_by_repo_agentblame) | **GET** /{org}/{repo}/agentblame | Get AI attribution data
[**get_by_org_by_repo_analytics**](AgentBlameApi.md#get_by_org_by_repo_analytics) | **GET** /{org}/{repo}/analytics | Get repository analytics
[**post_by_org_by_repo_analytics_refresh**](AgentBlameApi.md#post_by_org_by_repo_analytics_refresh) | **POST** /{org}/{repo}/analytics/refresh | Refresh repository analytics



## get_by_org_by_repo_agentblame

> models::GetByOrgByRepoAgentblame200Response get_by_org_by_repo_agentblame(org, repo, base, head)
Get AI attribution data

Retrieve agentblame attribution data for commits between two refs

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**base** | **String** |  | [required] |
**head** | **String** |  | [required] |

### Return type

[**models::GetByOrgByRepoAgentblame200Response**](getByOrgByRepoAgentblame_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_by_org_by_repo_analytics

> models::GetByOrgByRepoAnalytics200Response get_by_org_by_repo_analytics(org, repo, period)
Get repository analytics

Retrieve repository-wide AI attribution analytics from agentblame

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**period** | Option<**String**> |  |  |[default to all]

### Return type

[**models::GetByOrgByRepoAnalytics200Response**](getByOrgByRepoAnalytics_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_analytics_refresh

> models::GetByOrgByRepoAnalytics200Response post_by_org_by_repo_analytics_refresh(org, repo)
Refresh repository analytics

Trigger a full re-aggregation of repository analytics

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |

### Return type

[**models::GetByOrgByRepoAnalytics200Response**](getByOrgByRepoAnalytics_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

