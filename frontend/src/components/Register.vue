<script setup>
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const formData = ref({
  fullName: '',
  email: '',
  password: ''
})

const showPassword = ref(false)

const handleSubmit = async (e) => {
  e.preventDefault()
  console.log('Registration data:', formData.value)

  // In a real application, you would make an API call to register the user
  // For now, we'll simulate a successful registration
  try {
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 500))

    // On successful registration, redirect to login
    router.push('/login')
  } catch (error) {
    console.error('Registration failed:', error)
    // Handle registration error (show message to user, etc.)
  }
}
</script>

<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col font-display">
    <!-- Header -->
    <header class="w-full border-b border-gray-200 dark:border-gray-700 bg-white dark:bg-gray-900">
      <div class="flex items-center justify-between px-6 py-4 max-w-7xl mx-auto">
        <div class="flex items-center gap-2 text-gray-900 dark:text-white">
          <div class="size-8 text-blue-600">
            <span class="material-symbols-outlined" style="font-size: 32px;">check_circle</span>
          </div>
          <h2 class="text-lg font-bold leading-tight tracking-tight">ToDo App</h2>
        </div>
        <router-link to="/login" class="flex min-w-[84px] cursor-pointer items-center justify-center rounded-lg h-9 px-4 bg-gray-200 dark:bg-gray-700 text-gray-900 dark:text-white text-sm font-bold hover:bg-gray-300 dark:hover:bg-gray-600 transition-colors">
          <span class="truncate">Log In</span>
        </router-link>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-grow flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8">
      <div class="w-full max-w-md bg-white dark:bg-gray-800 shadow-sm rounded-xl border border-gray-200 dark:border-gray-700 overflow-hidden">
        <div class="px-8 pt-8 pb-6 text-center">
          <div class="mx-auto w-12 h-12 bg-blue-100 dark:bg-blue-900/30 rounded-full flex items-center justify-center mb-4">
            <span class="material-symbols-outlined text-blue-600 dark:text-blue-400" style="font-size: 24px;">person_add</span>
          </div>
          <h1 class="text-gray-900 dark:text-white tracking-tight text-2xl font-bold leading-tight">Create your account</h1>
          <p class="text-gray-600 dark:text-gray-400 text-sm mt-2">Start organizing your tasks efficiently today.</p>
        </div>

        <form @submit="handleSubmit" class="px-8 pb-8 space-y-5">
          <div>
            <label class="block text-sm font-medium text-gray-900 dark:text-white mb-1.5" for="full-name">Full Name</label>
            <div class="relative">
              <input
                v-model="formData.fullName"
                class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:border-blue-600 focus:ring-1 focus:ring-blue-600 py-3 px-3.5 text-base sm:text-sm"
                id="full-name"
                name="name"
                placeholder="John Doe"
                required
                type="text"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-900 dark:text-white mb-1.5" for="email-address">Email Address</label>
            <div class="relative">
              <input
                v-model="formData.email"
                autocomplete="email"
                class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:border-blue-600 focus:ring-1 focus:ring-blue-600 py-3 px-3.5 text-base sm:text-sm"
                id="email-address"
                name="email"
                placeholder="name@company.com"
                required
                type="email"
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-gray-900 dark:text-white mb-1.5" for="password">Password</label>
            <div class="relative group">
              <input
                v-model="formData.password"
                autocomplete="new-password"
                :type="showPassword ? 'text' : 'password'"
                class="block w-full rounded-lg border border-gray-300 dark:border-gray-600 bg-white dark:bg-gray-700 text-gray-900 dark:text-white placeholder-gray-500 dark:placeholder-gray-400 focus:border-blue-600 focus:ring-1 focus:ring-blue-600 py-3 px-3.5 pr-10 text-base sm:text-sm"
                id="password"
                name="password"
                placeholder="Min. 8 characters"
                required
              />
              <button
                @click.prevent="showPassword = !showPassword"
                class="absolute inset-y-0 right-0 flex items-center pr-3 text-gray-500 hover:text-gray-900 dark:hover:text-white transition-colors"
                type="button"
              >
                <span class="material-symbols-outlined" style="font-size: 20px;">
                  {{ showPassword ? 'visibility_off' : 'visibility' }}
                </span>
              </button>
            </div>
          </div>

          <div class="pt-2">
            <button
              class="group relative flex w-full justify-center rounded-lg bg-blue-600 py-3 px-4 text-sm font-semibold text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-600 focus:ring-offset-2 dark:focus:ring-offset-gray-800 transition-colors shadow-sm"
              type="submit"
            >
              Create Account
            </button>
          </div>
        </form>

        <div class="px-8 pb-8">
          <p class="text-center text-sm text-gray-600 dark:text-gray-400">
            Already have an account?
            <router-link to="/login" class="font-semibold text-blue-600 hover:text-blue-500 transition-colors">Log in</router-link>
          </p>
        </div>
      </div>
    </main>

    <!-- Footer -->
    <footer class="py-6 text-center text-sm text-gray-600 dark:text-gray-400">
      <p>© 2026 ToDo App. All rights reserved.</p>
    </footer>
  </div>
</template>

<style scoped>
.font-display {
  font-family: 'Inter', sans-serif;
}
</style>