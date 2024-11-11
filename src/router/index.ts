import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router';
import Home from '../views/Home.vue';
import DatabaseManagement from "../views/DatabaseManagement.vue";
import ChooseDatabase from "../views/ChooseDatabase.vue";
import CreateDatabase from "../views/CreateDatabase.vue";

const routes: Array<RouteRecordRaw> = [
    {
        path: '/',
        name: 'Home',
        component: Home
    },
    {
        path: '/manage_db/:db_name',
        name: 'Database Management',
        component: DatabaseManagement,
        props: true,
    },
    {
        path: '/choose_db',
        name: 'Choose Database',
        component: ChooseDatabase,
    },
    {
        path: '/create_db',
        name: 'Create Database',
        component: CreateDatabase,
    }
];

const router = createRouter({
    history: createWebHistory(import.meta.env.VITE_BASE_URL),
    routes
});

export default router;
