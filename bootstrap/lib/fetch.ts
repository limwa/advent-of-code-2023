export function createFetchClient(sessionCookie: string) {
    const fetchWithCookies: typeof fetch = (url, options) => {
        const newHeaders = new Headers(options?.headers);
        newHeaders.append("Cookie", `session=${sessionCookie}`);

        
        return fetch(url, {
            ...options,
            headers: newHeaders,
        });
    };

    return fetchWithCookies;
}