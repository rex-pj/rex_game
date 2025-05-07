import type { Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";

export class FlashcardService extends BaseService {
  constructor(cookies?: Cookies) {
    super(cookies);
  }

  async getList(
    fetch: (input: RequestInfo | URL, init?: RequestInit) => Promise<Response>,
    page: number = 1,
    pageSize: number = 10
  ) {
    const response = await this.get(
      fetch,
      "/flashcards",
      new URLSearchParams({ page: page.toString(), pageSize: pageSize.toString() })
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch flashcards");
    }
    return await response.json();
  }
}
