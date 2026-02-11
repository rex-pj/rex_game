import type { Cookies } from "@sveltejs/kit";
import type JsCookies from "js-cookie";
import { PUBLIC_API_URL } from "$env/static/public";
import type { BaseApiOptions } from "./apiOptions";

// Module-level: prevent concurrent refresh attempts across all BaseApi instances
const refreshPromises = new Map<string, Promise<string | null>>();

export class BaseApi {
  protected readonly _baseUrl: string;
  private readonly _headers: Headers = new Headers();
  protected readonly Cookies: Cookies | typeof JsCookies;
  private readonly _tokenKey: string;

  constructor(options: BaseApiOptions) {
    const { cookies, tokenKey } = options;
    this.Cookies = cookies;
    this._tokenKey = tokenKey;
    this._baseUrl = PUBLIC_API_URL;
    this._headers.set("Content-Type", "application/json");
    const access_token = cookies?.get(tokenKey);
    if (access_token) {
      this._headers.set("Authorization", `Bearer ${access_token}`);
    }
  }

  /**
   * Attempt to refresh the access token via the SvelteKit API proxy.
   * Only runs client-side. Uses a module-level promise to prevent concurrent refreshes.
   */
  private async tryRefreshOnClient(): Promise<string | null> {
    if (typeof window === "undefined") return null;

    // Reuse existing refresh promise for the same token type
    const existing = refreshPromises.get(this._tokenKey);
    if (existing) return existing;

    const promise = this.doClientRefresh();
    refreshPromises.set(this._tokenKey, promise);

    try {
      return await promise;
    } finally {
      refreshPromises.delete(this._tokenKey);
    }
  }

  private async doClientRefresh(): Promise<string | null> {
    const tokenType = this._tokenKey.startsWith("s_") ? "admin" : "user";

    try {
      const response = await fetch("/api/auth/refresh", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ tokenType }),
      });

      if (!response.ok) return null;

      const data = await response.json();
      const { access_token } = data;

      if (!access_token) return null;

      // Update the Authorization header for this instance
      this._headers.set("Authorization", `Bearer ${access_token}`);

      return access_token;
    } catch {
      return null;
    }
  }

  /**
   * Wrapper around fetch that handles 401 by refreshing the token and retrying once.
   */
  private async fetchWithRetry(
    fetchFn: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    init: RequestInit
  ): Promise<Response> {
    const response = await fetchFn(url, init);

    if (response.status === 401 && typeof window !== "undefined") {
      const newToken = await this.tryRefreshOnClient();
      if (newToken) {
        // Rebuild headers with new token for the retry
        const retryHeaders = new Headers(this._headers);
        // If original request was FormData, remove Content-Type
        if (init.body instanceof FormData) {
          retryHeaders.delete("Content-Type");
        }
        return await fetchFn(url, { ...init, headers: retryHeaders });
      }
    }

    return response;
  }

  async get(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    params: URLSearchParams,
    options?: any
  ) {
    let headers = this._headers;
    if (options && options["headers"]) {
      headers = { ...this._headers, ...options["headers"] };
    }

    const queryString = params && params.toString() ? `?${params.toString()}` : "";
    const response = await this.fetchWithRetry(
      fetch,
      `${this._baseUrl}${url}${queryString}`,
      {
        method: "GET",
        headers: headers,
      }
    );

    if (options && options["observe"]) {
      return response;
    }

    return await response.json();
  }

  async post(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    data: object,
    options?: any
  ) {
    const headers = new Headers(this._headers);
    if (options && options["headers"]) {
      const custom = options["headers"];
      for (const [key, value] of Object.entries(custom)) {
        headers.set(key, value as string);
      }
    }

    const config: { credentials?: RequestCredentials } = {};
    if (options && options["includeCredentials"]) {
      config.credentials = "include";
    }

    let body: string | FormData | undefined = undefined;
    if (data instanceof FormData) {
      headers.delete("Content-Type");
      body = data;
    } else {
      body = JSON.stringify(data);
    }

    const response = await this.fetchWithRetry(
      fetch,
      `${this._baseUrl}${url}`,
      {
        method: "POST",
        headers: headers,
        body: body,
        ...config,
      }
    );

    if (options && options["observe"]) {
      return response;
    }

    return await response.json();
  }

  async put(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    data: object,
    options?: any
  ) {
    const headers = new Headers(this._headers);
    if (options && options["headers"]) {
      const custom = options["headers"];
      for (const [key, value] of Object.entries(custom)) {
        headers.set(key, value as string);
      }
    }

    const config: { credentials?: RequestCredentials } = {};
    if (options && options["includeCredentials"]) {
      config.credentials = "include";
    }

    let body: string | FormData | undefined = undefined;
    if (data instanceof FormData) {
      headers.delete("Content-Type");
      body = data;
    } else {
      body = JSON.stringify(data);
    }

    const response = await this.fetchWithRetry(
      fetch,
      `${this._baseUrl}${url}`,
      {
        method: "PUT",
        headers: headers,
        body: body,
      }
    );

    if (options && options["observe"]) {
      return response;
    }
    return await response.json();
  }

  async delete(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    options?: any
  ): Promise<Response> {
    const headers = new Headers(this._headers);
    if (options && options["headers"]) {
      const custom = options["headers"];
      for (const [key, value] of Object.entries(custom)) {
        headers.set(key, value as string);
      }
    }

    const response = await this.fetchWithRetry(
      fetch,
      `${this._baseUrl}${url}`,
      {
        method: "DELETE",
        headers: headers,
      }
    );

    if (options && options["observe"]) {
      return response;
    }
    return await response.json();
  }

  async patch(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    data: object,
    options?: any
  ) {
    const headers = new Headers(this._headers);
    if (options && options["headers"]) {
      const custom = options["headers"];
      for (const [key, value] of Object.entries(custom)) {
        headers.set(key, value as string);
      }
    }

    let body: string | FormData | undefined = undefined;
    if (data instanceof FormData) {
      headers.delete("Content-Type");
      body = data;
    } else {
      body = JSON.stringify(data);
    }

    const response = await this.fetchWithRetry(
      fetch,
      `${this._baseUrl}${url}`,
      {
        method: "PATCH",
        headers: headers,
        body: body,
      }
    );

    if (options && options["observe"]) {
      return response;
    }
    return await response.json();
  }
}
