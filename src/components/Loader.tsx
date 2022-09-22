import type { Component } from 'solid-js';
import { twMerge } from 'tailwind-merge';

import IconMdiLoading from '~icons/mdi/loading';

type LoaderProps = {
  title?: string;
  class?: string;
  iconClass?: string;
};

const Loader: Component<LoaderProps> = (props) => {
  const inner = <IconMdiLoading class={twMerge('w-6 h-6 animate-spin', props.iconClass)} />;

  return (
    <>
      {props.title ? (
        <div class={twMerge('flex justify-center items-center space-x-3', props.class)}>
          {inner}
          <span class="text-sm font-light">{props.title}</span>
        </div>
      ) : (
        inner
      )}
    </>
  );
};

export default Loader;
