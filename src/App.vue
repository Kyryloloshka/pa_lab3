<template>
  <nav class="flex gap-6 py-6 px-12">
    <router-link to="/">Home</router-link>
    <router-link to="/manage_db">Manage db</router-link>
    <router-link to="/choose_db">Choose db</router-link>
    <router-link to="/create_db">Create db</router-link>
  </nav>
  <router-view />
</template>