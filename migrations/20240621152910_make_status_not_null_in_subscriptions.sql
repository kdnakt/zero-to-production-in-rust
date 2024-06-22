-- Add migration script here
BEGIN;
    UPDATE subscriptions
        SET status = 'comfirmed'
        WHERE status IS NULL;
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;
COMMIT;