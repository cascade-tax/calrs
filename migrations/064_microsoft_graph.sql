-- Microsoft 365 / Office 365 calendar integration through Microsoft Graph.
-- Client credentials are instance-wide; each calendar source stores its own
-- encrypted delegated access and refresh tokens in the existing OAuth columns.
ALTER TABLE auth_config ADD COLUMN microsoft_oauth2_client_id TEXT;
ALTER TABLE auth_config ADD COLUMN microsoft_oauth2_client_secret TEXT;
ALTER TABLE auth_config ADD COLUMN microsoft_oauth2_tenant TEXT NOT NULL DEFAULT 'organizations';
