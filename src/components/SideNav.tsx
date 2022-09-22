import type { Component } from 'solid-js';
import { For } from 'solid-js';
import { twMerge } from 'tailwind-merge';

import { setBeat } from '../stores/beat';

import IconMdiMusic from '~icons/mdi/music';

type SideNavProps = {
  beats: { name: string; link: string }[];
  class?: string;
};

const SideNav: Component<SideNavProps> = (props) => {
  const handleClick = (beat: { name: string; link: string }) => {
    setBeat(beat);
  };

  return (
    <div class={twMerge('flex flex-col h-full', props.class)}>
      <h1 class="uppercase tracking-widest text-4xl mb-8">Beats</h1>
      <div class="border-r-2 border-neutral-400 pr-4 overflow-hidden whitespace-nowrap">
        <For each={props.beats}>
          {(beat, idx) => (
            <button
              class={twMerge(
                'flex items-center text-left text-neutral-400 transform hover:scale-105 hover:text-neutral-50 transition duration-200 ease-in-out',
                idx() !== 0 && 'mt-2'
              )}
              onClick={() => handleClick(beat)}
            >
              <IconMdiMusic class="w-4 h-4 mr-3" />
              {beat.name}
            </button>
          )}
        </For>
      </div>
      <div class="mt-auto">{'Credits'}</div>
    </div>
  );
};

export default SideNav;
