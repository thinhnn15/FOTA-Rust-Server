# 📚 Database Design Proposal for Rust MQTT FOTA Server

## 1. Table: `devices`

Manages the registered devices.

| Field | Type | Description |
|:------|:-----|:------------|
| id | UUID (Primary Key) | Unique Device ID |
| device_id | TEXT | Device ID reported by the client |
| type | TEXT | Device Type (e.g., sensor, arm, x86) |
| name | TEXT | Friendly Name |
| firmware_version | TEXT | Current firmware version |
| last_seen | TIMESTAMP | Last communication timestamp |
| ip | TEXT | Last known IP Address |

---

## 2. Table: `firmwares`

Stores information about uploaded firmware files.

| Field | Type | Description |
|:------|:-----|:------------|
| id | UUID (Primary Key) | Unique Firmware ID |
| device_type | TEXT | Applicable device type |
| version | TEXT | Firmware semantic version |
| url | TEXT | URL to download the firmware |
| checksum | TEXT | SHA256 checksum |
| uploaded_by | TEXT | Username of uploader |
| uploaded_at | TIMESTAMP | Upload timestamp |

---

## 3. Table: `updates`

Tracks firmware update triggers per device.

| Field | Type | Description |
|:------|:-----|:------------|
| id | UUID (Primary Key) | Update ID |
| device_id | UUID (Foreign Key -> devices.id) | Target device ID |
| firmware_id | UUID (Foreign Key -> firmwares.id) | Firmware being pushed |
| sent_at | TIMESTAMP | Timestamp when the update was triggered |

---

## 4. Table: `update_results`

Logs the update result received from devices.

| Field | Type | Description |
|:------|:-----|:------------|
| id | UUID (Primary Key) | Result ID |
| device_id | UUID (Foreign Key -> devices.id) | Reporting device ID |
| firmware_version | TEXT | Firmware version reported by the device |
| status | TEXT | Update status: success / failed / partial |
| timestamp | TIMESTAMP | When the update result was received |

---

# 🔗 Entity Relationships (ERD Concept)

```plaintext
devices (1) --- (many) updates
devices (1) --- (many) update_results
firmwares (1) --- (many) updates
```

---

# 🛠️ Database Technology Stack

| Layer | Technology |
|:------|:-----------|
| Database Client | sqlx |
| UUID Generation | uuid crate |
| Timestamp Handling | chrono crate |
| Migration Tool | sqlx-cli |
| Database Engines | SQLite (Development) / PostgreSQL (Production) |

---

# 📋 Notes

- UUIDs are used as primary keys for global uniqueness.

- TIMESTAMP fields store UTC time.

- Foreign keys ensure referential integrity.

- Design is optimized for scalability and reporting needs.


---

# 📈 Quick dbdiagram.io Schema

```plaintext
Table devices {
  id UUID [pk]
  device_id TEXT [not null, unique]
  type TEXT
  name TEXT
  firmware_version TEXT
  last_seen TIMESTAMP
  ip TEXT
}

Table firmwares {
  id UUID [pk]
  device_type TEXT
  version TEXT
  url TEXT
  checksum TEXT
  uploaded_by TEXT
  uploaded_at TIMESTAMP
}

Table updates {
  id UUID [pk]
  device_id UUID [ref: > devices.id]
  firmware_id UUID [ref: > firmwares.id]
  sent_at TIMESTAMP
}

Table update_results {
  id UUID [pk]
  device_id UUID [ref: > devices.id]
  firmware_version TEXT
  status TEXT
  timestamp TIMESTAMP
}
```

---

# ✅ Designed for Rust MQTT Backend Server (April 2025)