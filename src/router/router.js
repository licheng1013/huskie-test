import {createRouter, createWebHistory} from "vue-router";
import HomeView from "../view/HomeView.vue";
import HttpView from "../view/HttpView.vue";
import SocketView from "../view/SocketView.vue";

export const tabs = ["Http", "WebSocket","Settings","Help"]
export const tabPath = ["/", "/socket","/","/socket"]

const router = createRouter({
    history: createWebHistory(),
    routes: [
        {
            path:"/",
            requestAddr:"home",
            component:HomeView,
            children:[
                {
                    path:"",
                    requestAddr:"http",
                    component:HttpView,
                },
                {
                    path:"socket",
                    requestAddr:"socket",
                    component:SocketView,
                }
            ]
        }
    ],
    strict: true,
})
export default router