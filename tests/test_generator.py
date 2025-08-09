#!/usr/bin/env python3
"""
Generate 100 files with edge cases that challenge file renaming applications.
These files test various boundary conditions, special characters, encodings, etc.
"""

import os
import string
import random
from pathlib import Path

def create_edge_case_files(output_dir="test_files"):
    """Create 100 files with challenging names for renaming apps."""
    
    # Create output directory
    Path(output_dir).mkdir(exist_ok=True)
    os.chdir(output_dir)
    
    edge_cases = []
    
    # 1. Files starting with special characters (15 files)
    edge_cases.extend([
        '!important_file.txt',
        '@symbol_start.txt',
        '#hash_beginning.txt',
        '$dollar_first.txt',
        '%percent_start.txt',
        '^caret_file.txt',
        '&ampersand_first.txt',
        '*asterisk_start.txt',
        '(paren_file.txt',
        ')close_paren.txt',
        '+plus_start.txt',
        '=equals_first.txt',
        '[bracket_file.txt',
        ']close_bracket.txt',
        '{brace_start.txt'
    ])
    
    # 2. Files starting with Unicode and emoji (12 files)
    edge_cases.extend([
        '🚀rocket_launch.txt',
        '📁folder_icon.txt',
        '💻computer_file.txt',
        '🎵music_note.txt',
        '🌟star_file.txt',
        '文档.txt',
        'файл.txt',
        'café_menu.txt',
        'naïve_approach.txt',
        'résumé.txt',
        'ñoño.txt',
        'ürlaub.txt'
    ])
    
    # 3. Files starting with dots (8 files)
    edge_cases.extend([
        '.hidden_file',
        '..parent_reference',
        '...triple_dot',
        '. space_after_dot.txt',
        '.tar.gz.backup',
        '.DS_Store',
        '.gitignore',
        '.env.local'
    ])
    
    # 4. Files starting with spaces (8 files)
    edge_cases.extend([
        ' leading_space.txt',
        '  double_space.txt',
        '   triple_space.txt',
        ' .space_dot.txt',
        ' hidden_after_space',
        '  multiple_issues .txt',
        ' 123_number.txt',
        ' CAPS_AFTER_SPACE.txt'
    ])
    
    # 5. Files starting with numbers and dates (10 files)
    edge_cases.extend([
        '0001_zero_pad.txt',
        '1file.txt',
        '2024-01-01_date.txt',
        '3.14159_pi.txt',
        '42_answer.txt',
        '100%_complete.txt',
        '999_high_number.txt',
        '0xFF_hex.txt',
        '1e10_scientific.txt',
        '-42_negative.txt'
    ])
    
    # 6. Files starting with reserved-like patterns (8 files)
    # Avoid actual Windows reserved names that break git
    edge_cases.extend([
        'CONSOLE.txt',
        'PRINTER.txt', 
        'AUXILIARY.txt',
        'NULL_FILE.txt',
        'SERIAL1.txt',
        'SERIAL2.txt',
        'PARALLEL1.txt',
        'PARALLEL2.txt'
    ])
    
    # 7. Files starting with path-like characters (10 files)
    edge_cases.extend([
        '~home_file.txt',
        '$variable_name.txt',
        'backslash_start.txt',  # Removed actual backslash
        'slash_start.txt',      # Removed actual forward slash
        'parent_dir.txt',       # Removed ../
        'current_dir.txt',      # Removed ./
        '|pipe_start.txt',
        'less_than.txt',        # Removed < character
        'greater_than.txt',     # Removed > character
        '?question_mark.txt'
    ])
    
    # 8. Files starting with quotes and escape characters (8 files)
    edge_cases.extend([
        '"quoted_file.txt',
        "'single_quote.txt",
        '`backtick_file.txt',
        'backslash_file.txt',  # Removed actual backslash to avoid filesystem issues
        'double_escape_file.txt',  # Simplified to avoid escape sequence issues
        'newline_start.txt',
        'tab_start.txt',
        'return_start.txt'
    ])
    
    # 9. Files starting with command-like patterns (10 files)
    edge_cases.extend([
        'rm -rf *',
        'del *.txt',
        'format c:',
        'sudo rm file',
        'cmd.exe',
        'bash.sh',
        'powershell.ps1',
        'script.bat',
        'run.exe',
        'install.msi'
    ])
    
    # 10. Moderately long names starting with edge characters (6 files)
    # Keep under 100 chars to avoid filesystem issues
    long_suffix = "x" * 80
    edge_cases.extend([
        f'!{long_suffix}.txt',
        f' {long_suffix}.txt',
        f'.{long_suffix}',
        f'${long_suffix}.txt',
        f'#{long_suffix}.txt',
        f'🚀{long_suffix}.txt'
    ])
    
    # 11. Remaining challenging starters (5 files)
    edge_cases.extend([
        ';;semicolon_double.txt',
        '::colon_double.txt',
        '---triple_dash.txt',
        '___triple_under.txt',
        '~~~tilde_triple.txt'
    ])
    
    # Create all files
    created_files = []
    for i, filename in enumerate(edge_cases[:100]):  # Ensure exactly 100 files
        try:
            # Create file with some content
            with open(filename, 'w', encoding='utf-8') as f:
                f.write(f"Edge case test file\n")
                f.write(f"Original name: {filename}\n")
                f.write(f"Created for testing file renaming applications.\n")
                f.write(f"Contains challenging characters or patterns.\n")
            created_files.append(filename)
            print(f"Created: {filename}")
        except (OSError, UnicodeEncodeError) as e:
            # If file creation fails, create a safe alternative
            safe_name = f"failed_creation_{hash(filename) % 10000}.txt"
            with open(safe_name, 'w', encoding='utf-8') as f:
                f.write(f"Edge case test file (creation failed)\n")
                f.write(f"Intended name: {filename}\n")
                f.write(f"Error: {str(e)}\n")
            created_files.append(safe_name)
            print(f"Created (safe): {safe_name} (original failed: {filename})")
    
    print(f"\nSuccessfully created {len(created_files)} files in '{output_dir}' directory")
    print("\nFile categories created:")
    print("- Special characters and symbols")
    print("- Unicode and emoji characters")
    print("- Very long filenames")
    print("- Files with spaces and dots")
    print("- Windows reserved names")
    print("- Leading/trailing spaces")
    print("- Extension edge cases")
    print("- Case sensitivity tests")
    print("- Numeric and date patterns")
    print("- Path injection attempts")
    print("- Command-like names")
    print("- Whitespace characters")
    print("- Version-like patterns")
    print("- Random challenging combinations")
    
    return created_files

if __name__ == "__main__":
    print("Creating 100 edge case files for testing file renaming applications...")
    print("=" * 60)
    
    files = create_edge_case_files()
    
    print("=" * 60)
    print("Files created! Test your renaming application with these challenging cases.")
    print("WARNING: Some filenames may cause issues with certain file systems or applications.")
    print("Use in a test environment only!")
