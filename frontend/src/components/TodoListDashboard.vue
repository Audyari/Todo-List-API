<template>
  <div class="flex h-screen w-full overflow-hidden bg-background-light dark:bg-background-dark text-[#111418] dark:text-white font-display">
    <!-- Sidebar -->
    <aside class="hidden md:flex flex-col w-72 bg-white dark:bg-[#1a2632] border-r border-[#f0f2f4] dark:border-slate-800 h-full flex-shrink-0">
      <!-- Logo Area -->
      <div class="p-6 flex items-center gap-3">
        <div class="size-8 bg-primary rounded-lg flex items-center justify-center text-white">
          <span class="material-symbols-outlined text-[20px]">check_box</span>
        </div>
        <h2 class="text-xl font-bold tracking-tight text-[#111418] dark:text-white">TaskMaster</h2>
      </div>

      <!-- User Profile -->
      <div class="px-4 pb-6 border-b border-[#f0f2f4] dark:border-slate-800 mx-2">
        <div class="flex items-center gap-3 p-3 rounded-xl bg-background-light dark:bg-background-dark">
          <div class="bg-center bg-no-repeat bg-cover rounded-full size-10 shrink-0"
               data-alt="User profile picture of a smiling man"
               style='background-image: url("https://lh3.googleusercontent.com/aida-public/AB6AXuB49siYi_nu1zy2fvwwZL0sZ6Q3w99p0DYMKGgEcW7_NnYZHjNIAJB515o8ZovvacXNL19GO3Bh-h_hxgMpgkXX55txbTfV_97PAR_-UKOobYofellkaFwoeXC9DAWVFa6erOiov3DVr8Y_WV5CZw8eV6kxPfdik_CKkCPu-sQilIcVrq_P_kjhPqtN25akGw5p0zGOGBL9J9_6a19lKf-NHqxgVcqbP1mD5K75uGJlING6Y0rxKxXreN6QADuQ5AqlRQ8n7nV6aA");'></div>
          <div class="flex flex-col min-w-0">
            <h1 class="text-sm font-bold truncate dark:text-white">John Doe</h1>
            <p class="text-xs text-[#617589] dark:text-slate-400 truncate">john.doe@example.com</p>
          </div>
        </div>
      </div>

      <!-- Navigation Links -->
      <nav class="flex-1 overflow-y-auto px-4 py-4 flex flex-col gap-2">
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg bg-primary/10 text-primary dark:bg-primary/20" href="#">
          <span class="material-symbols-outlined fill-1">check_circle</span>
          <span class="text-sm font-medium">My Tasks</span>
          <span class="ml-auto bg-primary text-white text-xs font-bold px-2 py-0.5 rounded-full">{{ tasks.filter(t => !t.completed).length }}</span>
        </a>
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-[#617589] dark:text-slate-300 hover:bg-[#f0f2f4] dark:hover:bg-slate-700 transition-colors" href="#">
          <span class="material-symbols-outlined">star</span>
          <span class="text-sm font-medium">Important</span>
        </a>
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-[#617589] dark:text-slate-300 hover:bg-[#f0f2f4] dark:hover:bg-slate-700 transition-colors" href="#">
          <span class="material-symbols-outlined">calendar_today</span>
          <span class="text-sm font-medium">Planned</span>
        </a>
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-[#617589] dark:text-slate-300 hover:bg-[#f0f2f4] dark:hover:bg-slate-700 transition-colors" href="#">
          <span class="material-symbols-outlined">person</span>
          <span class="text-sm font-medium">Assigned to me</span>
        </a>
        <div class="mt-6 mb-2 px-3">
          <p class="text-xs font-semibold text-[#617589] dark:text-slate-500 uppercase tracking-wider">Projects</p>
        </div>
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-[#617589] dark:text-slate-300 hover:bg-[#f0f2f4] dark:hover:bg-slate-700 transition-colors" href="#">
          <span class="material-symbols-outlined text-purple-500">folder</span>
          <span class="text-sm font-medium">Website Redesign</span>
        </a>
        <a class="flex items-center gap-3 px-3 py-2.5 rounded-lg text-[#617589] dark:text-slate-300 hover:bg-[#f0f2f4] dark:hover:bg-slate-700 transition-colors" href="#">
          <span class="material-symbols-outlined text-orange-500">folder</span>
          <span class="text-sm font-medium">Q4 Marketing</span>
        </a>
      </nav>

      <!-- Bottom Action -->
      <div class="p-4 border-t border-[#f0f2f4] dark:border-slate-800">
        <button class="flex w-full items-center justify-center gap-2 rounded-lg h-10 px-4 bg-background-light dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 text-[#111418] dark:text-white text-sm font-bold transition-colors">
          <span class="material-symbols-outlined text-[18px]">settings</span>
          <span>Settings</span>
        </button>
      </div>
    </aside>

    <!-- Main Content Area -->
    <main class="flex-1 flex flex-col h-full overflow-hidden relative">
      <!-- Top Header (Search & Actions) -->
      <header class="flex items-center justify-between px-6 py-4 bg-white dark:bg-[#1a2632] border-b border-[#f0f2f4] dark:border-slate-800 shrink-0">
        <div class="flex items-center gap-4 flex-1">
          <!-- Mobile Menu Button -->
          <button class="md:hidden p-2 text-[#617589] dark:text-white">
            <span class="material-symbols-outlined">menu</span>
          </button>
          <!-- Search Bar -->
          <label class="flex items-center max-w-md w-full h-10 bg-background-light dark:bg-background-dark rounded-lg px-3 focus-within:ring-2 focus-within:ring-primary/50 transition-shadow">
            <span class="material-symbols-outlined text-[#617589] text-[20px]">search</span>
            <input class="w-full bg-transparent border-none focus:ring-0 text-sm px-3 text-[#111418] dark:text-white placeholder:text-[#617589]" placeholder="Search tasks..." type="text"/>
          </label>
        </div>
        <div class="flex items-center gap-4">
          <button class="size-10 flex items-center justify-center rounded-full hover:bg-background-light dark:hover:bg-slate-700 text-[#617589] dark:text-slate-300 transition-colors relative">
            <span class="material-symbols-outlined text-[24px]">notifications</span>
            <span class="absolute top-2 right-2 size-2.5 bg-red-500 border-2 border-white dark:border-[#1a2632] rounded-full"></span>
          </button>
          <button
            @click="showCreateModal = true"
            class="hidden sm:flex h-10 items-center justify-center rounded-lg bg-primary hover:bg-blue-600 text-white px-4 text-sm font-bold gap-2 shadow-md hover:shadow-lg transition-all">
            <span class="material-symbols-outlined text-[20px]">add</span>
            <span>New Task</span>
          </button>
        </div>
      </header>

      <!-- Scrollable Content -->
      <div class="flex-1 overflow-y-auto p-4 md:p-8">
        <div class="max-w-5xl mx-auto flex flex-col gap-6">
          <!-- Page Heading -->
          <div class="flex flex-col md:flex-row md:items-end justify-between gap-4">
            <div>
              <h1 class="text-3xl md:text-4xl font-black tracking-tight text-[#111418] dark:text-white mb-2">My Tasks</h1>
              <p class="text-[#617589] dark:text-slate-400">Keep track of your daily priorities and manage your workflow.</p>
            </div>
            <!-- FAB for Mobile -->
            <button
              @click="showCreateModal = true"
              class="md:hidden fixed bottom-6 right-6 size-14 rounded-full bg-primary text-white shadow-xl flex items-center justify-center z-50">
              <span class="material-symbols-outlined text-[28px]">add</span>
            </button>
          </div>

          <!-- Filters and Toolbar -->
          <div class="flex flex-col sm:flex-row gap-4 items-start sm:items-center justify-between bg-white dark:bg-[#1a2632] p-2 rounded-xl shadow-sm border border-[#f0f2f4] dark:border-slate-800">
            <!-- Chips -->
            <div class="flex flex-wrap gap-2 px-2">
              <button class="flex items-center gap-2 h-8 px-3 rounded-lg bg-[#f0f2f4] dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">
                <span class="text-sm font-medium text-[#111418] dark:text-white">All Statuses</span>
                <span class="material-symbols-outlined text-[16px]">expand_more</span>
              </button>
              <button class="flex items-center gap-2 h-8 px-3 rounded-lg bg-[#f0f2f4] dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">
                <span class="text-sm font-medium text-[#111418] dark:text-white">High Priority</span>
                <span class="material-symbols-outlined text-[16px]">expand_more</span>
              </button>
              <button class="flex items-center gap-2 h-8 px-3 rounded-lg bg-[#f0f2f4] dark:bg-slate-700 hover:bg-slate-200 dark:hover:bg-slate-600 transition-colors">
                <span class="text-sm font-medium text-[#111418] dark:text-white">Due Today</span>
              </button>
            </div>
            <!-- View Options -->
            <div class="flex items-center gap-1 pr-2 ml-auto sm:ml-0">
              <button class="p-2 text-[#617589] hover:text-primary dark:text-slate-400 dark:hover:text-white transition-colors" title="Filter">
                <span class="material-symbols-outlined">filter_list</span>
              </button>
              <button class="p-2 text-[#617589] hover:text-primary dark:text-slate-400 dark:hover:text-white transition-colors" title="Sort">
                <span class="material-symbols-outlined">sort</span>
              </button>
            </div>
          </div>

          <!-- Error message display -->
          <div v-if="errorMessage" class="mb-4 p-3 bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg text-red-700 dark:text-red-300 text-sm">
            {{ errorMessage }}
          </div>

          <!-- Loading indicator -->
          <div v-if="loading" class="flex items-center justify-center py-12">
            <div class="flex flex-col items-center">
              <svg class="animate-spin h-8 w-8 text-primary" xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              <p class="mt-2 text-sm text-[#617589] dark:text-slate-400">Loading tasks...</p>
            </div>
          </div>

          <!-- Task List Container -->
          <div v-else class="flex flex-col bg-white dark:bg-[#1a2632] rounded-xl shadow-sm border border-[#f0f2f4] dark:border-slate-800 overflow-hidden">
            <!-- Header Row (Desktop) -->
            <div class="hidden md:flex items-center bg-background-light dark:bg-slate-800/50 px-6 py-3 border-b border-[#f0f2f4] dark:border-slate-800">
              <div class="w-8 shrink-0"></div>
              <div class="flex-1 text-xs font-bold text-[#617589] dark:text-slate-400 uppercase tracking-wider">Task Description</div>
              <div class="w-32 text-xs font-bold text-[#617589] dark:text-slate-400 uppercase tracking-wider">Status</div>
              <div class="w-32 text-xs font-bold text-[#617589] dark:text-slate-400 uppercase tracking-wider">Due Date</div>
              <div class="w-20 text-xs font-bold text-[#617589] dark:text-slate-400 uppercase tracking-wider text-right">Actions</div>
            </div>

            <!-- Task Items -->
            <div v-for="(task, index) in tasks" :key="task.id" :class="[
              'group flex flex-col md:flex-row md:items-center gap-4 p-4 md:px-6 md:py-4 border-b border-[#f0f2f4] dark:border-slate-800 hover:bg-[#f8f9fa] dark:hover:bg-slate-800/30 transition-colors',
              {'bg-gray-50/50 dark:bg-slate-800/20': index % 2 === 0}
            ]">
              <div class="flex items-start gap-4 flex-1">
                <div class="pt-1 md:pt-0">
                  <input
                    v-model="task.completed"
                    @change="updateTaskStatus(task)"
                    class="size-5 rounded border-gray-300 text-primary focus:ring-primary cursor-pointer"
                    type="checkbox"
                  />
                </div>
                <div class="flex flex-col gap-1" :class="{'opacity-60': task.completed}">
                  <div class="flex items-center gap-2">
                    <h3 class="text-base font-semibold text-[#111418] dark:text-white leading-tight" :class="{'line-through': task.completed}">{{ task.title }}</h3>
                    <span class="md:hidden px-2 py-0.5 rounded text-xs font-medium" :class="getStatusClass(task.status, 'mobile')">{{ task.status }}</span>
                  </div>
                  <p class="text-sm text-[#617589] dark:text-slate-400 line-clamp-1">{{ task.description }}</p>
                </div>
              </div>
              <div class="flex items-center justify-between md:justify-end gap-6 w-full md:w-auto pl-9 md:pl-0" :class="{'opacity-60': task.completed}">
                <div class="hidden md:block w-32">
                  <span class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium" :class="getStatusClass(task.status, 'desktop')">
                    {{ task.status }}
                  </span>
                </div>
                <div class="flex items-center gap-2 w-32 text-sm" :class="{'text-red-500 font-medium': isToday(task.dueDate), 'text-[#617589] dark:text-slate-400': !isToday(task.dueDate)}">
                  <span class="material-symbols-outlined text-[18px]">{{ task.completed ? 'event_available' : 'event' }}</span>
                  <span>{{ formatDate(task.dueDate) }}</span>
                </div>
                <div class="flex items-center justify-end gap-1 w-20 opacity-100 md:opacity-0 group-hover:opacity-100 transition-opacity">
                  <button class="p-1.5 text-[#617589] hover:text-primary rounded-md hover:bg-gray-100 dark:hover:bg-slate-700 dark:text-slate-400 transition-colors" title="Edit" @click="openEditModal(task)">
                    <span class="material-symbols-outlined text-[20px]">edit</span>
                  </button>
                  <button class="p-1.5 text-[#617589] hover:text-red-500 rounded-md hover:bg-gray-100 dark:hover:bg-slate-700 dark:text-slate-400 transition-colors" title="Delete" @click="deleteTask(task.id)">
                    <span class="material-symbols-outlined text-[20px]">delete</span>
                  </button>
                </div>
              </div>
            </div>

            <!-- Pagination -->
            <div class="flex flex-col sm:flex-row items-center justify-between px-6 py-4 gap-4">
              <p class="text-sm text-[#617589] dark:text-slate-400">Showing <span class="font-bold text-[#111418] dark:text-white">1-{{ tasks.length }}</span> of <span class="font-bold text-[#111418] dark:text-white">{{ tasks.length }}</span> tasks</p>
              <div class="flex items-center gap-2">
                <button class="flex items-center justify-center size-9 rounded-lg border border-[#f0f2f4] dark:border-slate-700 text-[#617589] hover:bg-[#f0f2f4] dark:hover:bg-slate-700 disabled:opacity-50 transition-colors">
                  <span class="material-symbols-outlined text-[20px]">chevron_left</span>
                </button>
                <button class="flex items-center justify-center size-9 rounded-lg bg-primary text-white font-bold text-sm">1</button>
                <button class="flex items-center justify-center size-9 rounded-lg border border-[#f0f2f4] dark:border-slate-700 text-[#617589] hover:bg-[#f0f2f4] dark:text-slate-400 dark:hover:bg-slate-700 transition-colors font-medium text-sm">2</button>
                <button class="flex items-center justify-center size-9 rounded-lg border border-[#f0f2f4] dark:border-slate-700 text-[#617589] hover:bg-[#f0f2f4] dark:text-slate-400 dark:hover:bg-slate-700 transition-colors font-medium text-sm">3</button>
                <span class="text-[#617589] px-1">...</span>
                <button class="flex items-center justify-center size-9 rounded-lg border border-[#f0f2f4] dark:border-slate-700 text-[#617589] hover:bg-[#f0f2f4] dark:text-slate-400 dark:hover:bg-slate-700 transition-colors">
                  <span class="material-symbols-outlined text-[20px]">chevron_right</span>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </main>

    <!-- Create Todo Modal -->
    <CreateTodoList
      v-if="showCreateModal"
      @close="showCreateModal = false"
      @save="addTask"
    />

    <!-- Edit Todo Modal -->
    <EditTodoList
      v-if="showEditModal && selectedTask"
      :task="selectedTask"
      @close="showEditModal = false"
      @save="updateTask"
      @delete="handleDeleteTask"
    />
  </div>
</template>

<script>
import CreateTodoList from './CreateTodoList.vue'
import EditTodoList from './EditTodoList.vue'
import api from '../services/api'

export default {
  name: 'TodoListDashboard',
  components: {
    CreateTodoList,
    EditTodoList
  },
  data() {
    return {
      // State Management (Data Center)
      showCreateModal: false,
      showEditModal: false,
      selectedTask: null,
      tasks: [], // Ini adalah "Single Source of Truth". Seluruh tampilan daftar tugas di HTML bergantung pada array ini. Jika tasks berubah, tampilan otomatis berubah.
      loading: false, // Loading state
      errorMessage: '' // Error message
    }
  },
  async mounted() {
    await this.loadTasks();
  },
  methods: {
    async loadTasks() {
      this.loading = true;
      this.errorMessage = '';

      try {
        // Fetch tasks from the backend
        const response = await api.taskAPI.getTasks();
        console.log('Tasks loaded:', response);

        // Transform the response to match the expected format
        // Check if response.data exists and is an array
        if (response.data && Array.isArray(response.data)) {
          this.tasks = response.data.map(task => {
            // Handle different ID formats from MongoDB
            const taskId = typeof task._id === 'object' && task._id.$oid ? task._id.$oid : task._id;

            return {
              _id: task._id, // Keep original format for internal use
              id: taskId, // Use string ID for compatibility
              title: task.title,
              description: task.description || '',
              status: task.completed ? 'Completed' : 'In Progress',
              dueDate: task.createdAt ? new Date(task.createdAt) : new Date(),
              completed: task.completed || false,
              priority: task.priority || 'normal' // Add priority if available
            };
          });
        } else {
          console.error('Invalid response format:', response);
          if (response.message) {
            this.errorMessage = response.message;
          }
          this.tasks = []; // Set to empty array if response is invalid
        }
      } catch (error) {
        console.error('Error loading tasks:', error);
        if (error.message.includes('401') || error.message.toLowerCase().includes('unauthorized') || error.message.toLowerCase().includes('invalid token')) {
          this.errorMessage = 'Authentication failed. Please log in again.';
        } else {
          this.errorMessage = error.message || 'Failed to load tasks. Please try again.';
        }
        this.tasks = []; // Ensure tasks is an empty array on error
      } finally {
        this.loading = false;
      }
    },
    getStatusClass(status, variant) {
      const baseClasses = variant === 'desktop' ? 'hidden md:block w-32' : 'md:hidden px-2 py-0.5 rounded text-xs font-medium';
      const statusClasses = {
        'In Progress': variant === 'desktop'
          ? 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300'
          : 'bg-blue-100 text-blue-700 dark:bg-blue-900/30 dark:text-blue-300',
        'Pending': variant === 'desktop'
          ? 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-300'
          : 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-300',
        'Completed': variant === 'desktop'
          ? 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300'
          : 'bg-green-100 text-green-700 dark:bg-green-900/30 dark:text-green-300',
        'Idea': variant === 'desktop'
          ? 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-300'
          : 'bg-purple-100 text-purple-700 dark:bg-purple-900/30 dark:text-purple-300',
        'Routine': variant === 'desktop'
          ? 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300'
          : 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300'
      };

      return `${baseClasses} ${statusClasses[status] || 'bg-gray-100 text-gray-700 dark:bg-gray-700 dark:text-gray-300'}`;
    },
    formatDate(date) {
      if (this.isToday(date)) {
        return 'Today';
      }
      return date.toLocaleDateString('en-US', { month: 'short', day: 'numeric', year: 'numeric' });
    },
    isToday(date) {
      const today = new Date();
      return date.getDate() === today.getDate() &&
        date.getMonth() === today.getMonth() &&
        date.getFullYear() === today.getFullYear();
    },
    async addTask(taskData) {
      try {
        // The task is already created via the API in CreateTodoList component
        // We just need to refresh the task list
        await this.loadTasks();
        this.showCreateModal = false;
      } catch (error) {
        console.error('Error adding task:', error);
        if (error.message.includes('401')) {
          this.errorMessage = 'Authentication failed. Please log in again.';
        } else {
          this.errorMessage = error.message || 'Failed to add task. Please try again.';
        }
      }
    },
    openEditModal(task) {
      this.selectedTask = { ...task }; // Create a copy to avoid direct mutation
      this.showEditModal = true;
    },
    async updateTask(updatedTask) {
      try {
        // The task is already updated via the API in EditTodoList component
        // We just need to refresh the task list
        await this.loadTasks();
        this.showEditModal = false;
        this.selectedTask = null;
      } catch (error) {
        console.error('Error updating task:', error);
        if (error.message.includes('401')) {
          this.errorMessage = 'Authentication failed. Please log in again.';
        } else {
          this.errorMessage = error.message || 'Failed to update task. Please try again.';
        }
      }
    },
    async deleteTask(taskId) {
      try {
        // The task is already deleted via the API in EditTodoList component
        // We just need to refresh the task list
        await this.loadTasks();
      } catch (error) {
        console.error('Error deleting task:', error);
        if (error.message.includes('401')) {
          this.errorMessage = 'Authentication failed. Please log in again.';
        } else if (error.message.includes('404')) {
          this.errorMessage = 'Task not found. It may have been deleted already.';
        } else {
          this.errorMessage = error.message || 'Failed to delete task. Please try again.';
        }
      }
    },
    async handleDeleteTask(taskId) {
      await this.deleteTask(taskId);
      this.showEditModal = false;
      this.selectedTask = null;
    },
    async updateTaskStatus(task) {
      try {
        // Update the task status via API
        const taskPayload = {
          completed: task.completed
        };

        // Use the MongoDB ObjectId - handle different formats
        let taskId = task._id;
        if (typeof task._id === 'object' && task._id.$oid) {
          taskId = task._id.$oid;
        } else if (!taskId) {
          taskId = task.id;
        }

        // Call updateTask with only the completed status, without userId
        // The backend should handle authorization via the JWT token
        const response = await api.taskAPI.updateTask(taskId, taskPayload);
        console.log('Task status updated:', response);

        // Refresh the entire task list to ensure consistency
        await this.loadTasks();
      } catch (error) {
        console.error('Error updating task status:', error);
        // Revert the checkbox state if the API call fails
        task.completed = !task.completed;
        if (error.message.includes('401')) {
          this.errorMessage = 'Authentication failed. Please log in again.';
        } else if (error.message.includes('400')) {
          this.errorMessage = 'Invalid request. Please try again.';
        } else if (error.message.includes('404')) {
          this.errorMessage = 'Task not found. It may have been deleted.';
        } else {
          this.errorMessage = error.message || 'Failed to update task status. Please try again.';
        }
      }
    },
    getStatusFromPriority(priority) {
      const statusMap = {
        'high': 'High Priority',
        'normal': 'In Progress',
        'low': 'Pending'
      };

      return statusMap[priority] || 'In Progress';
    }
  }
}
</script>

<style scoped>
/* Custom scrollbar for better aesthetics */
::-webkit-scrollbar {
  width: 8px;
  height: 8px;
}
::-webkit-scrollbar-track {
  background: transparent;
}
::-webkit-scrollbar-thumb {
  background: #cbd5e1;
  border-radius: 4px;
}
::-webkit-scrollbar-thumb:hover {
  background: #94a3b8;
}
.dark ::-webkit-scrollbar-thumb {
  background: #334155;
}
.dark ::-webkit-scrollbar-thumb:hover {
  background: #475569;
}

/* Material Symbols font */
.material-symbols-outlined {
  font-family: 'Material Symbols Outlined';
  font-weight: normal;
  font-style: normal;
  font-size: 24px;
  line-height: 1;
  letter-spacing: normal;
  text-transform: none;
  display: inline-block;
  white-space: nowrap;
  word-wrap: normal;
  direction: ltr;
  -webkit-font-feature-settings: 'liga';
  font-feature-settings: 'liga';
  -webkit-font-smoothing: antialiased;
}
</style>