# \BonusRestapiOperationsApi

All URIs are relative to *http://localhost:8080*

Method | HTTP request | Description
------------- | ------------- | -------------
[**edit_bonus**](BonusRestapiOperationsApi.md#edit_bonus) | **PATCH** /api/v1/privilege/{id} | Изменить значение количества бонусов
[**get_bonus**](BonusRestapiOperationsApi.md#get_bonus) | **GET** /api/v1/privilege/{id} | Get Bonus by ID
[**list_bonus_history**](BonusRestapiOperationsApi.md#list_bonus_history) | **GET** /api/v1/privilege_history | Get all Bonuses
[**list_bonuses**](BonusRestapiOperationsApi.md#list_bonuses) | **GET** /api/v1/privilege | Get all Bonuses



## edit_bonus

> crate::models::PrivilegeResponse edit_bonus(id, privilege_request)
Изменить значение количества бонусов

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |
**privilege_request** | [**PrivilegeRequest**](PrivilegeRequest.md) |  | [required] |

### Return type

[**crate::models::PrivilegeResponse**](PrivilegeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_bonus

> crate::models::PrivilegeResponse get_bonus(id)
Get Bonus by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **i32** |  | [required] |

### Return type

[**crate::models::PrivilegeResponse**](PrivilegeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bonus_history

> Vec<crate::models::BalanceHistory> list_bonus_history(username, ticket_uid)
Get all Bonuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**ticket_uid** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::BalanceHistory>**](BalanceHistory.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_bonuses

> Vec<crate::models::PrivilegeResponse> list_bonuses(username)
Get all Bonuses

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::PrivilegeResponse>**](PrivilegeResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

