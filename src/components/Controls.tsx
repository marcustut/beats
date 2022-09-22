import type { Component } from 'solid-js';
import { twMerge } from 'tailwind-merge';

import IconMdiPlay from '~icons/mdi/play';
import IconMdiRefresh from '~icons/mdi/refresh';

type ControlsProps = {
  class?: string;
};

const Controls: Component<ControlsProps> = (props) => {
  const handlePlayOrPause = () => {
    alert('Clicked Play or Pause');
  };
  const handleLoop = () => {
    alert('Clicked loop');
  };

  return (
    <div class={twMerge('w-full flex items-center justify-center mt-auto', props.class)}>
      <button
        class={'flex justify-center items-center rounded-full p-2 border-neutral-400 border-2 mr-3'}
        onClick={handlePlayOrPause}
      >
        <IconMdiPlay class="w-12 h-12" />
      </button>
      <button
        class={'flex justify-center items-center rounded-full p-2 border-neutral-400 border-2'}
        onClick={handleLoop}
      >
        <IconMdiRefresh class="w-8 h-8" />
      </button>
    </div>
  );
};

export default Controls;
