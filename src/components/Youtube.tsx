import type { Component } from 'solid-js';

import { beat } from '../stores/beat';
import { getYouTubeID } from '../utils/youtube';

const autoloop = true;

const Youtube: Component = () => {
  return (
    <div class="absolute overflow-hidden top-0 left-0 w-[100vw] h-[100vh] youtube fill">
      {beat.link ? (
        <iframe
          class="select-none w-[110vw] h-[110vh] absolute top-1/2 left-1/2 border-none transform -translate-x-1/2 -translate-y-1/2 pointer-events-none"
          src={`https://www.youtube.com/embed/${getYouTubeID(
            beat.link
          )}?modestbranding=1&disablekb=1&iv_load_policy=3&playsinline=1&origin=${
            window.origin
          }&autoplay=1&controls=0&loop=${autoloop}&enablejsapi=1&widgetid=1`}
          title="YouTube video player"
          allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture"
          allowfullscreen={true}
        />
      ) : (
        <div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
          select a beat to start playing
        </div>
      )}
    </div>
  );
};

export default Youtube;
