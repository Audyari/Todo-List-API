<template>
  <div class="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/40 backdrop-blur-sm">
    <!-- Modal Container -->
    <div class="relative w-full max-w-[600px] transform overflow-hidden rounded-xl bg-white dark:bg-[#1a2632] shadow-2xl transition-all border border-[#dbe0e6] dark:border-gray-700">
      <!-- Header Section -->
      <div class="flex items-center justify-between px-6 pt-6 pb-2">
        <div class="flex flex-col">
          <h2 class="text-[#111418] dark:text-white tracking-tight text-[28px] font-bold leading-tight text-left">
            Edit Task
          </h2>
          <p class="text-[#617589] dark:text-gray-400 text-sm font-normal leading-normal pt-1">
            Editing item: <span class="font-medium text-primary">#Task-{{ task.id }}</span>
          </p>
        </div>
        <!-- Close Icon Button -->
        <button 
          @click="closeModal"
          class="text-[#617589] dark:text-gray-400 hover:text-[#111418] dark:hover:text-white transition-colors cursor-pointer p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-800">
          <span class="material-symbols-outlined">close</span>
        </button>
      </div>
      
      <!-- Divider -->
      <div class="h-px bg-[#dbe0e6] dark:bg-gray-700 mx-6 my-2"></div>
      
      <!-- Form Content -->
      <div class="px-6 py-4 flex flex-col gap-5">
        <!-- Title Field -->
        <label class="flex flex-col w-full group">
          <p class="text-[#111418] dark:text-gray-100 text-base font-medium leading-normal pb-2 flex items-center gap-2">
            <span class="material-symbols-outlined text-[20px] text-gray-400">title</span>
            Task Title
          </p>
          <input 
            v-model="taskData.title"
            autofocus
            class="form-input flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white dark:bg-[#23303d] focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-gray-600 bg-white focus:border-primary h-12 placeholder:text-[#617589] px-[15px] text-base font-normal leading-normal transition-shadow" 
            type="text"
          />
        </label>
        
        <!-- Description Field -->
        <label class="flex flex-col w-full group">
          <p class="text-[#111418] dark:text-gray-100 text-base font-medium leading-normal pb-2 flex items-center gap-2">
            <span class="material-symbols-outlined text-[20px] text-gray-400">description</span>
            Description
          </p>
          <textarea 
            v-model="taskData.description"
            class="form-textarea flex w-full min-w-0 resize-none overflow-hidden rounded-lg text-[#111418] dark:text-white dark:bg-[#23303d] focus:outline-0 focus:ring-2 focus:ring-primary/50 border border-[#dbe0e6] dark:border-gray-600 bg-white focus:border-primary min-h-[160px] placeholder:text-[#617589] p-[15px] text-base font-normal leading-normal transition-shadow">
          </textarea>
        </label>
        
        <!-- Additional Meta Options -->
        <div class="flex gap-4">
          <label class="flex flex-col w-1/2">
            <p class="text-[#111418] dark:text-gray-100 text-sm font-medium leading-normal pb-2">Due Date</p>
            <div class="relative">
              <input 
                v-model="taskData.dueDate"
                class="w-full rounded-lg border border-[#dbe0e6] dark:border-gray-600 bg-white dark:bg-[#23303d] text-[#111418] dark:text-white h-10 px-3 text-sm focus:border-primary focus:ring-1 focus:ring-primary" 
                type="date"
              />
            </div>
          </label>
          <label class="flex flex-col w-1/2">
            <p class="text-[#111418] dark:text-gray-100 text-sm font-medium leading-normal pb-2">Priority</p>
            <select 
              v-model="taskData.priority"
              class="w-full rounded-lg border border-[#dbe0e6] dark:border-gray-600 bg-white dark:bg-[#23303d] text-[#111418] dark:text-white h-10 px-3 text-sm focus:border-primary focus:ring-1 focus:ring-primary">
              <option value="low">Low</option>
              <option value="normal">Normal</option>
              <option value="high">High</option>
            </select>
          </label>
        </div>
      </div>
      
      <!-- Error message display -->
      <div v-if="errorMessage" class="px-6 pt-2 pb-1 text-red-500 text-sm">
        {{ errorMessage }}
      </div>

      <!-- Footer Actions -->
      <div class="bg-gray-50 dark:bg-[#151f28] px-6 py-4 flex items-center justify-between rounded-b-xl border-t border-[#dbe0e6] dark:border-gray-700">
        <!-- Delete Action (Secondary/Destructive) -->
        <button
          @click="deleteTask"
          :disabled="loading"
          class="flex items-center justify-center gap-2 text-red-500 hover:text-red-700 text-sm font-medium px-2 py-2 rounded-lg hover:bg-red-50 dark:hover:bg-red-900/20 transition-colors disabled:opacity-50 disabled:cursor-not-allowed">
          <span v-if="loading" class="material-symbols-outlined text-[18px]">sync</span>
          <span v-else class="material-symbols-outlined text-[18px]">delete</span>
          <span class="hidden sm:inline">{{ loading ? 'Deleting...' : 'Delete Task' }}</span>
        </button>

        <!-- Primary Actions -->
        <div class="flex gap-3">
          <button
            @click="closeModal"
            :disabled="loading"
            class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-white dark:bg-transparent border border-[#dbe0e6] dark:border-gray-600 text-[#111418] dark:text-white hover:bg-gray-50 dark:hover:bg-gray-800 text-sm font-bold leading-normal tracking-[0.015em] transition-colors shadow-sm disabled:opacity-50 disabled:cursor-not-allowed">
            <span class="truncate">Cancel</span>
          </button>
          <button
            @click="saveTask"
            :disabled="loading"
            class="flex min-w-[84px] cursor-pointer items-center justify-center overflow-hidden rounded-lg h-10 px-6 bg-primary hover:bg-blue-600 text-white text-sm font-bold leading-normal tracking-[0.015em] transition-colors shadow-sm disabled:opacity-50 disabled:cursor-not-allowed">
            <span v-if="loading" class="flex items-center">
              <svg class="animate-spin -ml-1 mr-2 h-4 w-4 text-white" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Saving...
            </span>
            <span v-else class="truncate">Save Changes</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, onMounted } from 'vue'
