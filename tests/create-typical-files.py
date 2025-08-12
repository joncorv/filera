#!/usr/bin/env python3
"""
Generate 100 typical, well-behaved files for happy path testing of file renaming applications.
These are the kinds of files users commonly have in their directories.
"""

import os
import random
from pathlib import Path

def create_typical_files(output_dir="typical_files"):
    """Create 100 normal, well-behaved files for happy path testing."""
    
    # Create output directory
    Path(output_dir).mkdir(exist_ok=True)
    os.chdir(output_dir)
    
    typical_files = []
    
    # 1. Document files (15 files)
    document_files = [
        'meeting_notes.txt', 'project_proposal.doc', 'budget_2024.xlsx',
        'presentation_draft.ppt', 'contract_final.pdf', 'readme.md',
        'user_manual.txt', 'requirements.doc', 'invoice_march.pdf',
        'report_quarterly.xlsx', 'agenda_meeting.doc', 'notes_personal.txt',
        'checklist_project.md', 'summary_executive.pdf', 'draft_letter.txt'
    ]
    typical_files.extend(document_files)
    
    # 2. Image files (12 files)
    image_files = [
        'vacation_photo.jpg', 'profile_picture.png', 'screenshot_desktop.png',
        'family_portrait.jpg', 'logo_company.svg', 'diagram_flowchart.png',
        'photo_sunset.jpg', 'icon_app.png', 'banner_website.jpg',
        'chart_sales.png', 'image_background.jpg', 'thumbnail_video.png'
    ]
    typical_files.extend(image_files)
    
    # 3. Code and development files (10 files)
    code_files = [
        'main.py', 'config.json', 'app.js', 'style.css', 'index.html',
        'database.sql', 'server.php', 'component.tsx', 'utils.java', 'makefile'
    ]
    typical_files.extend(code_files)
    
    # 4. Media files (8 files)
    media_files = [
        'song_favorite.mp3', 'podcast_episode.mp3', 'video_tutorial.mp4',
        'movie_trailer.mp4', 'audiobook_chapter.m4a', 'ringtone_custom.mp3',
        'presentation_video.mov', 'interview_recording.wav'
    ]
    typical_files.extend(media_files)
    
    # 5. Archive and backup files (8 files)
    archive_files = [
        'backup_database.zip', 'project_archive.tar.gz', 'photos_2023.rar',
        'documents_old.7z', 'website_backup.zip', 'data_export.csv',
        'settings_backup.json', 'files_compressed.tar'
    ]
    typical_files.extend(archive_files)
    
    # 6. System and configuration files (7 files)
    system_files = [
        'config.ini', 'settings.xml', 'preferences.plist', 'data.db',
        'cache.tmp', 'log_application.log', 'error_report.txt'
    ]
    typical_files.extend(system_files)
    
    # 7. Versioned files (10 files)
    versioned_files = [
        'document_v1.txt', 'presentation_v2.ppt', 'design_v3.psd',
        'contract_final_v1.pdf', 'proposal_draft_v2.doc', 'budget_2024_v1.xlsx',
        'app_release_v1.0.zip', 'manual_user_v2.1.pdf', 'schema_db_v3.sql',
        'template_email_v1.html'
    ]
    typical_files.extend(versioned_files)
    
    # 8. Date-stamped files (10 files)
    date_files = [
        'report_2024_01_15.pdf', 'backup_2024_02_01.zip', 'notes_2024_03_10.txt',
        'meeting_2024_04_05.doc', 'data_2024_05_20.csv', 'log_2024_06_30.txt',
        'photo_2024_07_04.jpg', 'video_2024_08_12.mp4', 'invoice_2024_09_01.pdf',
        'summary_2024_10_15.xlsx'
    ]
    typical_files.extend(date_files)
    
    # 9. Common office files (10 files)
    office_files = [
        'annual_report.pdf', 'employee_handbook.doc', 'timesheet_template.xlsx',
        'expense_report.xlsx', 'company_policy.pdf', 'org_chart.ppt',
        'project_timeline.xlsx', 'client_list.csv', 'product_catalog.pdf',
        'training_materials.ppt'
    ]
    typical_files.extend(office_files)
    
    # 10. Personal files (10 files)
    personal_files = [
        'recipe_chocolate_cake.txt', 'shopping_list.txt', 'travel_itinerary.pdf',
        'book_recommendations.md', 'workout_routine.txt', 'grocery_budget.xlsx',
        'hobby_project_notes.txt', 'home_improvement.doc', 'garden_plan.pdf',
        'personal_journal.txt'
    ]
    typical_files.extend(personal_files)
    
    # Create all files
    created_files = []
    file_types = {
        '.txt': 'This is a text document with some sample content.\nUsed for testing file renaming applications.',
        '.doc': 'Microsoft Word document content placeholder.\nContains formatted text and paragraphs.',
        '.pdf': 'PDF document content placeholder.\nPortable Document Format file.',
        '.xlsx': 'Excel spreadsheet content placeholder.\nContains rows and columns of data.',
        '.ppt': 'PowerPoint presentation content placeholder.\nContains slides and multimedia elements.',
        '.jpg': 'JPEG image file content placeholder.\nCompressed image format.',
        '.png': 'PNG image file content placeholder.\nLossless image format with transparency.',
        '.mp3': 'MP3 audio file content placeholder.\nCompressed audio format.',
        '.mp4': 'MP4 video file content placeholder.\nCompressed video format.',
        '.zip': 'ZIP archive file content placeholder.\nCompressed file container.',
        '.json': '{\n  "type": "configuration",\n  "version": "1.0",\n  "settings": {}\n}',
        '.csv': 'Name,Type,Size\nFile1,Document,1024\nFile2,Image,2048\nFile3,Video,4096',
        '.html': '<!DOCTYPE html>\n<html>\n<head><title>Sample</title></head>\n<body><h1>Sample HTML</h1></body>\n</html>',
        '.css': 'body {\n  font-family: Arial, sans-serif;\n  margin: 0;\n  padding: 20px;\n}',
        '.js': 'function sampleFunction() {\n  console.log("Hello, World!");\n  return true;\n}',
        '.py': '#!/usr/bin/env python3\n# Sample Python script\ndef main():\n    print("Hello, World!")\n\nif __name__ == "__main__":\n    main()',
        '.md': '# Sample Markdown\n\nThis is a **sample** markdown file.\n\n- Item 1\n- Item 2\n- Item 3'
    }
    
    for filename in typical_files:
        try:
            # Determine file extension and content
            ext = '.' + filename.split('.')[-1] if '.' in filename else '.txt'
            content = file_types.get(ext, 'Sample file content for testing purposes.')
            
            # Create file with appropriate content
            with open(filename, 'w', encoding='utf-8') as f:
                f.write(content)
            
            created_files.append(filename)
            print(f"Created: {filename}")
            
        except Exception as e:
            print(f"Failed to create {filename}: {e}")
    
    print(f"\nSuccessfully created {len(created_files)} typical files in '{output_dir}' directory")
    print("\nFile categories created:")
    print("- Document files (.txt, .doc, .pdf, .xlsx, .ppt)")
    print("- Image files (.jpg, .png, .svg)")
    print("- Code files (.py, .js, .css, .html, .json)")
    print("- Media files (.mp3, .mp4, .wav, .mov)")
    print("- Archive files (.zip, .tar.gz, .rar)")
    print("- System files (.ini, .xml, .log, .db)")
    print("- Versioned files (v1, v2, etc.)")
    print("- Date-stamped files (YYYY_MM_DD format)")
    print("- Office documents")
    print("- Personal files")
    
    return created_files

if __name__ == "__main__":
    print("Creating 100 typical files for happy path testing...")
    print("=" * 60)
    
    files = create_typical_files()
    
    print("=" * 60)
    print("Files created! These represent common, well-behaved files for testing.")
    print("Perfect for happy path development and basic functionality testing.")
