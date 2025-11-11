import type { Todo } from "../lib/models/Todo";

class Todos {
  #todos: Todo[] = $state([]);

  get todos() {
    return this.#todos;
  }

  replace(todos: Todo[]) {
    this.#todos = todos;
  }
}
const todos = new Todos();
export default todos;
