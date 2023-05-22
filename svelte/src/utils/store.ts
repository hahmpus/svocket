import { writable, derived } from "svelte/store";


interface Posts {
    [key: string]: number;
}


//a "global" object with subscribable keys
function countPosts() {
    const store = writable<Posts>({});

    //add a post to the url
    const add = (url: string) => {
        store.update((posts:Posts) => {
            const paths = url.split('/');
            for (let i = 0; i < paths.length; i++) {
                const path = paths.slice(0, i + 1).join('/');
                if(path === 'api') continue;

                const val = posts[path] || 0;
                posts[path] = val + 1;
            }
            return posts;
        });
    };

    //get the number of posts to the url
    const to = (url: string) => {
        return derived(store, (posts:Posts) => {
            return posts[url] ? posts[url] : 0;
        });
    };

    return {
        add,
        to: (url: string) => to(url)
    }
}
export const postRequests = countPosts();