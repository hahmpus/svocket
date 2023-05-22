import type { PageLoad } from './$types';
 
export const load = (({ params }) => {
  return {count: params.count};
}) satisfies PageLoad;