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
    game_type_code?: string | null
  ) {
    const params = new URLSearchParams({
      page: page.toString(),
      page_size: page_size.toString()
    });

    // Only add game_type_code param if it's provided and not empty
    if (game_type_code && game_type_code.trim() !== '') {
      params.append('game_type_code', game_type_code);
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

  async toggleActive(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    id: number
  ): Promise<boolean> {
    const response = await this.put(fetch, `${this.baseUrl}/${id}/toggle-active`, {}, { observe: true });
    if (response.status !== 200) {
      throw new Error("Failed to toggle flashcard status");
    }
    return await response.json();
  }
}
