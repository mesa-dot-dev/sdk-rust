# \LfsApi

All URIs are relative to *https://depot.mesa.dev/api/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_by_org_by_repo_lfs_objects**](LfsApi.md#post_by_org_by_repo_lfs_objects) | **POST** /{org}/{repo}/lfs/objects | Upload LFS objects
[**post_by_org_by_repo_lfs_objects_download**](LfsApi.md#post_by_org_by_repo_lfs_objects_download) | **POST** /{org}/{repo}/lfs/objects/download | Download LFS objects



## post_by_org_by_repo_lfs_objects

> models::PostByOrgByRepoLfsObjects200Response post_by_org_by_repo_lfs_objects(org, repo, post_by_org_by_repo_lfs_objects_request)
Upload LFS objects

Request pre-signed URLs to upload large files to LFS storage. After uploading, use the commit endpoint with LFS file references to create commits.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**post_by_org_by_repo_lfs_objects_request** | Option<[**PostByOrgByRepoLfsObjectsRequest**](PostByOrgByRepoLfsObjectsRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoLfsObjects200Response**](postByOrgByRepoLfsObjects_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_by_org_by_repo_lfs_objects_download

> models::PostByOrgByRepoLfsObjects200Response post_by_org_by_repo_lfs_objects_download(org, repo, post_by_org_by_repo_lfs_objects_download_request)
Download LFS objects

Request pre-signed URLs to download large files from LFS storage.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org** | **String** |  | [required] |
**repo** | **String** |  | [required] |
**post_by_org_by_repo_lfs_objects_download_request** | Option<[**PostByOrgByRepoLfsObjectsDownloadRequest**](PostByOrgByRepoLfsObjectsDownloadRequest.md)> |  |  |

### Return type

[**models::PostByOrgByRepoLfsObjects200Response**](postByOrgByRepoLfsObjects_200_response.md)

### Authorization

[apiKey](../README.md#apiKey)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

