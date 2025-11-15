<script lang="ts">
  import TodoLi from "./components/TodoLi.svelte";
  import {
    add as addTodo,
    getAll,
    remove as removeTodo,
  } from "./lib/api/todos";
  import todosStore from "./stores/todos.svelte";
  let value = $state("");
  const initPromise = getAll();
  initPromise.then((r) => todosStore.replace(r));
  async function add(e: SubmitEvent) {
    e.preventDefault();
    if (value === "") return;

    const res = await addTodo(value);
    todosStore.replace(res);
    value = "";
  }

  async function onRemove(idx: number) {
    const res = await removeTodo(idx);
    todosStore.replace(res);
  }
</script>

<main class="f1 f cc">
  {#await initPromise}
    <h3>Loading...</h3>
  {:then _}
    <div class="f1 f c w100">
      <ul class="w100 f c">
        {#each todosStore.todos as todo (todo.id)}
          <TodoLi {todo} {onRemove} />
        {/each}
      </ul>
    </div>
    <form class="w100 f pd g" onsubmit={add}>
      <input class="f1" bind:value />
      <button>Add</button>
    </form>
  {/await}
</main>

<style>
  button {
    background-color: var(--main-bg-faint-color);
  }
  button:hover {
    border-color: var(--accent-color);
    transition: border-color 0.25s;
  }
</style>
