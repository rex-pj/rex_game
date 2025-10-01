import type { Cookies } from "@sveltejs/kit";
import { BaseApi } from "./baseApi";
import type JsCookies from "js-cookie";

export class FlashcardTypeApi extends BaseApi {
  private readonly baseUrl = "/flashcard-types";
  constructor(cookies?: Cookies | typeof JsCookies) {
    super(cookies);
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    page_size: number = 10
  ) {
    const response = await this.get(
      fetch,
      this.baseUrl,
      new URLSearchParams({ page: page.toString(), page_size: page_size.toString() }),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch flashcard types");
    }
    return await response.json();
  }

  async getById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.get(fetch, `${this.baseUrl}/${id}`, new URLSearchParams(), {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to fetch flashcard type");
    }
    return await response.json();
  }

  async create(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: { name: string; description: string }
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to create flashcard type");
    }
    return await response.json();
  }

  async update(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: { name: string; description: string }
  ) {
    const response = await this.patch(fetch, `${this.baseUrl}/${id}`, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to update flashcard type");
    }
    return await response.json();
  }

  async deleteById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.delete(fetch, `${this.baseUrl}/${id}`, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to delete flashcard type");
    }
    return await response.json();
  }
}
