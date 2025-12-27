import type { BaseApiOptions } from "./apiOptions";
import { BaseApi } from "./baseApi";

export class FlashcardApi extends BaseApi {
  private readonly baseUrl = "/flashcards";
  constructor(options: BaseApiOptions) {
    super(options);
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    page_size: number = 10,
    type_name?: string | null
  ) {
    const params = new URLSearchParams({
      page: page.toString(),
      page_size: page_size.toString()
    });

    // Only add type_name param if it's provided and not empty
    if (type_name && type_name.trim() !== '') {
      params.append('type_name', type_name);
    }

    const response = await this.get(
      fetch,
      "/flashcards",
      params,
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch flashcards");
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
      throw new Error("Failed to fetch flashcard");
    }
    return await response.json();
  }

  async create(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    data: FormData
  ) {
    const response = await this.post(fetch, this.baseUrl, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to create flashcard");
    }
    return await response.json();
  }

  async update(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number,
    data: FormData
  ) {
    const response = await this.patch(fetch, `${this.baseUrl}/${id}`, data, { observe: true });
    if (response.status !== 200) {
      const error = await response.json();
      if (error && error.error) {
        throw error;
      }
      throw new Error("Failed to update flashcard");
    }
    return await response.json();
  }

  async deleteById(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ) {
    const response = await this.delete(fetch, `${this.baseUrl}/${id}`, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to update flashcard");
    }
    return await response.json();
  }
}
