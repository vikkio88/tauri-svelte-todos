import { invoke } from "@tauri-apps/api/core";
import type { Todo } from "../models/Todo";

export const IPC = {
  BOOT: "boot",
  ADD_TODO: "add_todo",
  REMOVE_TODO: "remove_todo",
  UPDATE_TODO: "update_todo",
} as const;

export async function getAll(): Promise<Todo[]> {
  return invoke<Todo[]>(IPC.BOOT);
}

export async function add(description: string): Promise<Todo[]> {
  const res = await invoke<Todo[]>(IPC.ADD_TODO, { description });
  return res;
}

export async function remove(idx: number): Promise<Todo[]> {
  return await invoke<Todo[]>(IPC.REMOVE_TODO, { idx });
}

export async function update(idx: number, done: boolean): Promise<Todo[]> {
  return await invoke<Todo[]>(IPC.UPDATE_TODO, { idx, done });
}
