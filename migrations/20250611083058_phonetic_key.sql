-- Add phonetic_key column to people table
ALTER TABLE people ADD COLUMN phonetic_key TEXT;

-- Optional: Create index for better search performance
CREATE INDEX IF NOT EXISTS idx_people_phonetic_key ON people(phonetic_key);

-- Optional: Update existing records with phonetic keys
-- UPDATE people SET phonetic_key = SOUNDEX(name) WHERE phonetic_key IS NULL;