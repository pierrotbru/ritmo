import re

def format_markdown(input_path, output_path):
    with open(input_path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    md_lines = ["# Conversazione Copilot Chat\n"]
    domanda_num = 1
    in_code = False

    for line in lines:
        # Riconosce domande dell'utente (puoi personalizzare il pattern)
        if line.strip().startswith('>') or line.lower().startswith('domanda') or line.strip().startswith('Q:'):
            md_lines.append(f"\n## Domanda {domanda_num}\n")
            domanda_num += 1
            md_lines.append(f"> {line.strip('> ').strip()}\n")
        # Riconosce risposte dell'AI
        elif line.lower().startswith('**copilot') or line.lower().startswith('risposta copilot') or line.lower().startswith('assistant'):
            md_lines.append("\n**Risposta Copilot:**\n")
        # Blocchi di codice (inizio/fine)
        elif line.strip().startswith("```"):
            if not in_code:
                # Inizio blocco code
                md_lines.append(line)
                in_code = True
            else:
                md_lines.append(line)
                in_code = False
        else:
            md_lines.append(line)

    with open(output_path, "w", encoding="utf-8") as f:
        f.writelines(md_lines)

if __name__ == "__main__":
    format_markdown("conversazione.txt", "conversazione.md")
    print("Conversazione convertita e salvata in conversazione.md")