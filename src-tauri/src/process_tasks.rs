use crate::{AppState, Mutex, State, Task, WorkingFile};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

const BLANK_NAME: &str = "<BLANK>";

fn task_custom_text(file: &mut WorkingFile, text: &String, at_start: &bool, active: &bool) {
    if *active {
        let file_stem: String;
        let file_extension: String;
        let file_name = file.target.file_name().unwrap().to_string_lossy();
        let start_period: bool = file_name.starts_with(".");
        let has_period: bool = file_name.contains(".");

        // NOTE: if the first char is a period, the file stem will be entire file name
        if file_name == BLANK_NAME {
            file_stem = "".to_string();
        } else if let Some(t) = file.target.file_stem() {
            file_stem = t.to_string_lossy().to_string();
        } else {
            file_stem = "".to_string();
        }

        // NOTE: if there are no periods, the extension is empty
        if let Some(t) = file.target.extension() {
            file_extension = t.to_string_lossy().to_string();
        } else {
            file_extension = "".to_string();
        }

        if *at_start {
            if start_period || !has_period {
                file.target.set_file_name(format!("{text}{file_stem}"));
            } else {
                file.target.set_file_name(format!("{text}{file_stem}.{file_extension}"));
            }
        } else {
            if start_period || !has_period {
                file.target.set_file_name(format!("{file_stem}{text}"));
            } else {
                file.target.set_file_name(format!("{file_stem}{text}.{file_extension}"));
            }
        };
    }
}
fn task_find_and_replace(file: &mut WorkingFile, find_text: &str, replace_text: &str, active: &bool) {
    if *active {
        let mut new_file_name: String;

        let target_file_name = file.target.file_name();

        if let Some(t) = target_file_name {
            let temp_val = t.to_string_lossy().to_string();

            if temp_val == BLANK_NAME {
                new_file_name = BLANK_NAME.to_string();
            } else {
                new_file_name = temp_val.replace(find_text, replace_text);
            }
        } else {
            new_file_name = BLANK_NAME.to_string();
        }

        if new_file_name.is_empty() {
            new_file_name = BLANK_NAME.to_string();
        }

        file.target.set_file_name(new_file_name);
    }
}
fn task_clear_all(file: &mut WorkingFile, active: &bool) {
    if *active {
        let file_extension: String;

        if let Some(t) = file.target.extension() {
            file_extension = t.to_string_lossy().to_string();
            file.target.set_file_name(format!(".{file_extension}"));
        } else {
            file.target.set_file_name(BLANK_NAME);
        }
    }
}
fn task_change_case(file: &mut WorkingFile, case_choice: &u8, active: &bool) {
    if *active {
        let new_file_name: String;

        if let Some(t) = file.target.file_name() {
            new_file_name = t.to_string_lossy().to_string();
        } else {
            new_file_name = "".to_string();
        }

        if new_file_name != BLANK_NAME {
            match *case_choice {
                0 => {
                    file.target.set_file_name(new_file_name.to_lowercase());
                }
                1 => {
                    file.target.set_file_name(new_file_name.to_uppercase());
                }
                _ => {}
            }
        }
    }
}
fn task_num_sequence(
    file: &mut WorkingFile,
    index: usize,
    start_num: &u64,
    num_padding: &u64,
    at_start: &bool,
    separator: &String,
    active: &bool,
) {
    if *active {
        let file_stem: String;
        let file_extension: String;
        let index_u64 = u64::try_from(index).unwrap();
        let num_padding_usize: usize = usize::try_from(*num_padding).unwrap();
        let file_name = file.target.file_name().unwrap().to_string_lossy();

        if file_name == BLANK_NAME {
            file_stem = "".to_string();
        } else if let Some(t) = file.target.file_stem() {
            file_stem = t.to_string_lossy().to_string();
        } else {
            file_stem = "".to_string();
        }

        let raw_sequence_num = index_u64 + start_num;
        let sequence_num = format!("{raw_sequence_num:0num_padding_usize$}");

        if file_stem.is_empty() {
            file.target.set_file_name(&sequence_num);
        } else if let Some(t) = file.target.extension() {
            file_extension = t.to_string_lossy().to_string();

            if *at_start {
                file.target.set_file_name(format!("{sequence_num}{separator}{file_stem}.{file_extension}"));
            } else {
                file.target.set_file_name(format!("{file_stem}{separator}{sequence_num}.{file_extension}"));
            }
        } else {
            if *at_start {
                file.target.set_file_name(format!("{sequence_num}{separator}{file_stem}"));
            } else {
                file.target.set_file_name(format!("{file_stem}{separator}{sequence_num}"));
            }
        }
    }
}
fn task_date(file: &mut WorkingFile, year: &u8, month: &bool, day: &bool, at_start: &bool, separator: &String, active: &bool) {
    if *active {
        let file_metadata = file.source.metadata();

        if let Ok(t) = file_metadata {
            let systime = t.modified();

            if let Ok(j) = systime {
                let datetime = OffsetDateTime::from(j);
                let mut dates_combined_vector: Vec<String> = vec![];

                let year_val = datetime.year().to_string();
                match *year {
                    0 => {
                        dates_combined_vector.push(year_val);
                    }
                    1 => {
                        dates_combined_vector.push(year_val.chars().take(year_val.len().saturating_sub(2)).collect());
                    }
                    _ => {}
                };

                if *month {
                    let month_val = datetime.month() as u8;
                    dates_combined_vector.push(format!("{month_val:02}"));
                }
                if *day {
                    let day_val = datetime.day();
                    dates_combined_vector.push(format!("{day_val:02}"));
                }
                let dates_formatted_string: String = dates_combined_vector.join(separator);

                let file_stem: String;
                let file_extension: String;
                let file_name = file.target.file_name().unwrap().to_string_lossy();

                if file_name == BLANK_NAME {
                    file_stem = "".to_string();
                } else if let Some(t) = file.target.file_stem() {
                    file_stem = t.to_string_lossy().to_string();
                } else {
                    file_stem = "".to_string();
                }

                if file_stem.is_empty() {
                    file.target.set_file_name(&dates_formatted_string);
                } else if *at_start {
                    if let Some(t) = file.target.extension() {
                        file_extension = t.to_string_lossy().to_string();
                        file.target.set_file_name(format!("{dates_formatted_string}{separator}{file_stem}.{file_extension}"));
                    } else {
                        file.target.set_file_name(format!("{dates_formatted_string}{separator}{file_stem}"));
                    }
                } else {
                    if let Some(t) = file.target.extension() {
                        file_extension = t.to_string_lossy().to_string();
                        file.target.set_file_name(format!("{file_stem}{separator}{dates_formatted_string}.{file_extension}"));
                    } else {
                        file.target.set_file_name(format!("{file_stem}{separator}{dates_formatted_string}"));
                    }
                }
            } else {
                // able to access metadata but not file modified info
                // i guess we just skip this step altogether for now
                // should probably bubble up an error to the user
            }
        } else {
            // This means we can't access metadata on file at all
            // i guess we just skip this step altogether for now
            // should probably bubble up an error to the user
        }
    }
}
fn task_time(file: &mut WorkingFile, at_start: &bool, separator: &String, active: &bool) {
    if *active {
        let file_metadata = file.source.metadata();

        if let Ok(t) = file_metadata {
            let systime = t.modified();

            if let Ok(j) = systime {
                let datetime = OffsetDateTime::from(j);
                let datetime_hour = format!("{:02}", datetime.hour());
                let datetime_minute = format!("{:02}", datetime.minute());
                let datetime_second = format!("{:02}", datetime.second());

                let file_stem: String;
                let file_extension: String;
                let file_name = file.target.file_name().unwrap().to_string_lossy();

                if file_name == BLANK_NAME {
                    file_stem = "".to_string();
                } else if let Some(t) = file.target.file_stem() {
                    file_stem = t.to_string_lossy().to_string();
                } else {
                    file_stem = "".to_string();
                }

                if file_stem.is_empty() {
                    file.target.set_file_name(format!("{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}"));
                } else if *at_start {
                    if let Some(t) = file.target.extension() {
                        file_extension = t.to_string_lossy().to_string();
                        file.target.set_file_name(format!(
                                            "{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}{separator}{file_stem}.{file_extension}"
                                        ));
                    } else {
                        file.target.set_file_name(format!(
                            "{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}{separator}{file_stem}"
                        ));
                    }
                } else {
                    if let Some(t) = file.target.extension() {
                        file_extension = t.to_string_lossy().to_string();
                        file.target.set_file_name(format!(
                                            "{file_stem}{separator}{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}.{file_extension}"
                                        ));
                    } else {
                        file.target.set_file_name(format!(
                            "{file_stem}{separator}{datetime_hour}{separator}{datetime_minute}{separator}{datetime_second}"
                        ));
                    }
                }
            } else {
                // able to access metadata but not file modified info
                // i guess we just skip this step altogether for now
                // should probably bubble up an error to the user
            }
        } else {
            // This means we can't access metadata on file at all
            // i guess we just skip this step altogether for now
            // should probably bubble up an error to the user
        }
    }
}
fn task_filter_name(file: &mut WorkingFile, inclusive: &bool, name: &str) {
    if !name.is_empty() {
        if let Some(file_name) = file.target.file_name() {
            let file_name_string = file_name.to_string_lossy();
            let found = file_name_string.contains(name);

            if (found && *inclusive) || (!found && !inclusive) {
                file.active = false;
            }
        }
    }

    // inclusive means that if we find it, it is filtered
    // if it's NOT inclusive, if it is NOT found, it will be filtered
}
fn task_filter_doc_type(file: &mut WorkingFile, inclusive: &bool, doc_types: &[String]) {
    let mut found = false;

    if let Some(suffix) = file.target.extension() {
        let suffix_lower = suffix.to_string_lossy().to_lowercase();

        doc_types.iter().for_each(|doc_type| {
            let cleaned = doc_type.trim_start_matches('.').to_lowercase();
            if cleaned == suffix_lower {
                found = true;
            }
        });
    };

    if (found && *inclusive) || (!found && !inclusive) {
        file.active = false;
    }
}
fn task_filter_time(file: &mut WorkingFile, before: &bool, time: &Option<String>) {
    if let Some(time) = time {
        if let Ok(filter_datetime) = OffsetDateTime::parse(time, &Iso8601::DEFAULT) {
            if let Ok(metadata) = file.source.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let file_datetime = OffsetDateTime::from(modified);

                    if *before {
                        // "Filter Older Dates" — hide files older than selected date
                        if file_datetime < filter_datetime {
                            file.active = false;
                        }
                    } else {
                        // "Filter Newer Dates" — hide files newer than selected date
                        if file_datetime > filter_datetime {
                            file.active = false;
                        }
                    }
                }
            }
        }
    }
}
fn task_filter_time_period(file: &mut WorkingFile, inclusive: &bool, start_time: &Option<String>, end_time: &Option<String>) {
    if let (Some(start_time), Some(end_time)) = (start_time, end_time) {
        if let (Ok(start_dt), Ok(end_dt)) =
            (OffsetDateTime::parse(start_time, &Iso8601::DEFAULT), OffsetDateTime::parse(end_time, &Iso8601::DEFAULT))
        {
            if let Ok(metadata) = file.source.metadata() {
                if let Ok(modified) = metadata.modified() {
                    let file_datetime = OffsetDateTime::from(modified);
                    let in_range = file_datetime >= start_dt && file_datetime <= end_dt;

                    if (in_range && *inclusive) || (!in_range && !inclusive) {
                        file.active = false;
                    }
                }
            }
        }
    }
}

