PRAGMA foreign_keys = off;

-- Table: aliases
DROP TABLE IF EXISTS aliases;

CREATE TABLE IF NOT EXISTS aliases (
    id        INTEGER NOT NULL,
    name      TEXT    NOT NULL,
    person_id INTEGER NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    ),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
);


-- Table: books
DROP TABLE IF EXISTS books;

CREATE TABLE IF NOT EXISTS books (
    id                 INTEGER NOT NULL,
    name               TEXT    NOT NULL,
    publisher_id       INTEGER,
    format_id          INTEGER,
    publication_date   BIGINT,
    acquisition_date   BIGINT,
    last_modified_date BIGINT,
    series_id          INTEGER,
    series_index       INTEGER,
    original_title     TEXT,
    notes              TEXT,
    has_cover          INTEGER,
    has_paper          INTEGER,
    file_link          TEXT    UNIQUE,
    pre_accepted       INTEGER DEFAULT (1),
    FOREIGN KEY (
        publisher_id
    )
    REFERENCES publishers (id),
    FOREIGN KEY (
        format_id
    )
    REFERENCES formats (id),
    FOREIGN KEY (
        series_id
    )
    REFERENCES series (id),
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: books_contents
DROP TABLE IF EXISTS books_contents;

CREATE TABLE IF NOT EXISTS books_contents (
    book_id    INTEGER NOT NULL,
    content_id INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        content_id
    ),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id) 
);


-- Table: books_people_roles
DROP TABLE IF EXISTS books_people_roles;

CREATE TABLE IF NOT EXISTS books_people_roles (
    book_id   INTEGER NOT NULL,
    person_id INTEGER NOT NULL,
    role_id   INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        person_id,
        role_id
    ),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id),
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id) 
);


-- Table: books_tags
DROP TABLE IF EXISTS books_tags;

CREATE TABLE IF NOT EXISTS books_tags (
    book_id INTEGER NOT NULL,
    tag_id  INTEGER NOT NULL,
    PRIMARY KEY (
        book_id,
        tag_id
    ),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id),
    FOREIGN KEY (
        book_id
    )
    REFERENCES books (id) 
);


-- Table: contents
DROP TABLE IF EXISTS contents;

CREATE TABLE IF NOT EXISTS contents (
    id               INTEGER NOT NULL,
    name             TEXT    NOT NULL,
    original_title   TEXT,
    publication_date BIGINT,
    notes            TEXT,
    type_id          INTEGER,
    pre_accepted     INTEGER DEFAULT (1),
    FOREIGN KEY (
        type_id
    )
    REFERENCES types (id),
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: contents_languages
DROP TABLE IF EXISTS contents_languages;

CREATE TABLE IF NOT EXISTS contents_languages (
    contents_id  INTEGER,
    languages_id INTEGER,
    FOREIGN KEY (
        contents_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        languages_id
    )
    REFERENCES running_languages (id),
    PRIMARY KEY (
        contents_id,
        languages_id
    )
);


-- Table: contents_people_roles
DROP TABLE IF EXISTS contents_people_roles;

CREATE TABLE IF NOT EXISTS contents_people_roles (
    content_id INTEGER NOT NULL,
    person_id  INTEGER NOT NULL,
    role_id    INTEGER NOT NULL,
    FOREIGN KEY (
        role_id
    )
    REFERENCES roles (id),
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        person_id
    )
    REFERENCES people (id),
    PRIMARY KEY (
        content_id,
        person_id,
        role_id
    )
);


-- Table: contents_tags
DROP TABLE IF EXISTS contents_tags;

CREATE TABLE IF NOT EXISTS contents_tags (
    content_id INTEGER NOT NULL,
    tag_id     INTEGER NOT NULL,
    FOREIGN KEY (
        content_id
    )
    REFERENCES contents (id),
    FOREIGN KEY (
        tag_id
    )
    REFERENCES tags (id),
    PRIMARY KEY (
        content_id,
        tag_id
    )
);


-- Table: formats
DROP TABLE IF EXISTS formats;

