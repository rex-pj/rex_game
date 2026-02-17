import type { Flashcard } from "$lib/models/flashcard";
import type { GameType } from "$lib/models/game-type";
import { BaseApi } from "./baseApi";

export class GameTypeAdminApi extends BaseApi {
  private baseUrl = "/admin/game-types";

  async getList(
    fetch: Function,
    page: number,
    page_size: number,
    name?: string | null
  ) {
    const params: Record<string, string> = {
      page: page.toString(),
      page_size: page_size.toString(),
    };
    if (name && name.trim() !== "") {
      params.name = name;
    }
    const response: Response = await this.get(
      fetch,
      this.baseUrl,
      new URLSearchParams(params),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch game types");
    }
    return (await response.json()) as { items: GameType[]; total_count: number };
  }

  async getById(fetch: Function, id: number) {
    const response: Response = await this.get(
      fetch,
      `${this.baseUrl}/${id}`,
      new URLSearchParams(),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch game type");
    }
    return (await response.json()) as GameType;
  }

  async create(fetch: Function, data: { code: string; name: string; description?: string; icon?: string }) {
    const response: Response = await this.post(fetch, this.baseUrl, data, {
      observe: true,
    });
    if (response.status !== 200) {
      const errorText = await response.text();
      throw new Error(errorText || "Failed to create game type");
    }
    return (await response.json()) as number;
  }

  async update(fetch: Function, id: number, data: { code?: string; name?: string; description?: string; icon?: string }) {
    const response: Response = await this.patch(fetch, `${this.baseUrl}/${id}`, data, {
      observe: true,
    });
    if (response.status !== 200) {
      const errorText = await response.text();
      throw new Error(errorText || "Failed to update game type");
    }
    return (await response.json()) as boolean;
  }

  async deleteById(fetch: Function, id: number) {
    const response: Response = await this.delete(fetch, `${this.baseUrl}/${id}`, {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to delete game type");
    }
    return (await response.json()) as number;
  }

  async toggleActive(fetch: Function, id: number): Promise<boolean> {
    const response: Response = await this.put(
      fetch,
      `${this.baseUrl}/${id}/toggle-active`,
      {},
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to toggle game type active status");
    }
    return (await response.json()) as boolean;
  }

  async getFlashcards(fetch: Function, gameTypeId: number): Promise<Flashcard[]> {
    const response: Response = await this.get(
      fetch,
      `${this.baseUrl}/${gameTypeId}/flashcards`,
      new URLSearchParams(),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch game type flashcards");
    }
    return (await response.json()) as Flashcard[];
  }

  async assignFlashcards(fetch: Function, gameTypeId: number, flashcardIds: number[]): Promise<boolean> {
    const response: Response = await this.post(
      fetch,
      `${this.baseUrl}/${gameTypeId}/flashcards`,
      { flashcard_ids: flashcardIds },
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to assign flashcards");
    }
    return (await response.json()) as boolean;
  }

  async removeFlashcard(fetch: Function, gameTypeId: number, flashcardId: number): Promise<boolean> {
    const response: Response = await this.delete(
      fetch,
      `${this.baseUrl}/${gameTypeId}/flashcards/${flashcardId}`,
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to remove flashcard");
    }
    return (await response.json()) as boolean;
  }
}
