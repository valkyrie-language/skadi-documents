import {createRouter, createWebHistory} from 'vue-router'
import HomePage from '../view/HomePage.vue'
import OrganizationPage from '../view/OrganizationPage.vue'
import PackagePage from "../view/PackagePage.vue";
import DocumentPage from '../view/DocumentPage.vue';

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path: '/',
            name: 'HomePage',
            component: HomePage
        },
        {
            path: '/@:organization',
            name: 'OrganizationPage',
            component: OrganizationPage
        },
        {
            path: '/@:organization/:package',
            name: 'PackagePage',
            component: PackagePage
        },
        {
            path: '/@:organization/:package/:version',
            name: 'DocumentMain',
            component: DocumentPage
        },
        {
            path: '/@:organization/:package/:version/:module_path',
            name: 'DocumentPage',
            component: DocumentPage
        },
    ]
})

export default router