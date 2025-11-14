import Database from "@tauri-apps/plugin-sql";
import type { Todo } from "../models/Todo";

const db = async () => await Database.load("sqlite:db.sqlite");

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
  const d = await db();
  const rows: any[] = await d.select(
    "SELECT * FROM todos ORDER BY created DESC",
  );

  console.log({ rows });
  return rows.map(mapRow);
}

export async function add(description: string): Promise<Todo[]> {
  const now = Date.now();
  const d = await db();
  await d.execute(
    "INSERT INTO todos (description, done, created, updated) VALUES (?, ?, ?, ?)",
    [description, 0, now, now],
  );

  return getAll();
}

export async function remove(id: number): Promise<Todo[]> {
  const d = await db();
  await d.execute("DELETE FROM todos WHERE id = ?", [id]);
  return getAll();
}

export async function update(id: number, done: boolean): Promise<Todo[]> {
  const now = Date.now();
  const d = await db();
  await d.execute("UPDATE todos SET done = ?, updated = ? WHERE id = ?", [
    done ? 1 : 0,
    now,
    id,
  ]);

  return getAll();
}
