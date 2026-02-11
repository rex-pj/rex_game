import type { AdminGameSession } from "$lib/models/admin-game-session";
import { BaseApi } from "./baseApi";

export class GameSessionAdminApi extends BaseApi {
  private baseUrl = "/admin/game-sessions";

  async getList(fetch: Function, page: number, page_size: number) {
    const response: Response = await this.get(
      fetch,
      this.baseUrl,
      new URLSearchParams({ page: page.toString(), page_size: page_size.toString() }),
      { observe: true }
    );
    if (response.status !== 200) {
      throw new Error("Failed to fetch game sessions");
    }
    return (await response.json()) as { items: AdminGameSession[]; total_count: number };
  }

  async deleteById(fetch: Function, id: number) {
    const response: Response = await this.delete(fetch, `${this.baseUrl}/${id}`, {
      observe: true,
    });
    if (response.status !== 200) {
      throw new Error("Failed to delete game session");
    }
    return (await response.json()) as number;
  }
}
