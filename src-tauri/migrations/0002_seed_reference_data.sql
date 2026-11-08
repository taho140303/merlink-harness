-- =============================================================================
-- Migration 0002: Seed Reference Data
-- Authority: docs/business/be-core.md (§3.4, §12 MIG-01..05)
-- Target PRAGMA user_version: 2
-- Note: Idempotent reference data seeding for reference catalogs.
-- =============================================================================

-- Idempotent seed for standard reference data (e.g. system genres)
INSERT OR IGNORE INTO genres (id, code, display_name, description) VALUES
    ('018e3a00-0000-7000-8000-000000000001', 'fantasy', 'Fantasy', 'Fantasy genre and subgenres'),
    ('018e3a00-0000-7000-8000-000000000002', 'sci_fi', 'Science Fiction', 'Science Fiction genre and subgenres'),
    ('018e3a00-0000-7000-8000-000000000003', 'mystery', 'Mystery', 'Mystery and detective fiction');

PRAGMA user_version = 2;
