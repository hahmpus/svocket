import db from "./surreal";
import { error } from '@sveltejs/kit';

//FETCH A RECORD
export async function _GET( model:string ) {
    try {
        const result = await db.select(model)
        return JSON.stringify(result)
    } catch(e) {
        throw error(400, (e as any).message)
    }   
}

//ADD A RECORD
export async function _POST( model:string, data:any ) {
    try {
        const result = await db.create(model, data)
        return JSON.stringify(result)
    } catch(e) {
        throw error(400, (e as any).message)
    }   
}

//UPDATE A RECORD
export async function _PUT(model:string, data:any) {
    try {
        //MERGES DATA
        const result = await db.merge(model, data)
        return JSON.stringify(result)
    } catch(e) {
        throw error(400, (e as any).message)
    }   
}

export async function _PATCH(model:string, data:any) {
    try {
        //OVERWRITES DATA
        const result = await db.update(model, data)
        return JSON.stringify(result)
    } catch(e) {
        throw error(400, (e as any).message)
    }   
}


//DELETE A RECORD
export async function _DELETE( model:string ) {
    try {
        const result = await db.delete(model)
        return JSON.stringify(result)
    } catch(e) {
        throw error(400, (e as any).message)
    }   
}