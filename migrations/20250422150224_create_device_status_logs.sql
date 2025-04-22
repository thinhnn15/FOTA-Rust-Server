-- Create table device_status_logs
CREATE TABLE device_status_logs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    device_id TEXT NOT NULL,
    payload JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT now()
);

-- Optional index for faster queries
CREATE INDEX idx_device_status_logs_device_id ON device_status_logs(device_id);
CREATE INDEX idx_device_status_logs_created_at ON device_status_logs(created_at);
