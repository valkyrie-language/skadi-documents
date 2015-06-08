import {createRouter, createWebHistory} from 'vue-router'
import HomePage from '../view/HomePage.vue'
import LibraryPage from '../view/LibraryPage.vue'
import OrganizationPage from '../view/OrganizationPage.vue'

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomePage
        },
        {
            path: '/@:organization',
            name: 'organization',
            component: OrganizationPage
        },
        {
            path: '/@:organization/:library',
            name: 'library',
            component: LibraryPage
        },

    ]
})

export default router