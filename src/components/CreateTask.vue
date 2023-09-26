<script setup>
import { ref, inject } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const getTasks = inject('get-tasks')

const newTask = ref("");
const title = ref("");

async function createNewTask() {
  if (title.value === "") {
    return
  }
  // Invoke the command with the task content to update the JSON file
  newTask.value = await invoke("new_task", { title: title.value });
  title.value = "";
  await getTasks();
};

</script>

<template>
  <div class="fixed inset-x-0 bottom-10">
    <input ref="taskInput" id="new-task-input" v-model="title" placeholder="Enter a new task..." @keyup.enter="createNewTask()" />
    <button class=" " type="button" @click="createNewTask()" :disabled="title === ''" :class="{ 'opacity-40': title == '' }">Create Task</button>
  </div>
</template>

<style>
.faded {
  opacity: 80;
}
</style>
