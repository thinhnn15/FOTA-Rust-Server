-- Create 'devices' table
CREATE TABLE devices (
    id UUID PRIMARY KEY,
    device_id TEXT NOT NULL UNIQUE,
    type TEXT,
    name TEXT,
    firmware_version TEXT,
    last_seen TIMESTAMP,
    ip TEXT
);

-- Create 'firmwares' table
CREATE TABLE firmwares (
    id UUID PRIMARY KEY,
    device_type TEXT,
    version TEXT,
    url TEXT,
    checksum TEXT,
    uploaded_by TEXT,
    uploaded_at TIMESTAMP
);

-- Create 'updates' table
CREATE TABLE updates (
    id UUID PRIMARY KEY,
    device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    firmware_id UUID REFERENCES firmwares(id) ON DELETE CASCADE,
    sent_at TIMESTAMP
);

-- Create 'update_results' table
CREATE TABLE update_results (
    id UUID PRIMARY KEY,
    device_id UUID REFERENCES devices(id) ON DELETE CASCADE,
    firmware_version TEXT,
    status TEXT,
    timestamp TIMESTAMP
);