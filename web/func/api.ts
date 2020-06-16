// Injected by DefinePlugin
declare var API_BASE: string;

export async function fetchApi<T>(api: string): Promise<T> {
    const url = API_BASE + normalize(api);
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