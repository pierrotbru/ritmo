CREATE TABLE IF NOT EXISTS "aliases" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"person_id"	INTEGER NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT),
	FOREIGN KEY("person_id") REFERENCES "people"("id")
);
CREATE TABLE IF NOT EXISTS "books_contents" (
	"book_id"	INTEGER NOT NULL,
	"content_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","content_id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id")
);
CREATE TABLE IF NOT EXISTS "books_people_roles" (
	"book_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","person_id","role_id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id"),
	FOREIGN KEY("role_id") REFERENCES "roles"("id"),
	FOREIGN KEY("person_id") REFERENCES "people"("id")
);
CREATE TABLE IF NOT EXISTS "books_tags" (
	"book_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	PRIMARY KEY("book_id","tag_id"),
	FOREIGN KEY("tag_id") REFERENCES "tags"("id"),
	FOREIGN KEY("book_id") REFERENCES "books"("id")
);
CREATE TABLE IF NOT EXISTS "contents" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"original_title"	TEXT,
	"publication_date"	BIGINT,
	"notes"	TEXT,
	"type_id"	INTEGER,
	"pre_accepted"	INTEGER DEFAULT (1),
	FOREIGN KEY("type_id") REFERENCES "types"("id"),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "contents_people_roles" (
	"content_id"	INTEGER NOT NULL,
	"person_id"	INTEGER NOT NULL,
	"role_id"	INTEGER NOT NULL,
	FOREIGN KEY("role_id") REFERENCES "roles"("id"),
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("person_id") REFERENCES "people"("id"),
	PRIMARY KEY("content_id","person_id","role_id")
);
CREATE TABLE IF NOT EXISTS "contents_tags" (
	"content_id"	INTEGER NOT NULL,
	"tag_id"	INTEGER NOT NULL,
	FOREIGN KEY("content_id") REFERENCES "contents"("id"),
	FOREIGN KEY("tag_id") REFERENCES "tags"("id"),
	PRIMARY KEY("content_id","tag_id")
);
CREATE TABLE IF NOT EXISTS "formats" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "laverdure" (
	"key"	TEXT NOT NULL,
	"value"	TEXT,
	PRIMARY KEY("key")
);
CREATE TABLE IF NOT EXISTS "people" (
	"id"	INTEGER NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	"nationality"	TEXT,
	"birth_date"	INTEGER,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "publishers" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "roles" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "series" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "tags" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "types" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "languages_names" (
	"id"	INTEGER,
	"iso_code"	TEXT NOT NULL UNIQUE,
	"name"	TEXT NOT NULL,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "books" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"publisher_id"	INTEGER,
	"format_id"	INTEGER,
	"publication_date"	BIGINT,
	"acquisition_date"	BIGINT,
	"last_modified_date"	BIGINT,
	"series_id"	INTEGER,
	"series_index"	INTEGER,
	"original_title"	TEXT,
	"notes"	TEXT,
	"has_cover"	INTEGER,
	"has_paper"	INTEGER,
	"file_link"	TEXT UNIQUE,
	"pre_accepted"	INTEGER DEFAULT (1),
	FOREIGN KEY("publisher_id") REFERENCES "publishers"("id"),
	FOREIGN KEY("format_id") REFERENCES "formats"("id"),
	FOREIGN KEY("series_id") REFERENCES "series"("id"),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "languages_roles" (
	"id"	INTEGER,
	"name"	TEXT NOT NULL UNIQUE,
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "running_languages" (
	"id"	INTEGER,
	"iso_code"	TEXT,
	"role"	INTEGER,
	FOREIGN KEY("role") REFERENCES "languages_roles"("id"),
	FOREIGN KEY("iso_code") REFERENCES "languages_names"("iso_code"),
	PRIMARY KEY("id" AUTOINCREMENT)
);
CREATE TABLE IF NOT EXISTS "contents_languages" (
	"contents_id"	INTEGER,
	"languages_id"	INTEGER,
	FOREIGN KEY("contents_id") REFERENCES "contents"("id"),
	FOREIGN KEY("languages_id") REFERENCES "running_languages"("id"),
	PRIMARY KEY("contents_id","languages_id")
);
INSERT INTO "laverdure" VALUES ('author','laverdure');
INSERT INTO "laverdure" VALUES ('program','ritmo');
INSERT INTO "laverdure" VALUES ('program_release','0.0.0');
INSERT INTO "laverdure" VALUES ('database_version','0.0.0');
INSERT INTO "roles" VALUES (1,'Author');
INSERT INTO "roles" VALUES (2,'Translator');
INSERT INTO "roles" VALUES (3,'Curator');
INSERT INTO "roles" VALUES (4,'Cover designer');
INSERT INTO "roles" VALUES (5,'Commentator');
INSERT INTO "roles" VALUES (6,'Interviewer');
INSERT INTO "roles" VALUES (7,'Reporter');
INSERT INTO "roles" VALUES (8,'Photographer');
INSERT INTO "roles" VALUES (9,'Comic book artist');
INSERT INTO "types" VALUES (1,'Novel');
INSERT INTO "types" VALUES (2,'Short novel');
INSERT INTO "types" VALUES (3,'Short story');
INSERT INTO "types" VALUES (4,'Essay');
INSERT INTO "types" VALUES (5,'Treatise');
INSERT INTO "types" VALUES (6,'Dissertation');
INSERT INTO "types" VALUES (7,'Biography');
INSERT INTO "types" VALUES (8,'Autobiography');
INSERT INTO "types" VALUES (9,'Memoir');
INSERT INTO "types" VALUES (10,'Interview');
INSERT INTO "types" VALUES (11,'Play');
INSERT INTO "types" VALUES (12,'One-act play');
INSERT INTO "types" VALUES (13,'Poetry');
INSERT INTO "types" VALUES (14,'Opera');
INSERT INTO "languages_names" VALUES (1,'aar','Afar');
INSERT INTO "languages_names" VALUES (2,'abk','Abkhazian');
INSERT INTO "languages_names" VALUES (3,'ace','Achinese');
INSERT INTO "languages_names" VALUES (4,'ach','Acoli');
INSERT INTO "languages_names" VALUES (5,'ada','Adangme');
INSERT INTO "languages_names" VALUES (6,'ady','Adyghe; Adygei');
INSERT INTO "languages_names" VALUES (7,'afa','Afro-Asiatic languages');
INSERT INTO "languages_names" VALUES (8,'afh','Afrihili');
INSERT INTO "languages_names" VALUES (9,'afr','Afrikaans');
INSERT INTO "languages_names" VALUES (10,'ain','Ainu');
INSERT INTO "languages_names" VALUES (11,'aka','Akan');
INSERT INTO "languages_names" VALUES (12,'akk','Akkadian');
INSERT INTO "languages_names" VALUES (13,'alb','Albanian');
INSERT INTO "languages_names" VALUES (14,'ale','Aleut');
INSERT INTO "languages_names" VALUES (15,'alg','Algonquian languages');
INSERT INTO "languages_names" VALUES (16,'alt','Southern Altai');
INSERT INTO "languages_names" VALUES (17,'amh','Amharic');
INSERT INTO "languages_names" VALUES (18,'ang','English, Old (ca.450-1100)');
INSERT INTO "languages_names" VALUES (19,'anp','Angika');
INSERT INTO "languages_names" VALUES (20,'apa','Apache languages');
INSERT INTO "languages_names" VALUES (21,'ara','Arabic');
INSERT INTO "languages_names" VALUES (22,'arc','Official Aramaic (700-300 BCE); Imperial Aramaic (700-300 BCE)');
INSERT INTO "languages_names" VALUES (23,'arg','Aragonese');
INSERT INTO "languages_names" VALUES (24,'arm','Armenian');
INSERT INTO "languages_names" VALUES (25,'arn','Mapudungun; Mapuche');
INSERT INTO "languages_names" VALUES (26,'arp','Arapaho');
INSERT INTO "languages_names" VALUES (27,'art','Artificial languages');
INSERT INTO "languages_names" VALUES (28,'arw','Arawak');
INSERT INTO "languages_names" VALUES (29,'asm','Assamese');
INSERT INTO "languages_names" VALUES (30,'ast','Asturian; Bable; Leonese; Asturleonese');
INSERT INTO "languages_names" VALUES (31,'ath','Athapascan languages');
INSERT INTO "languages_names" VALUES (32,'aus','Australian languages');
INSERT INTO "languages_names" VALUES (33,'ava','Avaric');
INSERT INTO "languages_names" VALUES (34,'ave','Avestan');
INSERT INTO "languages_names" VALUES (35,'awa','Awadhi');
INSERT INTO "languages_names" VALUES (36,'aym','Aymara');
INSERT INTO "languages_names" VALUES (37,'aze','Azerbaijani');
INSERT INTO "languages_names" VALUES (38,'bad','Banda languages');
INSERT INTO "languages_names" VALUES (39,'bai','Bamileke languages');
INSERT INTO "languages_names" VALUES (40,'bak','Bashkir');
INSERT INTO "languages_names" VALUES (41,'bal','Baluchi');
INSERT INTO "languages_names" VALUES (42,'bam','Bambara');
INSERT INTO "languages_names" VALUES (43,'ban','Balinese');
INSERT INTO "languages_names" VALUES (44,'baq','Basque');
INSERT INTO "languages_names" VALUES (45,'bas','Basa');
INSERT INTO "languages_names" VALUES (46,'bat','Baltic languages');
INSERT INTO "languages_names" VALUES (47,'bej','Beja; Bedawiyet');
INSERT INTO "languages_names" VALUES (48,'bel','Belarusian');
INSERT INTO "languages_names" VALUES (49,'bem','Bemba');
INSERT INTO "languages_names" VALUES (50,'ben','Bengali');
INSERT INTO "languages_names" VALUES (51,'ber','Berber languages');
INSERT INTO "languages_names" VALUES (52,'bho','Bhojpuri');
INSERT INTO "languages_names" VALUES (53,'bih','Bihari languages');
INSERT INTO "languages_names" VALUES (54,'bik','Bikol');
INSERT INTO "languages_names" VALUES (55,'bin','Bini; Edo');
INSERT INTO "languages_names" VALUES (56,'bis','Bislama');
INSERT INTO "languages_names" VALUES (57,'bla','Siksika');
INSERT INTO "languages_names" VALUES (58,'bnt','Bantu languages');
INSERT INTO "languages_names" VALUES (59,'bos','Bosnian');
INSERT INTO "languages_names" VALUES (60,'bra','Braj');
INSERT INTO "languages_names" VALUES (61,'bre','Breton');
INSERT INTO "languages_names" VALUES (62,'btk','Batak languages');
INSERT INTO "languages_names" VALUES (63,'bua','Buriat');
INSERT INTO "languages_names" VALUES (64,'bug','Buginese');
INSERT INTO "languages_names" VALUES (65,'bul','Bulgarian');
INSERT INTO "languages_names" VALUES (66,'bur','Burmese');
INSERT INTO "languages_names" VALUES (67,'byn','Blin; Bilin');
INSERT INTO "languages_names" VALUES (68,'cad','Caddo');
INSERT INTO "languages_names" VALUES (69,'cai','Central American Indian languages');
INSERT INTO "languages_names" VALUES (70,'car','Galibi Carib');
INSERT INTO "languages_names" VALUES (71,'cat','Catalan; Valencian');
INSERT INTO "languages_names" VALUES (72,'cau','Caucasian languages');
INSERT INTO "languages_names" VALUES (73,'ceb','Cebuano');
INSERT INTO "languages_names" VALUES (74,'cel','Celtic languages');
INSERT INTO "languages_names" VALUES (75,'cha','Chamorro');
INSERT INTO "languages_names" VALUES (76,'chb','Chibcha');
INSERT INTO "languages_names" VALUES (77,'che','Chechen');
INSERT INTO "languages_names" VALUES (78,'chg','Chagatai');
INSERT INTO "languages_names" VALUES (79,'chi','Chinese');
INSERT INTO "languages_names" VALUES (80,'chk','Chuukese');
INSERT INTO "languages_names" VALUES (81,'chm','Mari');
INSERT INTO "languages_names" VALUES (82,'chn','Chinook jargon');
INSERT INTO "languages_names" VALUES (83,'cho','Choctaw');
INSERT INTO "languages_names" VALUES (84,'chp','Chipewyan; Dene Suline');
INSERT INTO "languages_names" VALUES (85,'chr','Cherokee');
INSERT INTO "languages_names" VALUES (86,'chu','Church Slavic; Old Slavonic; Church Slavonic; Old Bulgarian; Old Church Slavonic');
INSERT INTO "languages_names" VALUES (87,'chv','Chuvash');
INSERT INTO "languages_names" VALUES (88,'chy','Cheyenne');
INSERT INTO "languages_names" VALUES (89,'cmc','Chamic languages');
INSERT INTO "languages_names" VALUES (90,'cnr','Montenegrin');
INSERT INTO "languages_names" VALUES (91,'cop','Coptic');
INSERT INTO "languages_names" VALUES (92,'cor','Cornish');
INSERT INTO "languages_names" VALUES (93,'cos','Corsican');
INSERT INTO "languages_names" VALUES (94,'cpe','Creoles and pidgins, English based');
INSERT INTO "languages_names" VALUES (95,'cpf','Creoles and pidgins, French-based');
INSERT INTO "languages_names" VALUES (96,'cpp','Creoles and pidgins, Portuguese-based');
INSERT INTO "languages_names" VALUES (97,'cre','Cree');
INSERT INTO "languages_names" VALUES (98,'crh','Crimean Tatar; Crimean Turkish');
INSERT INTO "languages_names" VALUES (99,'crp','Creoles and pidgins');
INSERT INTO "languages_names" VALUES (100,'csb','Kashubian');
INSERT INTO "languages_names" VALUES (101,'cus','Cushitic languages');
INSERT INTO "languages_names" VALUES (102,'cze','Czech');
INSERT INTO "languages_names" VALUES (103,'dak','Dakota');
INSERT INTO "languages_names" VALUES (104,'dan','Danish');
INSERT INTO "languages_names" VALUES (105,'dar','Dargwa');
INSERT INTO "languages_names" VALUES (106,'day','Land Dayak languages');
INSERT INTO "languages_names" VALUES (107,'del','Delaware');
INSERT INTO "languages_names" VALUES (108,'den','Slave (Athapascan)');
INSERT INTO "languages_names" VALUES (109,'dgr','Tlicho; Dogrib');
INSERT INTO "languages_names" VALUES (110,'din','Dinka');
INSERT INTO "languages_names" VALUES (111,'div','Divehi; Dhivehi; Maldivian');
INSERT INTO "languages_names" VALUES (112,'doi','Dogri');
INSERT INTO "languages_names" VALUES (113,'dra','Dravidian languages');
INSERT INTO "languages_names" VALUES (114,'dsb','Lower Sorbian');
INSERT INTO "languages_names" VALUES (115,'dua','Duala');
INSERT INTO "languages_names" VALUES (116,'dum','Dutch, Middle (ca.1050-1350)');
INSERT INTO "languages_names" VALUES (117,'dut','Dutch; Flemish');
INSERT INTO "languages_names" VALUES (118,'dyu','Dyula');
INSERT INTO "languages_names" VALUES (119,'dzo','Dzongkha');
INSERT INTO "languages_names" VALUES (120,'efi','Efik');
INSERT INTO "languages_names" VALUES (121,'egy','Egyptian (Ancient)');
INSERT INTO "languages_names" VALUES (122,'eka','Ekajuk');
INSERT INTO "languages_names" VALUES (123,'elx','Elamite');
INSERT INTO "languages_names" VALUES (124,'eng','English');
INSERT INTO "languages_names" VALUES (125,'enm','English, Middle (1100-1500)');
INSERT INTO "languages_names" VALUES (126,'epo','Esperanto');
INSERT INTO "languages_names" VALUES (127,'est','Estonian');
INSERT INTO "languages_names" VALUES (128,'ewe','Ewe');
INSERT INTO "languages_names" VALUES (129,'ewo','Ewondo');
INSERT INTO "languages_names" VALUES (130,'fan','Fang');
INSERT INTO "languages_names" VALUES (131,'fao','Faroese');
INSERT INTO "languages_names" VALUES (132,'fat','Fanti');
INSERT INTO "languages_names" VALUES (133,'fij','Fijian');
INSERT INTO "languages_names" VALUES (134,'fil','Filipino; Pilipino');
INSERT INTO "languages_names" VALUES (135,'fin','Finnish');
INSERT INTO "languages_names" VALUES (136,'fiu','Finno-Ugrian languages');
INSERT INTO "languages_names" VALUES (137,'fon','Fon');
INSERT INTO "languages_names" VALUES (138,'fre','French');
INSERT INTO "languages_names" VALUES (139,'frm','French, Middle (ca.1400-1600)');
INSERT INTO "languages_names" VALUES (140,'fro','French, Old (842-ca.1400)');
INSERT INTO "languages_names" VALUES (141,'frr','Northern Frisian');
INSERT INTO "languages_names" VALUES (142,'frs','Eastern Frisian');
INSERT INTO "languages_names" VALUES (143,'fry','Western Frisian');
INSERT INTO "languages_names" VALUES (144,'ful','Fulah');
INSERT INTO "languages_names" VALUES (145,'fur','Friulian');
INSERT INTO "languages_names" VALUES (146,'gaa','Ga');
INSERT INTO "languages_names" VALUES (147,'gay','Gayo');
INSERT INTO "languages_names" VALUES (148,'gba','Gbaya');
INSERT INTO "languages_names" VALUES (149,'gem','Germanic languages');
INSERT INTO "languages_names" VALUES (150,'geo','Georgian');
INSERT INTO "languages_names" VALUES (151,'ger','German');
INSERT INTO "languages_names" VALUES (152,'gez','Geez');
INSERT INTO "languages_names" VALUES (153,'gil','Gilbertese');
INSERT INTO "languages_names" VALUES (154,'gla','Gaelic; Scottish Gaelic');
INSERT INTO "languages_names" VALUES (155,'gle','Irish');
INSERT INTO "languages_names" VALUES (156,'glg','Galician');
INSERT INTO "languages_names" VALUES (157,'glv','Manx');
INSERT INTO "languages_names" VALUES (158,'gmh','German, Middle High (ca.1050-1500)');
INSERT INTO "languages_names" VALUES (159,'goh','German, Old High (ca.750-1050)');
INSERT INTO "languages_names" VALUES (160,'gon','Gondi');
INSERT INTO "languages_names" VALUES (161,'gor','Gorontalo');
INSERT INTO "languages_names" VALUES (162,'got','Gothic');
INSERT INTO "languages_names" VALUES (163,'grb','Grebo');
INSERT INTO "languages_names" VALUES (164,'grc','Greek, Ancient (to 1453)');
INSERT INTO "languages_names" VALUES (165,'gre','Greek, Modern (1453-)');
INSERT INTO "languages_names" VALUES (166,'grn','Guarani');
INSERT INTO "languages_names" VALUES (167,'gsw','Swiss German; Alemannic; Alsatian');
INSERT INTO "languages_names" VALUES (168,'guj','Gujarati');
INSERT INTO "languages_names" VALUES (169,'gwi','Gwich''in');
INSERT INTO "languages_names" VALUES (170,'hai','Haida');
INSERT INTO "languages_names" VALUES (171,'hat','Haitian; Haitian Creole');
INSERT INTO "languages_names" VALUES (172,'hau','Hausa');
INSERT INTO "languages_names" VALUES (173,'haw','Hawaiian');
INSERT INTO "languages_names" VALUES (174,'heb','Hebrew');
INSERT INTO "languages_names" VALUES (175,'her','Herero');
INSERT INTO "languages_names" VALUES (176,'hil','Hiligaynon');
INSERT INTO "languages_names" VALUES (177,'him','Himachali languages; Western Pahari languages');
INSERT INTO "languages_names" VALUES (178,'hin','Hindi');
INSERT INTO "languages_names" VALUES (179,'hit','Hittite');
INSERT INTO "languages_names" VALUES (180,'hmn','Hmong; Mong');
INSERT INTO "languages_names" VALUES (181,'hmo','Hiri Motu');
INSERT INTO "languages_names" VALUES (182,'hrv','Croatian');
INSERT INTO "languages_names" VALUES (183,'hsb','Upper Sorbian');
INSERT INTO "languages_names" VALUES (184,'hun','Hungarian');
INSERT INTO "languages_names" VALUES (185,'hup','Hupa');
INSERT INTO "languages_names" VALUES (186,'iba','Iban');
INSERT INTO "languages_names" VALUES (187,'ibo','Igbo');
INSERT INTO "languages_names" VALUES (188,'ice','Icelandic');
INSERT INTO "languages_names" VALUES (189,'ido','Ido');
INSERT INTO "languages_names" VALUES (190,'iii','Sichuan Yi; Nuosu');
INSERT INTO "languages_names" VALUES (191,'ijo','Ijo languages');
INSERT INTO "languages_names" VALUES (192,'iku','Inuktitut');
INSERT INTO "languages_names" VALUES (193,'ile','Interlingue; Occidental');
INSERT INTO "languages_names" VALUES (194,'ilo','Iloko');
INSERT INTO "languages_names" VALUES (195,'ina','Interlingua (International Auxiliary Language Association)');
INSERT INTO "languages_names" VALUES (196,'inc','Indic languages');
INSERT INTO "languages_names" VALUES (197,'ind','Indonesian');
INSERT INTO "languages_names" VALUES (198,'ine','Indo-European languages');
INSERT INTO "languages_names" VALUES (199,'inh','Ingush');
INSERT INTO "languages_names" VALUES (200,'ipk','Inupiaq');
INSERT INTO "languages_names" VALUES (201,'ira','Iranian languages');
INSERT INTO "languages_names" VALUES (202,'iro','Iroquoian languages');
INSERT INTO "languages_names" VALUES (203,'ita','Italian');
INSERT INTO "languages_names" VALUES (204,'jav','Javanese');
INSERT INTO "languages_names" VALUES (205,'jbo','Lojban');
INSERT INTO "languages_names" VALUES (206,'jpn','Japanese');
INSERT INTO "languages_names" VALUES (207,'jpr','Judeo-Persian');
INSERT INTO "languages_names" VALUES (208,'jrb','Judeo-Arabic');
INSERT INTO "languages_names" VALUES (209,'kaa','Kara-Kalpak');
INSERT INTO "languages_names" VALUES (210,'kab','Kabyle');
INSERT INTO "languages_names" VALUES (211,'kac','Kachin; Jingpho');
INSERT INTO "languages_names" VALUES (212,'kal','Kalaallisut; Greenlandic');
INSERT INTO "languages_names" VALUES (213,'kam','Kamba');
INSERT INTO "languages_names" VALUES (214,'kan','Kannada');
INSERT INTO "languages_names" VALUES (215,'kar','Karen languages');
INSERT INTO "languages_names" VALUES (216,'kas','Kashmiri');
INSERT INTO "languages_names" VALUES (217,'kau','Kanuri');
INSERT INTO "languages_names" VALUES (218,'kaw','Kawi');
INSERT INTO "languages_names" VALUES (219,'kaz','Kazakh');
INSERT INTO "languages_names" VALUES (220,'kbd','Kabardian');
INSERT INTO "languages_names" VALUES (221,'kha','Khasi');
INSERT INTO "languages_names" VALUES (222,'khi','Khoisan languages');
INSERT INTO "languages_names" VALUES (223,'khm','Central Khmer');
INSERT INTO "languages_names" VALUES (224,'kho','Khotanese; Sakan');
INSERT INTO "languages_names" VALUES (225,'kik','Kikuyu; Gikuyu');
INSERT INTO "languages_names" VALUES (226,'kin','Kinyarwanda');
INSERT INTO "languages_names" VALUES (227,'kir','Kirghiz; Kyrgyz');
INSERT INTO "languages_names" VALUES (228,'kmb','Kimbundu');
INSERT INTO "languages_names" VALUES (229,'kok','Konkani');
INSERT INTO "languages_names" VALUES (230,'kom','Komi');
INSERT INTO "languages_names" VALUES (231,'kon','Kongo');
INSERT INTO "languages_names" VALUES (232,'kor','Korean');
INSERT INTO "languages_names" VALUES (233,'kos','Kosraean');
INSERT INTO "languages_names" VALUES (234,'kpe','Kpelle');
INSERT INTO "languages_names" VALUES (235,'krc','Karachay-Balkar');
INSERT INTO "languages_names" VALUES (236,'krl','Karelian');
INSERT INTO "languages_names" VALUES (237,'kro','Kru languages');
INSERT INTO "languages_names" VALUES (238,'kru','Kurukh');
INSERT INTO "languages_names" VALUES (239,'kua','Kuanyama; Kwanyama');
INSERT INTO "languages_names" VALUES (240,'kum','Kumyk');
INSERT INTO "languages_names" VALUES (241,'kur','Kurdish');
INSERT INTO "languages_names" VALUES (242,'kut','Kutenai');
INSERT INTO "languages_names" VALUES (243,'lad','Ladino');
INSERT INTO "languages_names" VALUES (244,'lah','Lahnda');
INSERT INTO "languages_names" VALUES (245,'lam','Lamba');
INSERT INTO "languages_names" VALUES (246,'lao','Lao');
INSERT INTO "languages_names" VALUES (247,'lat','Latin');
INSERT INTO "languages_names" VALUES (248,'lav','Latvian');
INSERT INTO "languages_names" VALUES (249,'lez','Lezghian');
INSERT INTO "languages_names" VALUES (250,'lim','Limburgan; Limburger; Limburgish');
INSERT INTO "languages_names" VALUES (251,'lin','Lingala');
INSERT INTO "languages_names" VALUES (252,'lit','Lithuanian');
INSERT INTO "languages_names" VALUES (253,'lol','Mongo');
INSERT INTO "languages_names" VALUES (254,'loz','Lozi');
INSERT INTO "languages_names" VALUES (255,'ltz','Luxembourgish; Letzeburgesch');
INSERT INTO "languages_names" VALUES (256,'lua','Luba-Lulua');
INSERT INTO "languages_names" VALUES (257,'lub','Luba-Katanga');
INSERT INTO "languages_names" VALUES (258,'lug','Ganda');
INSERT INTO "languages_names" VALUES (259,'lui','Luiseno');
INSERT INTO "languages_names" VALUES (260,'lun','Lunda');
INSERT INTO "languages_names" VALUES (261,'luo','Luo (Kenya and Tanzania)');
INSERT INTO "languages_names" VALUES (262,'lus','Lushai');
INSERT INTO "languages_names" VALUES (263,'mac','Macedonian');
INSERT INTO "languages_names" VALUES (264,'mad','Madurese');
INSERT INTO "languages_names" VALUES (265,'mag','Magahi');
INSERT INTO "languages_names" VALUES (266,'mah','Marshallese');
INSERT INTO "languages_names" VALUES (267,'mai','Maithili');
INSERT INTO "languages_names" VALUES (268,'mak','Makasar');
INSERT INTO "languages_names" VALUES (269,'mal','Malayalam');
INSERT INTO "languages_names" VALUES (270,'man','Mandingo');
INSERT INTO "languages_names" VALUES (271,'mao','Maori');
INSERT INTO "languages_names" VALUES (272,'map','Austronesian languages');
INSERT INTO "languages_names" VALUES (273,'mar','Marathi');
INSERT INTO "languages_names" VALUES (274,'mas','Masai');
INSERT INTO "languages_names" VALUES (275,'may','Malay');
INSERT INTO "languages_names" VALUES (276,'mdf','Moksha');
INSERT INTO "languages_names" VALUES (277,'mdr','Mandar');
INSERT INTO "languages_names" VALUES (278,'men','Mende');
INSERT INTO "languages_names" VALUES (279,'mga','Irish, Middle (900-1200)');
INSERT INTO "languages_names" VALUES (280,'mic','Mi''kmaq; Micmac');
INSERT INTO "languages_names" VALUES (281,'min','Minangkabau');
INSERT INTO "languages_names" VALUES (282,'mis','Uncoded languages');
INSERT INTO "languages_names" VALUES (283,'mkh','Mon-Khmer languages');
INSERT INTO "languages_names" VALUES (284,'mlg','Malagasy');
INSERT INTO "languages_names" VALUES (285,'mlt','Maltese');
INSERT INTO "languages_names" VALUES (286,'mnc','Manchu');
INSERT INTO "languages_names" VALUES (287,'mni','Manipuri');
INSERT INTO "languages_names" VALUES (288,'mno','Manobo languages');
INSERT INTO "languages_names" VALUES (289,'moh','Mohawk');
INSERT INTO "languages_names" VALUES (290,'mon','Mongolian');
INSERT INTO "languages_names" VALUES (291,'mos','Mossi');
INSERT INTO "languages_names" VALUES (292,'mul','Multiple languages');
INSERT INTO "languages_names" VALUES (293,'mun','Munda languages');
INSERT INTO "languages_names" VALUES (294,'mus','Creek');
INSERT INTO "languages_names" VALUES (295,'mwl','Mirandese');
INSERT INTO "languages_names" VALUES (296,'mwr','Marwari');
INSERT INTO "languages_names" VALUES (297,'myn','Mayan languages');
INSERT INTO "languages_names" VALUES (298,'myv','Erzya');
INSERT INTO "languages_names" VALUES (299,'nah','Nahuatl languages');
INSERT INTO "languages_names" VALUES (300,'nai','North American Indian languages');
INSERT INTO "languages_names" VALUES (301,'nap','Neapolitan');
INSERT INTO "languages_names" VALUES (302,'nau','Nauru');
INSERT INTO "languages_names" VALUES (303,'nav','Navajo; Navaho');
INSERT INTO "languages_names" VALUES (304,'nbl','Ndebele, South; South Ndebele');
INSERT INTO "languages_names" VALUES (305,'nde','Ndebele, North; North Ndebele');
INSERT INTO "languages_names" VALUES (306,'ndo','Ndonga');
INSERT INTO "languages_names" VALUES (307,'nds','Low German; Low Saxon; German, Low; Saxon, Low');
INSERT INTO "languages_names" VALUES (308,'nep','Nepali');
INSERT INTO "languages_names" VALUES (309,'new','Nepal Bhasa; Newari');
INSERT INTO "languages_names" VALUES (310,'nia','Nias');
INSERT INTO "languages_names" VALUES (311,'nic','Niger-Kordofanian languages');
INSERT INTO "languages_names" VALUES (312,'niu','Niuean');
INSERT INTO "languages_names" VALUES (313,'nno','Norwegian Nynorsk; Nynorsk, Norwegian');
INSERT INTO "languages_names" VALUES (314,'nob','Bokmål, Norwegian; Norwegian Bokmål');
INSERT INTO "languages_names" VALUES (315,'nog','Nogai');
INSERT INTO "languages_names" VALUES (316,'non','Norse, Old');
INSERT INTO "languages_names" VALUES (317,'nor','Norwegian');
INSERT INTO "languages_names" VALUES (318,'nqo','N''Ko');
INSERT INTO "languages_names" VALUES (319,'nso','Pedi; Sepedi; Northern Sotho');
INSERT INTO "languages_names" VALUES (320,'nub','Nubian languages');
INSERT INTO "languages_names" VALUES (321,'nwc','Classical Newari; Old Newari; Classical Nepal Bhasa');
INSERT INTO "languages_names" VALUES (322,'nya','Chichewa; Chewa; Nyanja');
INSERT INTO "languages_names" VALUES (323,'nym','Nyamwezi');
INSERT INTO "languages_names" VALUES (324,'nyn','Nyankole');
INSERT INTO "languages_names" VALUES (325,'nyo','Nyoro');
INSERT INTO "languages_names" VALUES (326,'nzi','Nzima');
INSERT INTO "languages_names" VALUES (327,'oci','Occitan (post 1500)');
INSERT INTO "languages_names" VALUES (328,'oji','Ojibwa');
INSERT INTO "languages_names" VALUES (329,'ori','Oriya');
INSERT INTO "languages_names" VALUES (330,'orm','Oromo');
INSERT INTO "languages_names" VALUES (331,'osa','Osage');
INSERT INTO "languages_names" VALUES (332,'oss','Ossetian; Ossetic');
INSERT INTO "languages_names" VALUES (333,'ota','Turkish, Ottoman (1500-1928)');
INSERT INTO "languages_names" VALUES (334,'oto','Otomian languages');
INSERT INTO "languages_names" VALUES (335,'paa','Papuan languages');
INSERT INTO "languages_names" VALUES (336,'pag','Pangasinan');
INSERT INTO "languages_names" VALUES (337,'pal','Pahlavi');
INSERT INTO "languages_names" VALUES (338,'pam','Pampanga; Kapampangan');
INSERT INTO "languages_names" VALUES (339,'pan','Panjabi; Punjabi');
INSERT INTO "languages_names" VALUES (340,'pap','Papiamento');
INSERT INTO "languages_names" VALUES (341,'pau','Palauan');
INSERT INTO "languages_names" VALUES (342,'peo','Persian, Old (ca.600-400 B.C.)');
INSERT INTO "languages_names" VALUES (343,'per','Persian');
INSERT INTO "languages_names" VALUES (344,'phi','Philippine languages');
INSERT INTO "languages_names" VALUES (345,'phn','Phoenician');
INSERT INTO "languages_names" VALUES (346,'pli','Pali');
INSERT INTO "languages_names" VALUES (347,'pol','Polish');
INSERT INTO "languages_names" VALUES (348,'pon','Pohnpeian');
INSERT INTO "languages_names" VALUES (349,'por','Portuguese');
INSERT INTO "languages_names" VALUES (350,'pra','Prakrit languages');
INSERT INTO "languages_names" VALUES (351,'pro','Provençal, Old (to 1500); Occitan, Old (to 1500)');
INSERT INTO "languages_names" VALUES (352,'pus','Pushto; Pashto');
INSERT INTO "languages_names" VALUES (353,'qaa-qtz','Reserved for local use');
INSERT INTO "languages_names" VALUES (354,'que','Quechua');
INSERT INTO "languages_names" VALUES (355,'raj','Rajasthani');
INSERT INTO "languages_names" VALUES (356,'rap','Rapanui');
INSERT INTO "languages_names" VALUES (357,'rar','Rarotongan; Cook Islands Maori');
INSERT INTO "languages_names" VALUES (358,'roa','Romance languages');
INSERT INTO "languages_names" VALUES (359,'roh','Romansh');
INSERT INTO "languages_names" VALUES (360,'rom','Romany');
INSERT INTO "languages_names" VALUES (361,'rum','Romanian; Moldavian; Moldovan');
INSERT INTO "languages_names" VALUES (362,'run','Rundi');
INSERT INTO "languages_names" VALUES (363,'rup','Aromanian; Arumanian; Macedo-Romanian');
INSERT INTO "languages_names" VALUES (364,'rus','Russian');
INSERT INTO "languages_names" VALUES (365,'sad','Sandawe');
INSERT INTO "languages_names" VALUES (366,'sag','Sango');
INSERT INTO "languages_names" VALUES (367,'sah','Yakut');
INSERT INTO "languages_names" VALUES (368,'sai','South American Indian languages');
INSERT INTO "languages_names" VALUES (369,'sal','Salishan languages');
INSERT INTO "languages_names" VALUES (370,'sam','Samaritan Aramaic');
INSERT INTO "languages_names" VALUES (371,'san','Sanskrit');
INSERT INTO "languages_names" VALUES (372,'sas','Sasak');
INSERT INTO "languages_names" VALUES (373,'sat','Santali');
INSERT INTO "languages_names" VALUES (374,'scn','Sicilian');
INSERT INTO "languages_names" VALUES (375,'sco','Scots');
INSERT INTO "languages_names" VALUES (376,'sel','Selkup');
INSERT INTO "languages_names" VALUES (377,'sem','Semitic languages');
INSERT INTO "languages_names" VALUES (378,'sga','Irish, Old (to 900)');
INSERT INTO "languages_names" VALUES (379,'sgn','Sign Languages');
INSERT INTO "languages_names" VALUES (380,'shn','Shan');
INSERT INTO "languages_names" VALUES (381,'sid','Sidamo');
INSERT INTO "languages_names" VALUES (382,'sin','Sinhala; Sinhalese');
INSERT INTO "languages_names" VALUES (383,'sio','Siouan languages');
INSERT INTO "languages_names" VALUES (384,'sit','Sino-Tibetan languages');
INSERT INTO "languages_names" VALUES (385,'sla','Slavic languages');
INSERT INTO "languages_names" VALUES (386,'slo','Slovak');
INSERT INTO "languages_names" VALUES (387,'slv','Slovenian');
INSERT INTO "languages_names" VALUES (388,'sma','Southern Sami');
INSERT INTO "languages_names" VALUES (389,'sme','Northern Sami');
INSERT INTO "languages_names" VALUES (390,'smi','Sami languages');
INSERT INTO "languages_names" VALUES (391,'smj','Lule Sami');
INSERT INTO "languages_names" VALUES (392,'smn','Inari Sami');
INSERT INTO "languages_names" VALUES (393,'smo','Samoan');
INSERT INTO "languages_names" VALUES (394,'sms','Skolt Sami');
INSERT INTO "languages_names" VALUES (395,'sna','Shona');
INSERT INTO "languages_names" VALUES (396,'snd','Sindhi');
INSERT INTO "languages_names" VALUES (397,'snk','Soninke');
INSERT INTO "languages_names" VALUES (398,'sog','Sogdian');
INSERT INTO "languages_names" VALUES (399,'som','Somali');
INSERT INTO "languages_names" VALUES (400,'son','Songhai languages');
INSERT INTO "languages_names" VALUES (401,'sot','Sotho, Southern');
INSERT INTO "languages_names" VALUES (402,'spa','Spanish; Castilian');
INSERT INTO "languages_names" VALUES (403,'srd','Sardinian');
INSERT INTO "languages_names" VALUES (404,'srn','Sranan Tongo');
INSERT INTO "languages_names" VALUES (405,'srp','Serbian');
INSERT INTO "languages_names" VALUES (406,'srr','Serer');
INSERT INTO "languages_names" VALUES (407,'ssa','Nilo-Saharan languages');
INSERT INTO "languages_names" VALUES (408,'ssw','Swati');
INSERT INTO "languages_names" VALUES (409,'suk','Sukuma');
INSERT INTO "languages_names" VALUES (410,'sun','Sundanese');
INSERT INTO "languages_names" VALUES (411,'sus','Susu');
INSERT INTO "languages_names" VALUES (412,'sux','Sumerian');
INSERT INTO "languages_names" VALUES (413,'swa','Swahili');
INSERT INTO "languages_names" VALUES (414,'swe','Swedish');
INSERT INTO "languages_names" VALUES (415,'syc','Classical Syriac');
INSERT INTO "languages_names" VALUES (416,'syr','Syriac');
INSERT INTO "languages_names" VALUES (417,'tah','Tahitian');
INSERT INTO "languages_names" VALUES (418,'tai','Tai languages');
INSERT INTO "languages_names" VALUES (419,'tam','Tamil');
INSERT INTO "languages_names" VALUES (420,'tat','Tatar');
INSERT INTO "languages_names" VALUES (421,'tel','Telugu');
INSERT INTO "languages_names" VALUES (422,'tem','Timne');
INSERT INTO "languages_names" VALUES (423,'ter','Tereno');
INSERT INTO "languages_names" VALUES (424,'tet','Tetum');
INSERT INTO "languages_names" VALUES (425,'tgk','Tajik');
INSERT INTO "languages_names" VALUES (426,'tgl','Tagalog');
INSERT INTO "languages_names" VALUES (427,'tha','Thai');
INSERT INTO "languages_names" VALUES (428,'tib','Tibetan');
INSERT INTO "languages_names" VALUES (429,'tig','Tigre');
INSERT INTO "languages_names" VALUES (430,'tir','Tigrinya');
INSERT INTO "languages_names" VALUES (431,'tiv','Tiv');
INSERT INTO "languages_names" VALUES (432,'tkl','Tokelau');
INSERT INTO "languages_names" VALUES (433,'tlh','Klingon; tlhIngan-Hol');
INSERT INTO "languages_names" VALUES (434,'tli','Tlingit');
INSERT INTO "languages_names" VALUES (435,'tmh','Tamashek');
INSERT INTO "languages_names" VALUES (436,'tog','Tonga (Nyasa)');
INSERT INTO "languages_names" VALUES (437,'ton','Tonga (Tonga Islands)');
INSERT INTO "languages_names" VALUES (438,'tpi','Tok Pisin');
INSERT INTO "languages_names" VALUES (439,'tsi','Tsimshian');
INSERT INTO "languages_names" VALUES (440,'tsn','Tswana');
INSERT INTO "languages_names" VALUES (441,'tso','Tsonga');
INSERT INTO "languages_names" VALUES (442,'tuk','Turkmen');
INSERT INTO "languages_names" VALUES (443,'tum','Tumbuka');
INSERT INTO "languages_names" VALUES (444,'tup','Tupi languages');
INSERT INTO "languages_names" VALUES (445,'tur','Turkish');
INSERT INTO "languages_names" VALUES (446,'tut','Altaic languages');
INSERT INTO "languages_names" VALUES (447,'tvl','Tuvalu');
INSERT INTO "languages_names" VALUES (448,'twi','Twi');
INSERT INTO "languages_names" VALUES (449,'tyv','Tuvinian');
INSERT INTO "languages_names" VALUES (450,'udm','Udmurt');
INSERT INTO "languages_names" VALUES (451,'uga','Ugaritic');
INSERT INTO "languages_names" VALUES (452,'uig','Uighur; Uyghur');
INSERT INTO "languages_names" VALUES (453,'ukr','Ukrainian');
INSERT INTO "languages_names" VALUES (454,'umb','Umbundu');
INSERT INTO "languages_names" VALUES (455,'und','Undetermined');
INSERT INTO "languages_names" VALUES (456,'urd','Urdu');
INSERT INTO "languages_names" VALUES (457,'uzb','Uzbek');
INSERT INTO "languages_names" VALUES (458,'vai','Vai');
INSERT INTO "languages_names" VALUES (459,'ven','Venda');
INSERT INTO "languages_names" VALUES (460,'vie','Vietnamese');
INSERT INTO "languages_names" VALUES (461,'vol','Volapük');
INSERT INTO "languages_names" VALUES (462,'vot','Votic');
INSERT INTO "languages_names" VALUES (463,'wak','Wakashan languages');
INSERT INTO "languages_names" VALUES (464,'wal','Wolaitta; Wolaytta');
INSERT INTO "languages_names" VALUES (465,'war','Waray');
INSERT INTO "languages_names" VALUES (466,'was','Washo');
INSERT INTO "languages_names" VALUES (467,'wel','Welsh');
INSERT INTO "languages_names" VALUES (468,'wen','Sorbian languages');
INSERT INTO "languages_names" VALUES (469,'wln','Walloon');
INSERT INTO "languages_names" VALUES (470,'wol','Wolof');
INSERT INTO "languages_names" VALUES (471,'xal','Kalmyk; Oirat');
INSERT INTO "languages_names" VALUES (472,'xho','Xhosa');
INSERT INTO "languages_names" VALUES (473,'yao','Yao');
INSERT INTO "languages_names" VALUES (474,'yap','Yapese');
INSERT INTO "languages_names" VALUES (475,'yid','Yiddish');
INSERT INTO "languages_names" VALUES (476,'yor','Yoruba');
INSERT INTO "languages_names" VALUES (477,'ypk','Yupik languages');
INSERT INTO "languages_names" VALUES (478,'zap','Zapotec');
INSERT INTO "languages_names" VALUES (479,'zbl','Blissymbols; Blissymbolics; Bliss');
INSERT INTO "languages_names" VALUES (480,'zen','Zenaga');
INSERT INTO "languages_names" VALUES (481,'zgh','Standard Moroccan Tamazight');
INSERT INTO "languages_names" VALUES (482,'zha','Zhuang; Chuang');
INSERT INTO "languages_names" VALUES (483,'znd','Zande languages');
INSERT INTO "languages_names" VALUES (484,'zul','Zulu');
INSERT INTO "languages_names" VALUES (485,'zun','Zuni');
INSERT INTO "languages_names" VALUES (486,'zxx','No linguistic content; Not applicable');
INSERT INTO "languages_names" VALUES (487,'zza','Zaza; Dimili; Dimli; Kirdki; Kirmanjki; Zazaki');
INSERT INTO "languages_roles" VALUES (1,'source');
INSERT INTO "languages_roles" VALUES (2,'original');
INSERT INTO "languages_roles" VALUES (3,'current');
CREATE INDEX IF NOT EXISTS "idx_books_contents_junction" ON "books_contents" (
	"book_id",
	"content_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_people_lookup" ON "books_people_roles" (
	"book_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_tags_lookup" ON "books_tags" (
	"book_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_core_search" ON "contents" (
	"name",
	"type_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_metadata" ON "contents" (
	"type_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_people_lookup" ON "contents_people_roles" (
	"content_id",
	"person_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_tags_lookup" ON "contents_tags" (
	"content_id",
	"tag_id"
);
CREATE INDEX IF NOT EXISTS "idx_contents_temporal" ON "contents" (
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_contents_type_date" ON "contents" (
	"type_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_people_search" ON "people" (
	"name",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_publishers_search" ON "publishers" (
	"name",
	"id"
);
CREATE INDEX IF NOT EXISTS "idx_roles_search" ON "roles" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_series_search" ON "series" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_tags_search" ON "tags" (
	"name"
);
CREATE INDEX IF NOT EXISTS "idx_v_contents_details" ON "contents" (
	"id",
	"type_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_books_core_search" ON "books" (
	"name",
	"series_id",
	"publication_date"
);
CREATE INDEX IF NOT EXISTS "idx_books_metadata" ON "books" (
	"publisher_id",
	"format_id",
	"series_id"
);
CREATE INDEX IF NOT EXISTS "idx_books_series_index" ON "books" (
	"series_id",
	"series_index"
);
CREATE INDEX IF NOT EXISTS "idx_books_temporal" ON "books" (
	"publication_date",
	"acquisition_date",
	"last_modified_date"
);