import api from '../services/api'

export default {
  name: 'EditTodoList',
  props: {
    task: {
      type: Object,
      required: true
    }
  },
  emits: ['close', 'save', 'delete'],
  setup(props, { emit }) {
    // Initialize taskData with the task prop values
    const taskData = ref({
      id: props.task.id,
      title: props.task.title,
      description: props.task.description || '',
      dueDate: props.task.dueDate ? (typeof props.task.dueDate === 'string' ? props.task.dueDate : props.task.dueDate.toISOString().split('T')[0]) : '',
      priority: props.task.priority || 'normal',
      completed: props.task.completed || false
    })

    const loading = ref(false)
    const errorMessage = ref('')

    const closeModal = () => {
      emit('close')
    }

    const saveTask = async () => {
      if (taskData.value.title.trim() && !loading.value) {
        loading.value = true
        errorMessage.value = ''

        try {
          // Prepare task data for API request
          const taskPayload = {
            title: taskData.value.title,
            description: taskData.value.description || null,
            completed: taskData.value.completed
          }

          // Use the MongoDB ObjectId from the original task - handle different formats
          let taskId = props.task.id;
          if (typeof taskId === 'object' && taskId.$oid) {
            taskId = taskId.$oid;
          } else if (!taskId && typeof props.task._id === 'object' && props.task._id.$oid) {
            taskId = props.task._id.$oid;
          } else if (!taskId) {
            taskId = props.task._id;
          }

          console.log('Updating task with ID:', taskId, 'and payload:', taskPayload);

          // Make API call to update task
          const response = await api.taskAPI.updateTask(taskId, taskPayload)
          console.log('Task updated successfully:', response)

          // Emit the updated task data
          emit('save', {
            ...response.data, // Use the response from the backend
            title: taskData.value.title,
            description: taskData.value.description,
            dueDate: taskData.value.dueDate,
            priority: taskData.value.priority,
            completed: taskData.value.completed
          })
        } catch (error) {
          console.error('Error updating task:', error)
          if (error.message.includes('401')) {
            errorMessage.value = 'Authentication failed. Please log in again.'
          } else if (error.message.includes('400')) {
            errorMessage.value = 'Invalid task data. Please check your input.'
          } else if (error.message.includes('404')) {
            errorMessage.value = 'Task not found. It may have been deleted.'
          } else {
            errorMessage.value = error.message || 'Failed to update task. Please try again.'
          }
        } finally {
          loading.value = false
        }
      }
    }

    const deleteTask = async () => {
      if (!loading.value) {
        loading.value = true
        errorMessage.value = ''

        try {
          // Use the MongoDB ObjectId from the original task - handle different formats
          let taskId = props.task.id;
          if (typeof taskId === 'object' && taskId.$oid) {
            taskId = taskId.$oid;
          } else if (!taskId && typeof props.task._id === 'object' && props.task._id.$oid) {
            taskId = props.task._id.$oid;
          } else if (!taskId) {
            taskId = props.task._id;
          }

          // Make API call to delete task
          const response = await api.taskAPI.deleteTask(taskId)
          console.log('Task deleted successfully:', response)

          // Emit the delete event with the task ID
          emit('delete', taskId)
        } catch (error) {
          console.error('Error deleting task:', error)
          if (error.message.includes('401')) {
            errorMessage.value = 'Authentication failed. Please log in again.'
          } else if (error.message.includes('400')) {
            errorMessage.value = 'Invalid request. Please try again.'
          } else if (error.message.includes('404')) {
            errorMessage.value = 'Task not found. It may have been deleted already.'
          } else {
            errorMessage.value = error.message || 'Failed to delete task. Please try again.'
          }
        } finally {
          loading.value = false
        }
      }
    }

    return {
      taskData,
      loading,
      errorMessage,
      closeModal,
      saveTask,
      deleteTask
    }
  }
}
</script>