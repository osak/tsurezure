// Injected by DefinePlugin
declare var URL_BASE: string;

import { Model } from '../model';

export type PostsResponse = {
    posts: Model.Post[],
    next: number | null
}

export type AdminGetPostResponse = {
    post: Model.Post,
}

export type SinglePostResponse = {
    post: Model.Post
}

export async function fetchApi<T>(api: string): Promise<T> {
    const url = URL_BASE + '/api' + normalize(api);
    const response = await fetch(url);
    if (!response.ok) {
        throw `Failed to fetch ${url}: ${response.status} ${response.statusText}`;
    }
    const result = await response.json();
    return result as T;
}

export async function fetchAdminApi<T>(api: string): Promise<T> {
    const url = URL_BASE + '/admin/api' + normalize(api);
    const response = await fetch(url);
    if (!response.ok) {
        throw `Failed to fetch ${url}: ${response.status} ${response.statusText}`;
    }
    const result = await response.json();
    return result as T;
}

function normalize(api: string): string {
    let result = api;
    if (result.charAt(0) != '/') {
        result = `/${result}`;
    }

    return result;
}