# Requirement Analysis Document

## Module: Rust MQTT Backend Server

**Version:** 0.1  
**Date:** April 2025  
**Prepared by:** Thinh Nguyen

---

## 1. Purpose
This document defines the technical requirements and expected behavior of the Rust backend server that manages MQTT communication with connected C++ client devices. The server is responsible for managing device registration, pushing firmware updates, handling device status reports, and storing update results.

## 2. Scope
This backend module provides:
- A secure MQTT interface
- Device metadata management
- Update notification publishing
- Logging and auditing of firmware update results
- Optional REST API for device management dashboard

## 3. Actors
| Actor | Description |
| :--- | :--- |
| Device Client | C++ application running on embedded device |
| MQTT Broker | Middleware handling the message routing |
| Backend Server | Rust-based service handling MQTT subscriptions and publishing updates |
| Admin/DevOps | Person managing firmware rollouts via optional dashboard/API |

## 4. Functional Requirements

### 4.1 Device Registration Handler
- **ID:** SR-001
- **Name:** Track Registered Devices
- **Description:** Server stores metadata of devices during or after registration
- **Fields:** device_id, type, name, status, last_seen, firmware_version
- **Storage:** PostgreSQL or SQLite (configurable per deployment)

### 4.2 Status Listener
- **ID:** SR-002
- **Name:** Handle Status Messages
- **Topic:** `device/{device_id}/status`
- **Payload:**
```json
{
  "version": "1.2.0",
  "url": "https://example.com/fw/device123.bin",
  "checksum": "abc123def456"
}
```
- **Behavior:** Update database record, store timestamp, log metrics
- **Trigger:** Periodically from client (e.g., every 10 minutes)

### 4.3 Update Publisher
- **ID:** SR-003
- **Name:** Push Firmware Update
- **Topic:** `device/{device_id}/update`
- **Payload:**
```json
{
  "version": "1.2.0",
  "url": "https://example.com/fw/device123.bin",
  "checksum": "abc123def456"
}
```
- **Trigger:** Admin action or automated rollout policy

### 4.4 Update Result Listener
- **ID:** SR-004
- **Name:** Handle Update Results
- **Topic:** `device/{device_id}/update/result`
- **Payload:**
```json
{
  "status": "success",
  "version": "1.2.0",
  "timestamp": 1684325472
}
```
- **Behavior:** Store result in DB, notify admin via dashboard or webhook

## 5. MQTT Topic Summary
| Topic | Direction | Auth | QoS | Description |
| :--- | :--- | :--- | :--- | :--- |
| device/+/status | Subscribe | Yes | 1 | Receive device status |
| device/+/update/result | Subscribe | Yes | 1 | Receive firmware update result |
| device/+/update | Publish | Yes | 1 | Send firmware update |

## 6. Security Requirements
- **Protocol:** MQTT over TLS (port 8883)
- **Authentication:** Username/password or X.509 client certificate
- **Authorization:** Per-topic access control (ACL)
- **Brute-force Protection:** IP rate limiting
- **Logging:** All publish/subscription attempts must be logged
