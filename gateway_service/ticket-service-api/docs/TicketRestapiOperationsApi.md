# \TicketRestapiOperationsApi

All URIs are relative to *http://localhost:8070*

Method | HTTP request | Description
------------- | ------------- | -------------
[**create_ticket**](TicketRestapiOperationsApi.md#create_ticket) | **POST** /api/v1/tickets | Create new Ticket
[**edit_ticket**](TicketRestapiOperationsApi.md#edit_ticket) | **PATCH** /api/v1/tickets/{ticketUid} | Update Ticket by ID
[**edit_ticket1**](TicketRestapiOperationsApi.md#edit_ticket1) | **DELETE** /api/v1/tickets/{ticketUid} | Remove Ticket by ID
[**get_ticket**](TicketRestapiOperationsApi.md#get_ticket) | **GET** /api/v1/tickets/{ticketUid} | Get Ticket by ID
[**list_tickets**](TicketRestapiOperationsApi.md#list_tickets) | **GET** /api/v1/tickets | Get all Tickets



## create_ticket

> create_ticket(ticket_request)
Create new Ticket

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_request** | [**TicketRequest**](TicketRequest.md) |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_ticket

> crate::models::TicketResponse edit_ticket(ticket_uid, ticket_request)
Update Ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_uid** | **uuid::Uuid** |  | [required] |
**ticket_request** | [**TicketRequest**](TicketRequest.md) |  | [required] |

### Return type

[**crate::models::TicketResponse**](TicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## edit_ticket1

> edit_ticket1(ticket_uid)
Remove Ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_uid** | **uuid::Uuid** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_ticket

> crate::models::TicketResponse get_ticket(ticket_uid)
Get Ticket by ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**ticket_uid** | **uuid::Uuid** |  | [required] |

### Return type

[**crate::models::TicketResponse**](TicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## list_tickets

> Vec<crate::models::TicketResponse> list_tickets(username, flight_number)
Get all Tickets

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**username** | Option<**String**> |  |  |
**flight_number** | Option<**String**> |  |  |

### Return type

[**Vec<crate::models::TicketResponse>**](TicketResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

