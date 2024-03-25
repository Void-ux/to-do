<script setup>
import CreateTask from "./components/CreateTask.vue";
import { ref, provide, onMounted, onUnmounted, nextTick } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

let taskLists = ref(null);
let currentTaskList = ref(null);
let tasks = ref("");

async function getTaskLists() {
  let lists = await invoke("get_task_lists");
  taskLists.value = lists;
  console.log(`Found ${lists.length} task lists (${lists})`)
}

async function getTasks() {
  if (currentTaskList.value !== null) {
    tasks.value = await invoke("get_tasks", { taskListId: currentTaskList.value[0] });    
  } else {
    tasks.value = [];
  }
};

async function setCurrentTaskList(taskList) {
  currentTaskList.value = taskList;
  await getTasks();
}

async function createTaskList() {
  await invoke("create_task_list")
  await getTaskLists();
}

async function deleteTask(id) {
  await invoke("delete_task", {"id": id});
  await getTasks();
};

getTaskLists();
// When no task lists exist
currentTaskList.value = currentTaskList.value !== null ? taskLists.value[0] : null;
provide('currentTaskList', currentTaskList)

getTasks();
// Provide the function to the CreateTask component to
// refresh the task list when a new task is submitted
provide('getTasks', getTasks);

const day = ref(null);
const date = ref(null);
const month = ref(null);

let nIntervId;
function setTime() {
  let now = new Date();
  day.value = ["MON", "TUE", "WED", "THU", "FRI", "SAT", "SUN"][now.getDay()];
  date.value = now.getDate();
  month.value = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"][now.getMonth()];
}

onMounted(() => {
  if (!nIntervId) {
    nIntervId = setInterval(setTime, 1000);
  }
})
onUnmounted(() => {
  clearInterval(nIntervId);
  nIntervId = null;
})
</script>

<template>
  <div class="flex">
    <div class="h-screen flex flex-col gap-2 text-left w-48 mx-4">
      <div class="flex flex-col gap-1.5 mt-20">
        <div class="flex justify-between">
          <p class="text-xl font-bold">My Lists</p>
          <div class="ml-7 rotate-90 cursor-pointer" @click="createTaskList()">
            <div class="absolute w-1 h-4 bg-[#8a8a8a] rounded-3xl"></div>
            <div class="absolute w-1 h-4 bg-[#8a8a8a] rounded-3xl rotate-90"></div>
          </div>
        </div>
        <div 
          class="text-lg font-medium cursor-pointer"
          v-for="taskList in taskLists"
          @click="setCurrentTaskList(taskList)"
        >
          {{ taskList[1] }}
        </div>
      </div>
    </div>

    <div class="flex flex-col text-center items-center gap-2 pt-5 w-full">
      <div class="flex items-center mb-3">
        <div class="flex flex-col text-center">
          <p class="text-stone-500 text-lg font-extrabold leading-6">{{ day }}</p>
          <p class="text-2xl font-bold leading-6">{{ date }}</p>
          <p class="text-zinc-500 text-lg font-medium leading-5">{{ month }}</p>
        </div>
        <p class="text-2xl font-bold">Good Afternoon, MS1</p>
      </div>

      <div
        v-if="currentTaskList && tasks !== null"
        v-for="[id, title, description] of tasks"
        class="flex w-5/6 h-20 rounded-lg p-2 text-left hover:brightness-95 partially-inset-shadow justify-between"
      >
        <div class="flex flex-col justify-evenly h-full ml-3">
          <div class="flex gap-1">
            <img src="./assets/lock.svg" class=w-3>
            <span class="text-[#7A7A7A] text-sm">My Lists > {{ currentTaskList[1] }}</span>
          </div>
          <p class="text-xl font-medium">{{ title }}</p>
        </div>
        <img src="./assets/plus.svg" class="float-right w-6 cursor-pointer rotate-45" @click="deleteTask(id)">
      </div>

      <CreateTask />
    </div>
  </div>
</template>

<style>
@import "https://fonts.googleapis.com/css?family=Montserrat:200,300&display=swap";

.gradiented-background {
  margin: 0px;
  font-weight: 900;
  background: linear-gradient(45deg, #EE7752, #E73C7E, #23A6D5, #23D5AB);
  background-size: 400%;
  -webkit-animation: gradient-shift 5s ease alternate infinite;
  animation: gradient-shift 5s ease alternate infinite;
  background-attachment: fixed;
  background-repeat: no-repeat;
  -webkit-text-fill-color: transparent;
}

.partially-inset-shadow {
  box-shadow: 0px 6px 6px 0px rgba(0, 0, 0, 0.05) inset, 0px 6px 6px 0px rgba(0, 0, 0, 0.05);
}

@-webkit-keyframes gradient-shift {
  0% {
    background-position: right;
  }

  100% {
    background-position: left;
  }
}

@keyframes gradient-shift {
  0% {
    background-position: right;
  }

  100% {
    background-position: left;
  }
}
</style>