CREATE TABLE IF NOT EXISTS formats (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: languages_names
DROP TABLE IF EXISTS languages_names;

CREATE TABLE IF NOT EXISTS languages_names (
    id       INTEGER,
    iso_code TEXT    NOT NULL
                     UNIQUE,
    name     TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);

INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 1, 'aar', 'Afar' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 2, 'abk', 'Abkhazian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 3, 'ace', 'Achinese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 4, 'ach', 'Acoli' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 5, 'ada', 'Adangme' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 6, 'ady', 'Adyghe; Adygei' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 7, 'afa', 'Afro-Asiatic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 8, 'afh', 'Afrihili' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 9, 'afr', 'Afrikaans' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 10, 'ain', 'Ainu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 11, 'aka', 'Akan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 12, 'akk', 'Akkadian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 13, 'alb', 'Albanian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 14, 'ale', 'Aleut' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 15, 'alg', 'Algonquian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 16, 'alt', 'Southern Altai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 17, 'amh', 'Amharic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 18, 'ang', 'English, Old (ca.450-1100)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 19, 'anp', 'Angika' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 20, 'apa', 'Apache languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 21, 'ara', 'Arabic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 22, 'arc', 'Official Aramaic (700-300 BCE); Imperial Aramaic (700-300 BCE)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 23, 'arg', 'Aragonese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 24, 'arm', 'Armenian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 25, 'arn', 'Mapudungun; Mapuche' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 26, 'arp', 'Arapaho' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 27, 'art', 'Artificial languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 28, 'arw', 'Arawak' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 29, 'asm', 'Assamese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 30, 'ast', 'Asturian; Bable; Leonese; Asturleonese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 31, 'ath', 'Athapascan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 32, 'aus', 'Australian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 33, 'ava', 'Avaric' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 34, 'ave', 'Avestan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 35, 'awa', 'Awadhi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 36, 'aym', 'Aymara' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 37, 'aze', 'Azerbaijani' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 38, 'bad', 'Banda languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 39, 'bai', 'Bamileke languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 40, 'bak', 'Bashkir' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 41, 'bal', 'Baluchi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 42, 'bam', 'Bambara' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 43, 'ban', 'Balinese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 44, 'baq', 'Basque' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 45, 'bas', 'Basa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 46, 'bat', 'Baltic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 47, 'bej', 'Beja; Bedawiyet' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 48, 'bel', 'Belarusian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 49, 'bem', 'Bemba' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 50, 'ben', 'Bengali' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 51, 'ber', 'Berber languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 52, 'bho', 'Bhojpuri' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 53, 'bih', 'Bihari languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 54, 'bik', 'Bikol' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 55, 'bin', 'Bini; Edo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 56, 'bis', 'Bislama' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 57, 'bla', 'Siksika' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 58, 'bnt', 'Bantu languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 59, 'bos', 'Bosnian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 60, 'bra', 'Braj' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 61, 'bre', 'Breton' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 62, 'btk', 'Batak languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 63, 'bua', 'Buriat' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 64, 'bug', 'Buginese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 65, 'bul', 'Bulgarian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 66, 'bur', 'Burmese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 67, 'byn', 'Blin; Bilin' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 68, 'cad', 'Caddo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 69, 'cai', 'Central American Indian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 70, 'car', 'Galibi Carib' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 71, 'cat', 'Catalan; Valencian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 72, 'cau', 'Caucasian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 73, 'ceb', 'Cebuano' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 74, 'cel', 'Celtic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 75, 'cha', 'Chamorro' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 76, 'chb', 'Chibcha' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 77, 'che', 'Chechen' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 78, 'chg', 'Chagatai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 79, 'chi', 'Chinese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 80, 'chk', 'Chuukese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 81, 'chm', 'Mari' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 82, 'chn', 'Chinook jargon' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 83, 'cho', 'Choctaw' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 84, 'chp', 'Chipewyan; Dene Suline' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 85, 'chr', 'Cherokee' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 86, 'chu', 'Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 87, 'chv', 'Chuvash' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 88, 'chy', 'Cheyenne' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 89, 'cmc', 'Chamic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 90, 'cnr', 'Montenegrin' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 91, 'cop', 'Coptic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 92, 'cor', 'Cornish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 93, 'cos', 'Corsican' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 94, 'cpe', 'Creoles and pidgins, English based' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 95, 'cpf', 'Creoles and pidgins, French-based' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 96, 'cpp', 'Creoles and pidgins, Portuguese-based' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 97, 'cre', 'Cree' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 98, 'crh', 'Crimean Tatar; Crimean Turkish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 99, 'crp', 'Creoles and pidgins' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 100, 'csb', 'Kashubian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 101, 'cus', 'Cushitic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 102, 'cze', 'Czech' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 103, 'dak', 'Dakota' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 104, 'dan', 'Danish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 105, 'dar', 'Dargwa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 106, 'day', 'Land Dayak languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 107, 'del', 'Delaware' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 108, 'den', 'Slave (Athapascan)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 109, 'dgr', 'Tlicho; Dogrib' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 110, 'din', 'Dinka' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 111, 'div', 'Divehi; Dhivehi; Maldivian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 112, 'doi', 'Dogri' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 113, 'dra', 'Dravidian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 114, 'dsb', 'Lower Sorbian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 115, 'dua', 'Duala' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 116, 'dum', 'Dutch, Middle (ca.1050-1350)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 117, 'dut', 'Dutch; Flemish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 118, 'dyu', 'Dyula' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 119, 'dzo', 'Dzongkha' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 120, 'efi', 'Efik' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 121, 'egy', 'Egyptian (Ancient)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 122, 'eka', 'Ekajuk' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 123, 'elx', 'Elamite' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 124, 'eng', 'English' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 125, 'enm', 'English, Middle (1100-1500)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 126, 'epo', 'Esperanto' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 127, 'est', 'Estonian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 128, 'ewe', 'Ewe' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 129, 'ewo', 'Ewondo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 130, 'fan', 'Fang' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 131, 'fao', 'Faroese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 132, 'fat', 'Fanti' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 133, 'fij', 'Fijian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 134, 'fil', 'Filipino; Pilipino' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 135, 'fin', 'Finnish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 136, 'fiu', 'Finno-Ugrian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 137, 'fon', 'Fon' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 138, 'fre', 'French' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 139, 'frm', 'French, Middle (ca.1400-1600)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 140, 'fro', 'French, Old (842-ca.1400)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 141, 'frr', 'Northern Frisian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 142, 'frs', 'Eastern Frisian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 143, 'fry', 'Western Frisian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 144, 'ful', 'Fulah' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 145, 'fur', 'Friulian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 146, 'gaa', 'Ga' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 147, 'gay', 'Gayo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 148, 'gba', 'Gbaya' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 149, 'gem', 'Germanic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 150, 'geo', 'Georgian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 151, 'ger', 'German' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 152, 'gez', 'Geez' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 153, 'gil', 'Gilbertese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 154, 'gla', 'Gaelic; Scottish Gaelic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 155, 'gle', 'Irish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 156, 'glg', 'Galician' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 157, 'glv', 'Manx' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 158, 'gmh', 'German, Middle High (ca.1050-1500)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 159, 'goh', 'German, Old High (ca.750-1050)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 160, 'gon', 'Gondi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 161, 'gor', 'Gorontalo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 162, 'got', 'Gothic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 163, 'grb', 'Grebo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 164, 'grc', 'Greek, Ancient (to 1453)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 165, 'gre', 'Greek, Modern (1453-)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 166, 'grn', 'Guarani' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 167, 'gsw', 'Swiss German; Alemannic; Alsatian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 168, 'guj', 'Gujarati' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 169, 'gwi', 'Gwich''in' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 170, 'hai', 'Haida' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 171, 'hat', 'Haitian; Haitian Creole' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 172, 'hau', 'Hausa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 173, 'haw', 'Hawaiian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 174, 'heb', 'Hebrew' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 175, 'her', 'Herero' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 176, 'hil', 'Hiligaynon' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 177, 'him', 'Himachali languages; Western Pahari languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 178, 'hin', 'Hindi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 179, 'hit', 'Hittite' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 180, 'hmn', 'Hmong; Mong' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 181, 'hmo', 'Hiri Motu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 182, 'hrv', 'Croatian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 183, 'hsb', 'Upper Sorbian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 184, 'hun', 'Hungarian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 185, 'hup', 'Hupa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 186, 'iba', 'Iban' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 187, 'ibo', 'Igbo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 188, 'ice', 'Icelandic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 189, 'ido', 'Ido' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 190, 'iii', 'Sichuan Yi; Nuosu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 191, 'ijo', 'Ijo languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 192, 'iku', 'Inuktitut' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 193, 'ile', 'Interlingue; Occidental' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 194, 'ilo', 'Iloko' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 195, 'ina', 'Interlingua (International Auxiliary Language Association)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 196, 'inc', 'Indic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 197, 'ind', 'Indonesian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 198, 'ine', 'Indo-European languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 199, 'inh', 'Ingush' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 200, 'ipk', 'Inupiaq' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 201, 'ira', 'Iranian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 202, 'iro', 'Iroquoian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 203, 'ita', 'Italian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 204, 'jav', 'Javanese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 205, 'jbo', 'Lojban' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 206, 'jpn', 'Japanese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 207, 'jpr', 'Judeo-Persian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 208, 'jrb', 'Judeo-Arabic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 209, 'kaa', 'Kara-Kalpak' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 210, 'kab', 'Kabyle' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 211, 'kac', 'Kachin; Jingpho' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 212, 'kal', 'Kalaallisut; Greenlandic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 213, 'kam', 'Kamba' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 214, 'kan', 'Kannada' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 215, 'kar', 'Karen languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 216, 'kas', 'Kashmiri' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 217, 'kau', 'Kanuri' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 218, 'kaw', 'Kawi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 219, 'kaz', 'Kazakh' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 220, 'kbd', 'Kabardian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 221, 'kha', 'Khasi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 222, 'khi', 'Khoisan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 223, 'khm', 'Central Khmer' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 224, 'kho', 'Khotanese; Sakan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 225, 'kik', 'Kikuyu; Gikuyu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 226, 'kin', 'Kinyarwanda' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 227, 'kir', 'Kirghiz; Kyrgyz' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 228, 'kmb', 'Kimbundu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 229, 'kok', 'Konkani' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 230, 'kom', 'Komi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 231, 'kon', 'Kongo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 232, 'kor', 'Korean' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 233, 'kos', 'Kosraean' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 234, 'kpe', 'Kpelle' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 235, 'krc', 'Karachay-Balkar' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 236, 'krl', 'Karelian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 237, 'kro', 'Kru languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 238, 'kru', 'Kurukh' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 239, 'kua', 'Kuanyama; Kwanyama' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 240, 'kum', 'Kumyk' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 241, 'kur', 'Kurdish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 242, 'kut', 'Kutenai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 243, 'lad', 'Ladino' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 244, 'lah', 'Lahnda' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 245, 'lam', 'Lamba' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 246, 'lao', 'Lao' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 247, 'lat', 'Latin' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 248, 'lav', 'Latvian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 249, 'lez', 'Lezghian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 250, 'lim', 'Limburgan; Limburger; Limburgish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 251, 'lin', 'Lingala' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 252, 'lit', 'Lithuanian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 253, 'lol', 'Mongo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 254, 'loz', 'Lozi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 255, 'ltz', 'Luxembourgish; Letzeburgesch' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 256, 'lua', 'Luba-Lulua' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 257, 'lub', 'Luba-Katanga' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 258, 'lug', 'Ganda' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 259, 'lui', 'Luiseno' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 260, 'lun', 'Lunda' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 261, 'luo', 'Luo (Kenya and Tanzania)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 262, 'lus', 'Lushai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 263, 'mac', 'Macedonian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 264, 'mad', 'Madurese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 265, 'mag', 'Magahi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 266, 'mah', 'Marshallese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 267, 'mai', 'Maithili' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 268, 'mak', 'Makasar' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 269, 'mal', 'Malayalam' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 270, 'man', 'Mandingo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 271, 'mao', 'Maori' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 272, 'map', 'Austronesian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 273, 'mar', 'Marathi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 274, 'mas', 'Masai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 275, 'may', 'Malay' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 276, 'mdf', 'Moksha' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 277, 'mdr', 'Mandar' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 278, 'men', 'Mende' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 279, 'mga', 'Irish, Middle (900-1200)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 280, 'mic', 'Mi''kmaq; Micmac' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 281, 'min', 'Minangkabau' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 282, 'mis', 'Uncoded languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 283, 'mkh', 'Mon-Khmer languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 284, 'mlg', 'Malagasy' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 285, 'mlt', 'Maltese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 286, 'mnc', 'Manchu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 287, 'mni', 'Manipuri' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 288, 'mno', 'Manobo languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 289, 'moh', 'Mohawk' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 290, 'mon', 'Mongolian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 291, 'mos', 'Mossi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 292, 'mul', 'Multiple languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 293, 'mun', 'Munda languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 294, 'mus', 'Creek' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 295, 'mwl', 'Mirandese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 296, 'mwr', 'Marwari' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 297, 'myn', 'Mayan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 298, 'myv', 'Erzya' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 299, 'nah', 'Nahuatl languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 300, 'nai', 'North American Indian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 301, 'nap', 'Neapolitan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 302, 'nau', 'Nauru' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 303, 'nav', 'Navajo; Navaho' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 304, 'nbl', 'Ndebele, South; South Ndebele' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 305, 'nde', 'Ndebele, North; North Ndebele' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 306, 'ndo', 'Ndonga' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 307, 'nds', 'Low German; Low Saxon; German, Low; Saxon, Low' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 308, 'nep', 'Nepali' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 309, 'new', 'Nepal Bhasa; Newari' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 310, 'nia', 'Nias' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 311, 'nic', 'Niger-Kordofanian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 312, 'niu', 'Niuean' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 313, 'nno', 'Norwegian Nynorsk; Nynorsk, Norwegian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 314, 'nob', 'Bokmål, Norwegian; Norwegian Bokmål' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 315, 'nog', 'Nogai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 316, 'non', 'Norse, Old' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 317, 'nor', 'Norwegian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 318, 'nqo', 'N''Ko' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 319, 'nso', 'Pedi; Sepedi; Northern Sotho' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 320, 'nub', 'Nubian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 321, 'nwc', 'Classical Newari; Old Newari; Classical Nepal Bhasa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 322, 'nya', 'Chichewa; Chewa; Nyanja' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 323, 'nym', 'Nyamwezi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 324, 'nyn', 'Nyankole' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 325, 'nyo', 'Nyoro' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 326, 'nzi', 'Nzima' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 327, 'oci', 'Occitan (post 1500)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 328, 'oji', 'Ojibwa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 329, 'ori', 'Oriya' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 330, 'orm', 'Oromo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 331, 'osa', 'Osage' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 332, 'oss', 'Ossetian; Ossetic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 333, 'ota', 'Turkish, Ottoman (1500-1928)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 334, 'oto', 'Otomian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 335, 'paa', 'Papuan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 336, 'pag', 'Pangasinan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 337, 'pal', 'Pahlavi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 338, 'pam', 'Pampanga; Kapampangan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 339, 'pan', 'Panjabi; Punjabi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 340, 'pap', 'Papiamento' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 341, 'pau', 'Palauan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 342, 'peo', 'Persian, Old (ca.600-400 B.C.)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 343, 'per', 'Persian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 344, 'phi', 'Philippine languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 345, 'phn', 'Phoenician' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 346, 'pli', 'Pali' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 347, 'pol', 'Polish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 348, 'pon', 'Pohnpeian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 349, 'por', 'Portuguese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 350, 'pra', 'Prakrit languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 351, 'pro', 'Provençal, Old (to 1500); Occitan, Old (to 1500)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 352, 'pus', 'Pushto; Pashto' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 353, 'qaa-qtz', 'Reserved for local use' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 354, 'que', 'Quechua' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 355, 'raj', 'Rajasthani' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 356, 'rap', 'Rapanui' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 357, 'rar', 'Rarotongan; Cook Islands Maori' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 358, 'roa', 'Romance languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 359, 'roh', 'Romansh' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 360, 'rom', 'Romany' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 361, 'rum', 'Romanian; Moldavian; Moldovan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 362, 'run', 'Rundi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 363, 'rup', 'Aromanian; Arumanian; Macedo-Romanian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 364, 'rus', 'Russian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 365, 'sad', 'Sandawe' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 366, 'sag', 'Sango' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 367, 'sah', 'Yakut' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 368, 'sai', 'South American Indian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 369, 'sal', 'Salishan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 370, 'sam', 'Samaritan Aramaic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 371, 'san', 'Sanskrit' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 372, 'sas', 'Sasak' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 373, 'sat', 'Santali' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 374, 'scn', 'Sicilian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 375, 'sco', 'Scots' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 376, 'sel', 'Selkup' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 377, 'sem', 'Semitic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 378, 'sga', 'Irish, Old (to 900)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 379, 'sgn', 'Sign Languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 380, 'shn', 'Shan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 381, 'sid', 'Sidamo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 382, 'sin', 'Sinhala; Sinhalese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 383, 'sio', 'Siouan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 384, 'sit', 'Sino-Tibetan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 385, 'sla', 'Slavic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 386, 'slo', 'Slovak' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 387, 'slv', 'Slovenian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 388, 'sma', 'Southern Sami' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 389, 'sme', 'Northern Sami' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 390, 'smi', 'Sami languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 391, 'smj', 'Lule Sami' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 392, 'smn', 'Inari Sami' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 393, 'smo', 'Samoan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 394, 'sms', 'Skolt Sami' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 395, 'sna', 'Shona' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 396, 'snd', 'Sindhi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 397, 'snk', 'Soninke' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 398, 'sog', 'Sogdian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 399, 'som', 'Somali' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 400, 'son', 'Songhai languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 401, 'sot', 'Sotho, Southern' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 402, 'spa', 'Spanish; Castilian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 403, 'srd', 'Sardinian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 404, 'srn', 'Sranan Tongo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 405, 'srp', 'Serbian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 406, 'srr', 'Serer' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 407, 'ssa', 'Nilo-Saharan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 408, 'ssw', 'Swati' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 409, 'suk', 'Sukuma' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 410, 'sun', 'Sundanese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 411, 'sus', 'Susu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 412, 'sux', 'Sumerian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 413, 'swa', 'Swahili' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 414, 'swe', 'Swedish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 415, 'syc', 'Classical Syriac' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 416, 'syr', 'Syriac' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 417, 'tah', 'Tahitian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 418, 'tai', 'Tai languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 419, 'tam', 'Tamil' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 420, 'tat', 'Tatar' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 421, 'tel', 'Telugu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 422, 'tem', 'Timne' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 423, 'ter', 'Tereno' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 424, 'tet', 'Tetum' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 425, 'tgk', 'Tajik' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 426, 'tgl', 'Tagalog' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 427, 'tha', 'Thai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 428, 'tib', 'Tibetan' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 429, 'tig', 'Tigre' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 430, 'tir', 'Tigrinya' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 431, 'tiv', 'Tiv' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 432, 'tkl', 'Tokelau' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 433, 'tlh', 'Klingon; tlhIngan-Hol' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 434, 'tli', 'Tlingit' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 435, 'tmh', 'Tamashek' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 436, 'tog', 'Tonga (Nyasa)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 437, 'ton', 'Tonga (Tonga Islands)' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 438, 'tpi', 'Tok Pisin' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 439, 'tsi', 'Tsimshian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 440, 'tsn', 'Tswana' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 441, 'tso', 'Tsonga' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 442, 'tuk', 'Turkmen' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 443, 'tum', 'Tumbuka' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 444, 'tup', 'Tupi languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 445, 'tur', 'Turkish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 446, 'tut', 'Altaic languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 447, 'tvl', 'Tuvalu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 448, 'twi', 'Twi' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 449, 'tyv', 'Tuvinian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 450, 'udm', 'Udmurt' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 451, 'uga', 'Ugaritic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 452, 'uig', 'Uighur; Uyghur' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 453, 'ukr', 'Ukrainian' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 454, 'umb', 'Umbundu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 455, 'und', 'Undetermined' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 456, 'urd', 'Urdu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 457, 'uzb', 'Uzbek' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 458, 'vai', 'Vai' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 459, 'ven', 'Venda' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 460, 'vie', 'Vietnamese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 461, 'vol', 'Volapük' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 462, 'vot', 'Votic' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 463, 'wak', 'Wakashan languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 464, 'wal', 'Wolaitta; Wolaytta' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 465, 'war', 'Waray' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 466, 'was', 'Washo' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 467, 'wel', 'Welsh' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 468, 'wen', 'Sorbian languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 469, 'wln', 'Walloon' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 470, 'wol', 'Wolof' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 471, 'xal', 'Kalmyk; Oirat' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 472, 'xho', 'Xhosa' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 473, 'yao', 'Yao' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 474, 'yap', 'Yapese' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 475, 'yid', 'Yiddish' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 476, 'yor', 'Yoruba' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 477, 'ypk', 'Yupik languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 478, 'zap', 'Zapotec' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 479, 'zbl', 'Blissymbols; Blissymbolics; Bliss' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 480, 'zen', 'Zenaga' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 481, 'zgh', 'Standard Moroccan Tamazight' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 482, 'zha', 'Zhuang; Chuang' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 483, 'znd', 'Zande languages' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 484, 'zul', 'Zulu' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 485, 'zun', 'Zuni' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 486, 'zxx', 'No linguistic content; Not applicable' ) ;
INSERT INTO languages_names ( id, iso_code, name ) VALUES ( 487, 'zza', 'Zaza; Dimili; Dimli; Kirdki; Kirmanjki; Zazaki' ) ;

