<template>
  <main class="flex flex-col p-5 gap-5 justify-center items-center">
    <div class="body__border"></div>
    <h1 class="text-3xl fade-header font-semibold tracking-wider">Welcome to database manage system</h1>
    <div class="flex flex-col gap-1 text-gray-500 text-center items-center">
      <p>
        This is a simple database management system that allows you to add, update, find, and delete records.
      </p>
      <p>
        Create your own database and manage it with ease.
      </p>
      <p>
        Chose what data you want to manage and start managing it.
      </p>
    </div>
    <router-link to="/choose_db">
      <button
          type="button"
      >
        Go to management page
      </button>
    </router-link>
  </main>
</template>