import { createStore } from 'solid-js/store';

export const [beat, setBeat] = createStore<{ link: string | null; name: string | null }>({
  link: null,
  name: null,
});
