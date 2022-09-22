import { createResource } from 'solid-js';

import { random, wait } from '../utils/data';

const beats = [
  {
    link: 'https://www.youtube.com/watch?v=aMyO6GNkfpo',
    name: 'keshi - beside you',
  },
  {
    link: 'https://www.youtube.com/watch?v=mtoeTzYKyaQ',
    name: 'keshi - less of you',
  },
  {
    link: 'https://www.youtube.com/watch?v=eAs4Z_9I5LA',
    name: 'yihuik - estrangement',
  },
  {
    link: 'https://www.youtube.com/watch?v=a7fzkqLozwA',
    name: 'lauv - i like me better',
  },
  {
    link: 'https://www.youtube.com/watch?v=6a3Pwb_c6fI&list=RDGMEMQ1dJ7wXfLlqCjwV0xfSNbA&start_radio=1&rv=a7fzkqLozwA',
    name: 'priscilla abby - 夜空中最亮的星',
  },
  {
    link: 'https://www.youtube.com/watch?v=3MeCh9OapjE',
    name: 'My菜 - 菲道尔',
  },
  {
    link: 'https://www.youtube.com/watch?v=dos9LST6kbU',
    name: '菲道尔 Firdhaus【月亮失約了Absent Moon】',
  },
];

function fetchBeats(): Promise<typeof beats> {
  return wait(random(500, 1000), beats);
}

export default function HomeData() {
  const [data] = createResource(fetchBeats);
  return data;
}
