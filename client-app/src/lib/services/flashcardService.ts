import type { Cookies } from "@sveltejs/kit";
import { BaseService } from "./baseService";
import type JsCookies from "js-cookie";

export class FlashcardService extends BaseService {
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
      "/flashcards",
      new URLSearchParams({ page: page.toString(), page_size: page_size.toString() })
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch flashcards");
    }
    return await response.json();
  }
}
