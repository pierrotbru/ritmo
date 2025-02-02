import sys
import re
import os

def generate_rust_constants(output_file, sql_files):
    """Genera costanti Rust da uno o più file SQL, gestendo tabelle, viste, indici e altre entità."""
    print(f"Debug: Generating constants from SQL files: {sql_files}")
    print(f"Debug: Output file: {output_file}")
    constants = []

    for sql_file in sql_files:
        try:
            with open(sql_file, 'r') as f:
                sql_content = f.read()
                print(f"Debug: Successfully read SQL file: {sql_file}")
        except FileNotFoundError:
            print(f"File SQL non trovato: {sql_file}", file=sys.stderr)
            continue
        except Exception as e:
            print(f"Errore durante la lettura del file {sql_file}: {e}", file=sys.stderr)
            continue

        # Regex per i commenti (gestisce anche commenti vuoti)
        comment_regex = re.compile(r"^--.*$", re.MULTILINE)

        # Rimuove i commenti
        sql_content = comment_regex.sub("", sql_content)

        # Regex per tabelle normali
        table_regex = re.compile(r"CREATE TABLE IF NOT EXISTS (\w+) \((.*?)\);", re.DOTALL | re.IGNORECASE)

        # Regex per tabelle virtuali
        virtual_regex = re.compile(r"CREATE VIRTUAL TABLE (\w+) \((.*?)\);", re.DOTALL | re.IGNORECASE)

        # Regex per viste
        view_regex = re.compile(r"CREATE\s+VIEW\s+(\w+)\s+AS\s+SELECT", re.DOTALL | re.IGNORECASE)

        # Regex per le colonne (comune a tutti i tipi di tabella)
        column_regex = re.compile(r"(\w+)\s+(?:TEXT|INTEGER|BIGINT|REAL|BLOB)(?:\s+PRIMARY KEY)?(?:\s+AUTOINCREMENT)?(?:\s+NOT NULL)?(?:\s+UNIQUE)?(?:\s+CHECK.*?)?", re.IGNORECASE)

        # Parsa tabelle normali
        for table_match in table_regex.finditer(sql_content):
            table_name = table_match.group(1)
            print(f"Debug: Found table: {table_name}")
            add_table_and_columns_constants(constants, table_name, table_match.group(2), column_regex)

        # Parsa tabelle virtuali
        for virtual_match in virtual_regex.finditer(sql_content):
            table_name = virtual_match.group(1)
            print(f"Debug: Found virtual table: {table_name}")
            add_table_and_columns_constants(constants, table_name, virtual_match.group(2), column_regex)

        # parsing delle viste
        for match in view_regex.finditer(sql_content):
            view_name = match.group(1)
            print(f"Debug: Found view: {view_name}")
            constants.append(f"pub const VIEW_{view_name.upper()}: &str = \"{view_name}\";")

    print(f"Debug: Total constants generated: {len(constants)}")
    print(f"Debug: Constants: {constants}")

    try:
        with open(output_file, 'w') as outfile:
            print(f"Debug: Attempting to write to output file: {output_file}")
            outfile.write("pub mod constants {\n")
            for constant in sorted(set(constants)):
                outfile.write(f"    {constant}\n")
            outfile.write("}\n")
        print(f"Debug: Successfully wrote constants to {output_file}")
    except Exception as e:
        print(f"Errore durante la scrittura del file {output_file}: {e}", file=sys.stderr)
        sys.exit(1)

def add_table_and_columns_constants(constants, table_name, columns_content, column_regex):
    constants.append(f"pub const TABLE_{table_name.upper()}: &str = \"{table_name}\";")
    print(f"Debug: Added table constant for {table_name}")
    
    for column_match in column_regex.finditer(columns_content):
        column_name = column_match.group(1)
        constants.append(f"pub const COLUMN_{table_name.upper()}_{column_name.upper()}: &str = \"{column_name}\";")
        print(f"Debug: Added column constant for {table_name}.{column_name}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Uso: python generate_constants.py <file_output> <file_sql1> [file_sql2 ...]", file=sys.stderr)
        sys.exit(1)

    output_file = sys.argv[1]
    sql_files = sys.argv[2:]
    generate_rust_constants(output_file, sql_files)
