<script lang="ts">
  import { update } from "../lib/api/todos";
  import type { Todo } from "../lib/models/Todo";
  import todosStore from "../stores/todos.svelte";

  type Props = {
    todo: Todo;
    onRemove: (idx: number) => void;
  };

  let { todo, onRemove }: Props = $props();
  let done = $state(todo.done);

  function remove() {
    onRemove(todo.id);
  }

  async function mark() {
    const result = await update(todo.id, done);
    todosStore.replace(result);
  }
</script>

<li>
  <span class:done>
    {todo.description}
  </span>
  <div class="f g r aic">
    <input type="checkbox" bind:checked={done} onchange={mark} />
    <button class="n-btn" onclick={remove}> 🗑️ </button>
  </div>
</li>

<style>
  li {
    font-size: 1.3rem;
    width: 100%;
    padding: 1rem 1.5rem;
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;
    border-bottom: var(--borders);
  }

  li:hover {
    background-color: var(--main-bg-faint-color);
  }

  .done {
    text-decoration: line-through;
  }
</style>
