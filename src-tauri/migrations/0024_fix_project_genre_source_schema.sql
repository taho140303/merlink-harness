-- 0024: fix the add_project_genre schema published by 0023.
--
-- 0023 declared the scalar `declared_source`, but the executor's SourceParamMode is
-- MapDeclaredSources and reads `declared_sources.genre_code`. The model followed the schema,
-- every genre write was rejected with `missing_or_invalid_declared_source`, project.genres stayed
-- empty and the readiness gate kept the draft closed even though get_field_null returned no gaps.

UPDATE tools SET input_schema = '{"type":"object","properties":{"genre_code":{"type":"string"},"is_primary":{"type":"boolean"},"declared_sources":{"type":"object","properties":{"genre_code":{"type":"string","enum":["user","ai_generated"]},"is_primary":{"type":"string","enum":["user","ai_generated"]}},"required":["genre_code"]}},"required":["genre_code","declared_sources"]}' WHERE code='add_project_genre';

PRAGMA user_version = 24;
