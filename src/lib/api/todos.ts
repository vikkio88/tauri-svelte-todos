import Database from "@tauri-apps/plugin-sql";
import {
  DummyDriver,
  Kysely,
  SqliteAdapter,
  SqliteIntrospector,
  SqliteQueryCompiler,
} from "kysely";
import type { Todo, TodoTable } from "../models/Todo";

type Db = {
  todos: TodoTable;
};

const q = new Kysely<Db>({
  dialect: {
    createAdapter: () => new SqliteAdapter(),
    createDriver: () => new DummyDriver(),
    createIntrospector: (db) => new SqliteIntrospector(db),
    createQueryCompiler: () => new SqliteQueryCompiler(),
  },
});

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
    q
      .selectFrom("todos")
      .selectAll()
      .orderBy("done", "asc")
      .orderBy("updated", "desc")
      .compile().sql,
  );

  return rows.map(mapRow);
}

export async function add(description: string): Promise<Todo[]> {
  const now = Date.now();
  const d = await db();

  const query = q
    .insertInto("todos")
    .values({ description, done: 0, updated: now, created: now })
    .compile();
  await d.execute(query.sql, [...query.parameters]);

  return getAll();
}

export async function remove(id: number): Promise<Todo[]> {
  const d = await db();
  const query = q.deleteFrom("todos").where("id", "=", id).compile();
  await d.execute(query.sql, [...query.parameters]);
  return getAll();
}

export async function update(id: number, done: boolean): Promise<Todo[]> {
  const now = Date.now();
  const d = await db();
  const query = q
    .updateTable("todos")
    .set({ done: Number(done), updated: now })
    .where("id", "=", id)
    .compile();
  await d.execute(query.sql, [...query.parameters]);

  return getAll();
}
