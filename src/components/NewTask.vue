<script setup>
import { ref, inject } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const getTasks = inject('get-tasks')

const newTask = ref("");
const name = ref("");

async function createNewTask() {
  // Invoke the command with the task content to update the JSON file
  newTask.value = await invoke("new_task", { name: name.value });
  await getTasks();
};

</script>

<template>
  
  <div class="fixed inset-x-0 bottom-10">
    <input ref="taskInput" id="new-task-input" v-model="name" placeholder="Enter a new task..." />
    <button type="button" @click="createNewTask(); this.$refs.taskInput.value='';">Create Task</button>
  </div>

</template>
