-- Add migration script here
PRAGMA foreign_keys = OFF;
ALTER TABLE people ADD COLUMN verified INTEGER;
PRAGMA foreign_keys = ON;

