import type { Generated } from "kysely";

export type Todo = {
  id: number;
  description: string;
  done: boolean;
  created: Date;
  updated: Date;
};

export type TodoTable = {
  id: Generated<number>;
  description: string;
  done: number;
  created: number;
  updated: number;
};
