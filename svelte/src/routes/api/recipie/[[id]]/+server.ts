import { _GET, _DELETE, _POST  } from "../../queries";

import type { RequestHandler } from '../$types';
import type { Params } from '../RecipieTypes';

export const GET = (async ({params}) => {
    try {

        let target = 'recipie';
        if((params as Params).id) {
            target = (params as Params).id;
        }

        const recipie = await _GET(target);
        return new Response(recipie);
        
    } catch(e) {

        return new Response(JSON.stringify({error: (e as any).message}));

    }
}) satisfies RequestHandler

export const POST = (async ({request}) => {
    try {
        const body = await request.json();
        const recipie = await _POST('recipie', body);
        return new Response(recipie);

    } catch(e) {

        return new Response(JSON.stringify({error: (e as any).message}));

    }
}) satisfies RequestHandler

export const DELETE = (async ({params}) => {
    try {

        const id = (params as Params).id;
        if(id === 'undefined' || !id) throw new Error('ID must be provided');

        const recipie = await _DELETE(id);
        return new Response(recipie);

    } catch(e) {

        return new Response(JSON.stringify({error: (e as any).message}));

    }
}) satisfies RequestHandler