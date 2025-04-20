var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (g && (g = 0, op[0] && (_ = 0)), _) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
import { createRouter, createWebHistory } from 'vue-router';
import HomeView from '../views/HomeView.vue';
import LoginView from '../views/LoginView.vue';
import DashboardView from '../views/DashboardView.vue';
import GalleryView from '../views/GalleryView.vue';
import GallerySessionView from '../views/GallerySessionView.vue';
import NotFoundView from '../views/NotFoundView.vue';
import { supabase } from '../main';
import NewGalleryView from '@/views/NewGalleryView.vue';
var router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            component: HomeView,
            meta: {
                requiresAuth: false,
            },
        },
        {
            path: '/login',
            name: 'login',
            component: LoginView,
            meta: {
                requiresAuth: false,
            },
        },
        {
            path: '/dashboard',
            name: 'dashboard',
            component: DashboardView,
            meta: {
                requiresAuth: true,
            },
        },
        {
            path: '/new_gallery',
            name: 'new_gallery',
            component: NewGalleryView,
            meta: {
                requiresAuth: true,
            },
        },
        {
            path: '/gallery/:id',
            name: 'gallery',
            component: GalleryView,
            meta: {
                requiresAuth: true,
            },
        },
        {
            path: '/session/:id',
            name: 'gallery_session',
            component: GallerySessionView,
            meta: {
                requiresAuth: true,
            },
        },
        {
            path: '/:pathMatch(.*)*',
            name: 'not_found',
            component: NotFoundView,
        },
    ],
});
// Navigation guard for auth-required routes
router.beforeEach(function (to, from, next) { return __awaiter(void 0, void 0, void 0, function () {
    var _a, user, error;
    return __generator(this, function (_b) {
        switch (_b.label) {
            case 0: return [4 /*yield*/, supabase.auth.getUser()];
            case 1:
                _a = _b.sent(), user = _a.data.user, error = _a.error;
                if (to.matched.some(function (record) { return record.meta.requiresAuth; })) {
                    if (user == null || error != null) {
                        next({ name: 'login' });
                    }
                    else {
                        next();
                    }
                }
                else if (to.matched.some(function (record) { return record.meta.requiresAuth == false; })) {
                    if (user != null) {
                        next({ name: 'dashboard' });
                    }
                    else {
                        next();
                    }
                }
                return [2 /*return*/];
        }
    });
}); });
export default router;
