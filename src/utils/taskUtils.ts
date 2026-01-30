import type { TaskWithId } from "./types";

export function deleteTask(list: TaskWithId[], index: number): void {
    list.splice(index, 1);
}

export function moveTaskUp(list: TaskWithId[], index: number): boolean {
    if (index > 0) {
        const [item] = list.splice(index, 1);
        list.splice(index - 1, 0, item);
        return true;
    }
    return false;
}

export function moveTaskDown(list: TaskWithId[], index: number): boolean {
    if (index < list.length - 1) {
        const [item] = list.splice(index, 1);
        list.splice(index + 1, 0, item);
        return true;
    }
    return false;
}
