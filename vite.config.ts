import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';
import eslint from 'vite-plugin-eslint';
import solidPlugin from 'vite-plugin-solid';

import WindiCSS from 'vite-plugin-windicss';

export default defineConfig({
  plugins: [solidPlugin(), WindiCSS(), eslint(), Icons({ compiler: 'solid', autoInstall: true })],
  build: {
    target: 'esnext',
  },
});
