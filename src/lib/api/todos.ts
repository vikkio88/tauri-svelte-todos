import Database from "@tauri-apps/plugin-sql";
import type { Todo } from "../models/Todo";

const db = await Database.load("sqlite:test.db");

function mapRow(row: any): Todo {
  return {
    id: row.id,
    description: row.description,
    done: !!row.done,
    created: new Date(row.created),
    updated: new Date(row.updated),
  };
}

export async function getAll(): Promise<Todo[]> {
  const rows: any[] = await db.select(
    "SELECT * FROM todos ORDER BY created DESC",
  );
  return rows.map(mapRow);
}

export async function add(description: string): Promise<Todo[]> {
  const now = Date.now();

  await db.execute(
    "INSERT INTO todos (description, done, created, updated) VALUES (?, ?, ?, ?)",
    [description, 0, now, now],
  );

  return getAll();
}

export async function remove(id: number): Promise<Todo[]> {
  await db.execute("DELETE FROM todos WHERE id = ?", [id]);
  return getAll();
}

export async function update(id: number, done: boolean): Promise<Todo[]> {
  const now = Date.now();

  await db.execute("UPDATE todos SET done = ?, updated = ? WHERE id = ?", [
    done ? 1 : 0,
    now,
    id,
  ]);

  return getAll();
}