fn task_filter_size(file: &mut WorkingFile, greater_than: &bool, byte_base_size: &u64, size: &u64) {
    if let Ok(metadata) = file.source.metadata() {
        let file_size = metadata.len();
        // byte_base_size is a tier: 0=Bytes, 1=KB, 2=MB, 3=GB, 4=TB
        let multiplier: u64 = 1024_u64.pow(*byte_base_size as u32);
        let search_size = multiplier * size;

        if *greater_than {
            if file_size >= search_size {
                file.active = false;
            }
        } else {
            if file_size < search_size {
                file.active = false;
            }
        }
    }
}

#[tauri::command]
pub fn process_tasks_on_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let state = &mut *state;

    for (index, file) in &mut state.working_files.iter_mut().enumerate() {
        file.target = file.source.clone();
        file.active = true;

        for task in &state.tasks {
            match task {
                Task::CustomText { text, at_start, active } => {
                    task_custom_text(file, text, at_start, active);
                }
                Task::FindAndReplace { find_text, replace_text, active } => {
                    task_find_and_replace(file, find_text, replace_text, active);
                }

                Task::ClearAll { active } => {
                    task_clear_all(file, active);
                }

                Task::ChangeCase { case_choice, active } => {
                    task_change_case(file, case_choice, active);
                }
                Task::NumSequence { start_num, num_padding, separator, at_start, active } => {
                    task_num_sequence(file, index, start_num, num_padding, at_start, separator, active);
                }

                Task::Date { year, month, day, at_start, separator, active } => {
                    task_date(file, year, month, day, at_start, separator, active);
                }

                Task::Time { at_start, separator, active } => {
                    task_time(file, at_start, separator, active);
                }
                Task::FilterName { inclusive, name } => {
                    task_filter_name(file, inclusive, name);
                }
                Task::FilterDocType { inclusive, doc_types } => {
                    task_filter_doc_type(file, inclusive, doc_types);
                }
                Task::FilterTime { before, time } => {
                    task_filter_time(file, before, time);
                }
                Task::FilterTimePeriod { inclusive, start_time, end_time } => {
                    task_filter_time_period(file, inclusive, start_time, end_time);
                }
                Task::FilterSize { greater_than, byte_base_size, size } => {
                    task_filter_size(file, greater_than, byte_base_size, size);
                }
            }
        }
    }
}
