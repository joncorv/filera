use crate::{AppState, Mutex, State, Task};
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;

#[tauri::command]
pub fn process_tasks_on_working_files(state: &State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    let tasks = &state.tasks.clone();

    // reset status to true on each change
    for file in &mut state.working_files.iter_mut() {
        file.active = true;
    }

    for (index, file) in &mut state.working_files.iter_mut().enumerate() {
        // for each WorkingFile in the array, copy from source => target
        file.target = file.source.clone();

        // iterate over all tasks in task_list
        for task in tasks {
            match task {
                Task::CustomText { text, at_start, active } => {
                    if *active {
                        let file_stem: String;
                        let file_extension: String;
                        let file_name = file.target.file_name().unwrap().to_string_lossy();
                        let start_period: bool = file_name.starts_with(".");
                        let has_period: bool = file_name.contains(".");

                        // NOTE: if the first char is a period, the file stem will be entire file name
                        if let Some(t) = file.target.file_stem() {
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
                Task::FindAndReplace {
                    find_text,
                    replace_text,
                    active,
                } => {
                    if *active {
                        let mut new_file_name: String;

                        let target_file_name = file.target.file_name();

                        // if the file name exists
                        if let Some(t) = target_file_name {
                            // store the file name into temp_val string
                            let temp_val = t.to_string_lossy().to_string();

                            // if the file name is <BLANK>
                            if temp_val == "<BLANK>".to_string() {
                                // because the string is blank, we don't operate.
                                new_file_name = "<BLANK>".to_string();
                            } else {
                                // because it's not blank, we can operate on it
                                new_file_name = temp_val.replace(find_text, replace_text);
                            }
                        } else {
                            new_file_name = "".to_string();
                        }

                        // return the <BLANK> token if new_file_name is empty
                        if new_file_name.is_empty() {
                            new_file_name = "<BLANK>".to_string();
                        }

                        file.target.set_file_name(new_file_name);
                    }
                }

                // TODO: if filename starts w/ or does not contain ".", we return a special "<BLANK>" token that needs to be accounted for as an edge case in the rest of the tasks
                Task::ClearAll { active } => {
                    if *active {
                        let file_extension: String;

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();
                            file.target.set_file_name(format!(".{file_extension}"));
                        } else {
                            file.target.set_file_name("<BLANK>");
                        }
                    }
                }

                // TODO: Need to build in cases for TitleCase.
                // TODO: account for <BLANK> edge case
                Task::ChangeCase { case_choice, active } => {
                    if *active {
                        let new_file_name: String;

                        if let Some(t) = file.target.file_name() {
                            new_file_name = t.to_string_lossy().to_string();
                        } else {
                            new_file_name = "".to_string();
                        }

                        match *case_choice {
                            0_u8 => {
                                // lowercase
                                file.target.set_file_name(new_file_name.to_lowercase());
                            }
                            1_u8 => {
                                // UPPERCASE
                                file.target.set_file_name(new_file_name.to_uppercase());
                            }
                            2_u8..=u8::MAX => {
                                // figure out other case options later
                                // file.target.set_file_name(new_file_name);
                            }
                        }
                    }
                }
                // TODO: account for <BLANK> edge case
                Task::NumSequence {
                    start_num,
                    num_padding,
                    separator,
                    at_start,
                    active,
                } => {
                    if *active {
                        let file_stem: String;
                        let file_extension: String;
                        let index_u64 = u64::try_from(index).unwrap();
                        let num_padding_usize: usize = usize::try_from(*num_padding).unwrap();

                        if let Some(t) = file.target.file_stem() {
                            file_stem = t.to_string_lossy().to_string();
                        } else {
                            file_stem = "".to_string();
                        }

                        let raw_sequence_num = index_u64 + start_num;
                        let sequence_num = format!("{raw_sequence_num:0num_padding_usize$}");

                        if let Some(t) = file.target.extension() {
                            file_extension = t.to_string_lossy().to_string();

                            // build the file name with extension, and number at start
                            if *at_start {
                                file.target
                                    .set_file_name(format!("{sequence_num}{separator}{file_stem}.{file_extension}"));
                            } else
                            // build the file name with extension, and number at the end
                            {
                                file.target
                                    .set_file_name(format!("{file_stem}{separator}{sequence_num}.{file_extension}"));
                            }
                        } else {
                            // build the file name without extension, and number at start
                            if *at_start {
                                file.target.set_file_name(format!("{sequence_num}{separator}{file_stem}"));
                            } else
                            // build the file name without an extension, and number at the end
                            {
                                file.target.set_file_name(format!("{file_stem}{separator}{sequence_num}",));
                            }
                        }
                    }
                }

                // TODO: account for <BLANK> edge case
                Task::Date {
                    year,
                    month,
                    day,
                    at_start,
                    separator,
                    active,
                } => {
                    if *active {
                        let file_metadata = file.source.metadata();

                        if let Ok(t) = file_metadata {
                            // create SystemTime Instance from metadata
                            let systime = t.modified();

                            if let Ok(j) = systime {
                                // able to access metadata and file modified info
                                let datetime = OffsetDateTime::from(j);
                                let mut dates_combined_vector: Vec<String> = vec![];

                                // calculate year by user input
                                let year_val = datetime.year().to_string();
                                match *year {
                                    0 => {
                                        dates_combined_vector.push(year_val);
                                    }
                                    1 => {
                                        dates_combined_vector.push(year_val.chars().take(year_val.len().saturating_sub(2)).collect());
                                    }
                                    2_u8..=std::u8::MAX => {}
                                };

                                // calculate month & day by user input
                                if *month {
                                    let month_val = datetime.month() as u8;
                                    dates_combined_vector.push(format!("{month_val:02}"));
                                }
                                if *day {
                                    let day_val = datetime.day();
                                    dates_combined_vector.push(format!("{day_val:02}"));
                                }
                                let dates_formatted_string: String = dates_combined_vector.join(separator);

                                //TODO: Edge Case: Leading "."
                                //TODO: Edge Case: Contains no "."
                                //TODO: Contains the <<SPECIAL_CLEAR_a432LKJ>> value
                                //OK let's write the logic here:
                                // if *at_start -> dates + file_stem + "." + file_extension
                                let file_stem: String;
                                let file_extension: String;
                                // let has_extension: bool;
                                // let has_stem: bool;

                                // if there is a file stem, use it, or make it blank
                                if let Some(t) = file.target.file_stem() {
                                    file_stem = t.to_string_lossy().to_string();
                                } else {
                                    file_stem = "".to_string();
                                }

                                if *at_start {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target
                                            .set_file_name(format!("{dates_formatted_string}{separator}{file_stem}.{file_extension}"));
                                    } else {
                                        file.target.set_file_name(format!("{dates_formatted_string}{separator}{file_stem}"));
                                    }
                                } else {
                                    if let Some(t) = file.target.extension() {
                                        file_extension = t.to_string_lossy().to_string();
                                        file.target
                                            .set_file_name(format!("{file_stem}{separator}{dates_formatted_string}.{file_extension}"));
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

                // TODO: Update to use a 24 hour clock, and include am and pm automatically
                // TODO: account for <BLANK> edge case
                Task::Time {
                    at_start,
                    separator,
                    active,
                } => {
                    if *active {
                        let file_metadata = file.source.metadata();

                        if let Ok(t) = file_metadata {
                            // create SystemTime Instance from metadata
                            let systime = t.modified();

                            if let Ok(j) = systime {
                                // able to access metadata and file modified info
                                let datetime = OffsetDateTime::from(j);
                                let datetime_hour = format!("{:02}", datetime.hour());
                                let datetime_minute = format!("{:02}", datetime.minute());
                                let datetime_second = format!("{:02}", datetime.second());

                                // now that we have system time, let's build the file
                                // get file_stem and file_extension
                                let file_stem: String;
                                let file_extension: String;

                                // if there is a file stem, use it, or make it blank
                                if let Some(t) = file.target.file_stem() {
                                    file_stem = t.to_string_lossy().to_string();
                                } else {
                                    file_stem = "".to_string();
                                }

                                // if there isn't an extension, we do need to change how we approach building here

                                if *at_start {
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
                Task::FilterName { inclusive, name } => {
                    // println!("filter processing now status on {}: {}", file.source.to_string_lossy(), file.active);

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
                Task::FilterDocType { inclusive, doc_types } => {
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
                Task::FilterTime { before, time } => {
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
                Task::FilterTimePeriod {
                    inclusive,
                    start_time,
                    end_time,
                } => {
                    if let (Some(start_time), Some(end_time)) = (start_time, end_time) {
                        if let (Ok(start_dt), Ok(end_dt)) = (
                            OffsetDateTime::parse(start_time, &Iso8601::DEFAULT),
                            OffsetDateTime::parse(end_time, &Iso8601::DEFAULT),
                        ) {
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
                Task::FilterSize {
                    greater_than,
                    byte_base_size,
                    size,
                } => {
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
            }
        }
    }
}
