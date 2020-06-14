# DB Migration

1. Modify `schema/schema.sql` to reflect the desired state after update.
2. Create migration SQL in `schema` directory. It won't be automatically consumed by any programs, but just in case
   I will need to refer to how I made a change in the past.
   The file name should start with `YYYY-mm-dd`.
3. Connect to Postgres.
4. Run migration in a transaction. Start transaction with `BEGIN;` and commit it with `COMMIT;`.