-- Change devices.last_seen column to TIMESTAMPTZ

ALTER TABLE devices
ALTER COLUMN last_seen TYPE TIMESTAMPTZ
USING last_seen AT TIME ZONE 'UTC';
