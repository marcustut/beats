import { lazy } from 'solid-js';
import type { RouteDefinition } from 'solid-app-router';

import Home from './pages/home';
import HomeData from './pages/home.data';

export const routes: RouteDefinition[] = [
  {
    path: '/',
    component: Home,
    data: HomeData,
  },
  {
    path: '**',
    component: lazy(() => import('./errors/404')),
  },
];
