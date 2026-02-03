//  <-- === WorkingFile Interface === -->
export interface FileStatus {
    old_file_name: string;
    new_file_name: string;
    active: boolean;
}

//  <-- === Task Type. Contains all Sub Tasks within === -->
export type Task =
    | { CustomText: { text: string; at_start: boolean; active: boolean } }
    | {
          FindAndReplace: {
              find_text: string;
              replace_text: string;
              active: boolean;
          };
      }
    | { ClearAll: { active: boolean } }
    | { ChangeCase: { case_choice: number; active: boolean } }
    | {
          NumSequence: {
              start_num: number;
              num_padding: number;
              at_start: boolean;
              separator: string;
              active: boolean;
          };
      }
    | {
          Date: {
              year: number;
              month: boolean;
              day: boolean;
              separator: string;
              at_start: boolean;
              active: boolean;
          };
      }
    | {
          Time: {
              at_start: boolean;
              separator: string;
              active: boolean;
          };
      }
    | { FilterName: { inclusive: boolean; name: string } }
    | { FilterDocType: { inclusive: boolean; doc_types: string[] } }
    | { FilterTimePeriod: { inclusive: boolean; start_time: Date | null; end_time: Date | null } }
    | { FilterTime: { before: boolean; time: Date | null } }
    | { FilterSize: { greater_than: boolean; byte_base_size: number; size: number } };

//  <-- === Add unique ID to each task. Needed for proper animation in the DOM === -->
export interface TaskWithId {
    id: number;
    task: Task;
}

//  <-- === Task Factory Functions === -->
export const createCustomTextTask = (): Task => ({
    CustomText: { text: "", at_start: true, active: true },
});

export const createFindReplaceTask = (): Task => ({
    FindAndReplace: { find_text: "", replace_text: "", active: true },
});

export const createClearAllTask = (): Task => ({
    ClearAll: { active: true },
});

export const createChangeCaseTask = (): Task => ({
    ChangeCase: { case_choice: 0, active: true },
});

export const createNumSequenceTask = (): Task => ({
    NumSequence: { start_num: 0, num_padding: 4, at_start: true, separator: "_", active: true },
});

export const createDateTask = (): Task => ({
    Date: { year: 0, month: true, day: true, at_start: true, separator: "_", active: true },
});

export const createTimeTask = (): Task => ({
    Time: { at_start: true, separator: "_", active: true },
});

export const createFilterNameTask = (): Task => ({
    FilterName: { inclusive: true, name: "" },
});

export const createFilterDocTypeTask = (): Task => ({
    FilterDocType: { inclusive: true, doc_types: [] },
});

export const createFilterTimePeriodTask = (): Task => ({
    FilterTimePeriod: { inclusive: true, start_time: null, end_time: null },
});

export const createFilterTimeTask = (): Task => ({
    FilterTime: { before: true, time: null },
});

export const createFilterSizeTask = (): Task => ({
    FilterSize: { greater_than: false, byte_base_size: 2, size: 0 },
});

//  <-- === Type Guards === -->
export const isCustomText = (
    task: Task,
): task is {
    CustomText: { text: string; at_start: boolean; active: boolean };
} => {
    return "CustomText" in task;
};

export const isFindAndReplace = (
    task: Task,
): task is {
    FindAndReplace: {
        find_text: string;
        replace_text: string;
        active: boolean;
    };
} => {
    return "FindAndReplace" in task;
};

export const isClearAll = (
    task: Task,
): task is {
    ClearAll: { active: boolean };
} => {
    return "ClearAll" in task;
};

export const isChangeCase = (
    task: Task,
): task is {
    ChangeCase: { case_choice: number; active: boolean };
} => {
    return "ChangeCase" in task;
};

export const isNumSequence = (
    task: Task,
): task is {
    NumSequence: {
        start_num: number;
        num_padding: number;
        at_start: boolean;
        separator: string;
        active: boolean;
    };
} => {
    return "NumSequence" in task;
};

export const isDate = (
    task: Task,
): task is {
    Date: {
        year: number;
        month: boolean;
        day: boolean;
        at_start: boolean;
        separator: string;
        active: boolean;
    };
} => {
    return "Date" in task;
};

export const isTime = (
    task: Task,
): task is {
    Time: {
        at_start: boolean;
        ampm: boolean;
        separator: string;
        active: boolean;
    };
} => {
    return "Time" in task;
};

export const isFilterName = (
    task: Task,
): task is {
    FilterName: {
        inclusive: boolean;
        name: string;
    };
} => {
    return "FilterName" in task;
};

export const isFilterDocType = (
    task: Task,
): task is {
    FilterDocType: {
        inclusive: boolean;
        doc_types: string[];
    };
} => {
    return "FilterDocType" in task;
};

export const isFilterTimePeriod = (
    task: Task,
): task is {
    FilterTimePeriod: {
        inclusive: boolean;
        start_time: Date | null;
        end_time: Date | null;
    };
} => {
    return "FilterTimePeriod" in task;
};

export const isFilterTime = (
    task: Task,
): task is {
    FilterTime: {
        before: boolean;
        time: Date | null;
    };
} => {
    return "FilterTime" in task;
};

export const isFilterSize = (
    task: Task,
): task is {
    FilterSize: {
        greater_than: boolean;
        byte_base_size: number;
        size: number;
    };
} => {
    return "FilterSize" in task;
};
