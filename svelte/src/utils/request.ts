import { writable } from 'svelte/store'
import { postRequests } from './store'

declare type Method = 'GET' | 'POST' | 'PATCH' | 'DELETE'

export default function (method:Method, url: string, requestData: any, queries: any = []) {

    //initial values
    const data:any  = writable();
    const fetching  = writable(false);
    const error     = writable(false);

    //do the thing
    async function get() {
        fetching.set(true);

        let headers: any = {
            'Content-Type': 'application/json'
        };
        let queryString: string = '';
        let params: any = {
            method: method,
            Headers: headers,
            mode: 'cors',
            credentials: 'same-origin'
        };

        if(queries.length > 0) {
            queryString = '?';
            for(const query of queries) {
                queryString += query.key + '=' + query.value + '&';
            }
        }

        //adding or removing
        if(method == 'POST' || method == 'DELETE') {
            postRequests.add(url);
        }

        if(requestData && (method == 'POST' || method == 'PATCH')) {
            params.body = JSON.stringify(requestData);
        }

        try {
            const body = await fetch(url + queryString, params);
            const json = await body.json();
            data.set(json);
        } catch (e:any) {
            error.set(e);
        }
        fetching.set(false);
    }

    //run synchronously, dont wait
    get();

    return [data, fetching, error, get];
}