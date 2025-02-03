# ritmo
 Start of a new books manager Calibre style

This program is a learning exercise to develop a Calibre like books manager.
The main point is to focus on readers instead of publishers.
To realize this I decided to use a contents object as center, instead of Book object.
contents is the literary part of information, and contains all data about title, author, original language, current language, translator, etc.
while the Book object contains one or more Contents, and all the info related to the books, as publisher, series, acquisition date, format, etc.

The database used is SQLite, and the program is written in Rust.

I am learning Rust and SQL while I write code, so at any point there will be a lot of wrong stuff, or that can be improved.
