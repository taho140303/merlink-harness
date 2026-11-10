-- Ensure every granted planning tool carries a callable JSON schema. Write handlers validate the
-- real field names, so the schema is intentionally permissive: it lets the model emit the
-- documented parameters without the backend inventing per-tool required lists.
UPDATE tools
SET input_schema = '{"type":"object","additionalProperties":true,"properties":{}}'
WHERE input_schema IS NULL;
