import type { Cookies } from "@sveltejs/kit";
import type JsCookies from "js-cookie";
import { PUBLIC_API_URL } from "$env/static/public";

export class BaseService {
  private readonly _baseUrl: string;
  private readonly _headers: HeadersInit;
  protected readonly Cookies: Cookies | typeof JsCookies | undefined;
  constructor(cookies?: Cookies | typeof JsCookies) {
    this.Cookies = cookies;
    this._baseUrl = PUBLIC_API_URL;
    this._headers = {
      "Content-Type": "application/json",
    };

    const access_token = cookies?.get("access_token");
    if (access_token) {
      this._headers["Authorization"] = `Bearer ${access_token}`;
    }
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
    const response = await fetch(`${this._baseUrl}${url}${queryString}`, {
      method: "GET",
      headers: headers,
    });
    return await response.json();
  }

  async post(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    data: object,
    options?: any
  ) {
    let headers = this._headers;
    if (options && options["headers"]) {
      headers = { ...this._headers, ...options["headers"] };
    }

    const response = await fetch(`${this._baseUrl}${url}`, {
      method: "POST",
      headers: headers,
      body: JSON.stringify(data),
    });

    if (options && options["observe"]) {
      // If the 'observe' option is set, return the response directly without parsing as JSON
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
    let headers = this._headers;
    if (options && options["headers"]) {
      headers = { ...this._headers, ...options["headers"] };
    }

    const response = await fetch(`${this._baseUrl}${url}`, {
      method: "PUT",
      headers: headers,
      body: JSON.stringify(data),
    });
    return await response.json();
  }

  async delete(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    options?: any
  ) {
    let headers = this._headers;
    if (options && options["headers"]) {
      headers = { ...this._headers, ...options["headers"] };
    }

    const response = await fetch(`${this._baseUrl}${url}`, {
      method: "DELETE",
      headers: headers,
    });
    return await response.json();
  }

  async patch(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    url: string,
    data: object,
    options?: any
  ) {
    let headers = this._headers;
    if (options && options["headers"]) {
      headers = { ...this._headers, ...options["headers"] };
    }

    const response = await fetch(`${this._baseUrl}${url}`, {
      method: "PATCH",
      headers: headers,
      body: JSON.stringify(data),
    });
    return await response.json();
  }
}