-- Table: languages_roles
DROP TABLE IF EXISTS languages_roles;

CREATE TABLE IF NOT EXISTS languages_roles (
    id   INTEGER,
    name TEXT    NOT NULL
                 UNIQUE,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);

INSERT INTO languages_roles ( id, name ) VALUES ( 1, 'source' )  ;

INSERT INTO languages_roles ( id, name ) VALUES ( 2, 'original' )  ;

INSERT INTO languages_roles ( id, name ) VALUES ( 3, 'current' )  ;


-- Table: laverdure
DROP TABLE IF EXISTS laverdure;

CREATE TABLE IF NOT EXISTS laverdure (
    key   TEXT NOT NULL,
    value TEXT,
    PRIMARY KEY (
        key
    )
);

INSERT INTO laverdure (key, value ) VALUES ( 'author', 'laverdure' )  ;

INSERT INTO laverdure (key, value ) VALUES ( 'program', 'ritmo' )  ;

INSERT INTO laverdure (key, value ) VALUES ( 'program_release', '0.0.0' )  ;

INSERT INTO laverdure (key, value ) VALUES ( 'database_version', '0.0.0' )  ;


-- Table: people
DROP TABLE IF EXISTS people;

CREATE TABLE IF NOT EXISTS people (
    id          INTEGER NOT NULL
                        UNIQUE,
    name        TEXT    NOT NULL,
    nationality TEXT,
    birth_date  INTEGER,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: publishers
DROP TABLE IF EXISTS publishers;

CREATE TABLE IF NOT EXISTS publishers (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: roles
DROP TABLE IF EXISTS roles;

CREATE TABLE IF NOT EXISTS roles (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);

INSERT INTO roles (id, name ) VALUES ( 1, 'Author' )  ;

INSERT INTO roles (id, name ) VALUES ( 2, 'Translator' )  ;

INSERT INTO roles (id, name ) VALUES ( 3, 'Curator' )  ;

INSERT INTO roles (
                      id,
                      name
                  )
                  VALUES (
                      4,
                      'Cover designer'
                  );

INSERT INTO roles (id, name ) VALUES ( 5, 'Commentator' )  ;

INSERT INTO roles (id, name ) VALUES ( 6, 'Interviewer' )  ;

INSERT INTO roles (id, name ) VALUES ( 7, 'Reporter' )  ;

INSERT INTO roles (id, name ) VALUES ( 8, 'Photographer' )  ;

INSERT INTO roles (id, name ) VALUES ( 9, 'Comic book artist' );

INSERT INTO roles ( id, name ) VALUES ( 10, 'fancazzista' )  ;


-- Table: running_languages
DROP TABLE IF EXISTS running_languages;

CREATE TABLE IF NOT EXISTS running_languages (
    id       INTEGER,
    iso_code TEXT,
    role     INTEGER,
    FOREIGN KEY (
        role
    )
    REFERENCES languages_roles (id),
    FOREIGN KEY (
        iso_code
    )
    REFERENCES languages_names (iso_code),
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: series
DROP TABLE IF EXISTS series;

CREATE TABLE IF NOT EXISTS series (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: tags
DROP TABLE IF EXISTS tags;

CREATE TABLE IF NOT EXISTS tags (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);


-- Table: types
DROP TABLE IF EXISTS types;

CREATE TABLE IF NOT EXISTS types (
    id   INTEGER NOT NULL,
    name TEXT    NOT NULL,
    PRIMARY KEY (
        id AUTOINCREMENT
    )
);

INSERT INTO types ( id, name ) VALUES ( 1, 'Novel' )  ;

INSERT INTO types ( id,  name  )  VALUES (  2,  'Short novel'  ); 
INSERT INTO types ( id, name ) VALUES ( 3, 'Short story' );
INSERT INTO types ( id, name ) VALUES ( 4, 'Essay' )  ;

INSERT INTO types ( id, name ) VALUES ( 5, 'Treatise' )  ;

INSERT INTO types ( id, name ) VALUES ( 6, 'Dissertation' )  ;

INSERT INTO types ( id, name ) VALUES ( 7, 'Biography' )  ;

INSERT INTO types ( id, name ) VALUES ( 8, 'Autobiography' )  ;

INSERT INTO types ( id, name ) VALUES ( 9, 'Memoir' )  ;

INSERT INTO types ( id, name ) VALUES ( 10, 'Interview' )  ;

INSERT INTO types ( id, name ) VALUES ( 11, 'Play' )  ;

INSERT INTO types ( id, name ) VALUES ( 12, 'One-act play' );

INSERT INTO types ( id, name ) VALUES ( 13, 'Poetry' )  ;

INSERT INTO types ( id, name ) VALUES ( 14, 'Opera' )  ;


-- Index: idx_books_contents_junction
DROP INDEX IF EXISTS idx_books_contents_junction;

CREATE INDEX IF NOT EXISTS idx_books_contents_junction ON books_contents (
    "book_id",
    "content_id"
);


-- Index: idx_books_core_search
DROP INDEX IF EXISTS idx_books_core_search;

CREATE INDEX IF NOT EXISTS idx_books_core_search ON books (
    "name",
    "series_id",
    "publication_date"
);


-- Index: idx_books_metadata
DROP INDEX IF EXISTS idx_books_metadata;

CREATE INDEX IF NOT EXISTS idx_books_metadata ON books (
    "publisher_id",
    "format_id",
    "series_id"
);


-- Index: idx_books_people_lookup
DROP INDEX IF EXISTS idx_books_people_lookup;

CREATE INDEX IF NOT EXISTS idx_books_people_lookup ON books_people_roles (
    "book_id",
    "person_id"
);


-- Index: idx_books_series_index
DROP INDEX IF EXISTS idx_books_series_index;

CREATE INDEX IF NOT EXISTS idx_books_series_index ON books (
    "series_id",
    "series_index"
);


-- Index: idx_books_tags_lookup
DROP INDEX IF EXISTS idx_books_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_books_tags_lookup ON books_tags (
    "book_id",
    "tag_id"
);


-- Index: idx_books_temporal
DROP INDEX IF EXISTS idx_books_temporal;

CREATE INDEX IF NOT EXISTS idx_books_temporal ON books (
    "publication_date",
    "acquisition_date",
    "last_modified_date"
);


-- Index: idx_contents_core_search
DROP INDEX IF EXISTS idx_contents_core_search;

CREATE INDEX IF NOT EXISTS idx_contents_core_search ON contents (
    "name",
    "type_id",
    "publication_date"
);


-- Index: idx_contents_metadata
DROP INDEX IF EXISTS idx_contents_metadata;

CREATE INDEX IF NOT EXISTS idx_contents_metadata ON contents (
    "type_id"
);


-- Index: idx_contents_people_lookup
DROP INDEX IF EXISTS idx_contents_people_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_people_lookup ON contents_people_roles (
    "content_id",
    "person_id"
);


-- Index: idx_contents_tags_lookup
DROP INDEX IF EXISTS idx_contents_tags_lookup;

CREATE INDEX IF NOT EXISTS idx_contents_tags_lookup ON contents_tags (
    "content_id",
    "tag_id"
);


-- Index: idx_contents_temporal
DROP INDEX IF EXISTS idx_contents_temporal;

CREATE INDEX IF NOT EXISTS idx_contents_temporal ON contents (
    "publication_date"
);


-- Index: idx_contents_type_date
DROP INDEX IF EXISTS idx_contents_type_date;

CREATE INDEX IF NOT EXISTS idx_contents_type_date ON contents (
    "type_id",
    "publication_date"
);


-- Index: idx_people_search
DROP INDEX IF EXISTS idx_people_search;

CREATE INDEX IF NOT EXISTS idx_people_search ON people (
    "name",
    "id"
);


-- Index: idx_publishers_search
DROP INDEX IF EXISTS idx_publishers_search;

CREATE INDEX IF NOT EXISTS idx_publishers_search ON publishers (
    "name",
    "id"
);


-- Index: idx_roles_search
DROP INDEX IF EXISTS idx_roles_search;

CREATE INDEX IF NOT EXISTS idx_roles_search ON roles (
    "name"
);


-- Index: idx_series_search
DROP INDEX IF EXISTS idx_series_search;

CREATE INDEX IF NOT EXISTS idx_series_search ON series (
    "name"
);


-- Index: idx_tags_search
DROP INDEX IF EXISTS idx_tags_search;

CREATE INDEX IF NOT EXISTS idx_tags_search ON tags (
    "name"
);


-- Index: idx_v_contents_details
DROP INDEX IF EXISTS idx_v_contents_details;

CREATE INDEX IF NOT EXISTS idx_v_contents_details ON contents (
    "id",
    "type_id",
    "publication_date"
);


-- View: BooksPeopleRolesDetails
DROP VIEW IF EXISTS BooksPeopleRolesDetails;
CREATE VIEW IF NOT EXISTS BooksPeopleRolesDetails AS
    SELECT bpr.book_id,
           b.name AS book_name,
           p.id AS person_id,
           p.name AS person_name,
           r.name AS role_name
      FROM books_people_roles bpr
           JOIN
           books b ON bpr.book_id = b.id
           JOIN
           people p ON bpr.person_id = p.id
           JOIN
           roles r ON bpr.role_id = r.id;


-- View: BooksTagsDetails
DROP VIEW IF EXISTS BooksTagsDetails;
CREATE VIEW IF NOT EXISTS BooksTagsDetails AS
    SELECT bt.book_id,
           b.name AS book_name,
           t.name AS tag_name
      FROM books_tags bt
           JOIN
           books b ON bt.book_id = b.id
           JOIN
           tags t ON bt.tag_id = t.id;


-- View: BooksWithDetails
DROP VIEW IF EXISTS BooksWithDetails;
CREATE VIEW IF NOT EXISTS BooksWithDetails AS
    SELECT b.id,
           b.name AS book_name,
           p.name AS publisher_name,
           f.name AS format_name,
           s.name AS series_name,
           b.series_index,
           b.publication_date,
           b.acquisition_date,
           b.last_modified_date,
           b.original_title,
           b.notes,
           b.has_cover,
           b.has_paper,
           b.file_link,
           b.pre_accepted
      FROM books b
           LEFT JOIN
           publishers p ON b.publisher_id = p.id
           LEFT JOIN
           formats f ON b.format_id = f.id
           LEFT JOIN
           series s ON b.series_id = s.id;


-- View: ContentsLanguagesDetails
DROP VIEW IF EXISTS ContentsLanguagesDetails;
CREATE VIEW IF NOT EXISTS ContentsLanguagesDetails AS
    SELECT cl.contents_id,
           c.name AS content_name,
           ln.name AS language_name,
           lr.name AS language_role
      FROM contents_languages cl
           JOIN
           contents c ON cl.contents_id = c.id
           JOIN
           running_languages rl ON cl.languages_id = rl.id
           JOIN
           languages_names ln ON rl.iso_code = ln.iso_code
           JOIN
           languages_roles lr ON rl.role = lr.id;


-- View: ContentsPeopleRolesDetails
DROP VIEW IF EXISTS ContentsPeopleRolesDetails;
CREATE VIEW IF NOT EXISTS ContentsPeopleRolesDetails AS
    SELECT cpr.content_id,
           c.name AS content_name,
           p.id AS person_id,
           p.name AS person_name,
           r.name AS role_name
      FROM contents_people_roles cpr
           JOIN
           contents c ON cpr.content_id = c.id
           JOIN
           people p ON cpr.person_id = p.id
           JOIN
           roles r ON cpr.role_id = r.id;


-- View: ContentsTagsDetails
DROP VIEW IF EXISTS ContentsTagsDetails;
CREATE VIEW IF NOT EXISTS ContentsTagsDetails AS
    SELECT ct.content_id,
           c.name AS content_name,
           t.name AS tag_name
      FROM contents_tags ct
           JOIN
           contents c ON ct.content_id = c.id
           JOIN
           tags t ON ct.tag_id = t.id;


-- View: ContentsWithDetails
DROP VIEW IF EXISTS ContentsWithDetails;
CREATE VIEW IF NOT EXISTS ContentsWithDetails AS
    SELECT c.id,
           c.name AS content_name,
           c.original_title,
           c.publication_date,
           c.notes,
           t.name AS type_name,
           c.pre_accepted
      FROM contents c
           LEFT JOIN
           types t ON c.type_id = t.id;


PRAGMA foreign_keys = on;
