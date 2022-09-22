import type { Component } from 'solid-js';
import { useRoutes } from 'solid-app-router';

import { routes } from './routes';

const App: Component = () => {
  const Route = useRoutes(routes);

  return (
    <main class="bg-dark-900 text-neutral-50 py-8 px-8 flex w-full h-[100vh] overflow-hidden">
      <Route />
    </main>
  );
};

export default App;
