import { Suspense } from 'solid-js';
import { useRouteData } from 'solid-app-router';

import Controls from '../components/Controls';
import Loader from '../components/Loader';
import SideNav from '../components/SideNav';
import Youtube from '../components/Youtube';

import type HomeData from './home.data';

export default function Home() {
  const data = useRouteData<typeof HomeData>();

  return (
    <Suspense
      fallback={
        <Loader
          class="absolute top-1/2 left-1/2 transform -translate-x-1/2 -translate-y-1/2"
          title="Fetching beats..."
        />
      }
    >
      <SideNav beats={data()} class="z-10" />
      <Controls class="z-10" />
      <Youtube />
    </Suspense>
  );
}
